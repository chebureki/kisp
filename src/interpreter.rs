use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Octal, Write};
use std::rc::Rc;
use std::slice::Iter;
use crate::ast::SExpression;
use crate::scope::{Scope, ScopeRef};
use crate::stdlib::std_lib_functions;

pub struct Function<'ast>{
    in_scope: ScopeRef<'ast>,
    name: String,
    arguments: Vec<String>,
    body: &'ast SExpression,
}

impl <'ast> Function<'ast> {
    pub fn from(in_scope: ScopeRef<'ast>, name: String, arguments: Vec<String>, body: &'ast SExpression) -> Function<'ast> {
        Function{in_scope, name, arguments, body}
    }
}

pub enum Callable<'ast>{
    Internal(InternalCallback<'ast>),
    Function(Function<'ast>),
    //Expression(&'ast SExpression),
}

#[derive(Debug)]
pub enum EvalValue<'ast>{
    IntValue(i32),
    StringValue(String),
    Unit,
    True, // anything non nil
    //False, //really just nil
    //ExpressionRef(&'ast SExpression),
    CallableValue(Callable<'ast>),
}

pub type EvalValueRef<'ast> = Rc<EvalValue<'ast>>;

impl <'ast> EvalValue<'ast>{
    pub fn to_ref(self) -> EvalValueRef<'ast>{
        Rc::new(self)
    }
}

impl Display for EvalValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::IntValue(i) => f.write_str(i.to_string().as_str()),
            EvalValue::StringValue(s) => f.write_str(s.as_str()),
            EvalValue::Unit => f.write_str("unit"),
            EvalValue::True => f.write_str("true"),
            //EvalValue::ExpressionRef(_) => f.write_str("<expression>"),
            EvalValue::CallableValue(_) => f.write_str("<callable>")
        }
    }
}

#[derive(Debug)]
pub enum EvalError{
    Other(String),
    UnknownSymbol(String),
    CallingNonCallable,
    InvalidType,
    MissingArgument,
    NotImplemented,
    Reassignment,
}

fn env_scope<'ast>() -> ScopeRef<'ast> {
    let scope = Scope::new();
    scope.insert("answer_to_all".to_string(),EvalValue::IntValue(42).to_ref());
    scope.insert("true".to_string(), EvalValue::True.to_ref());
    for bi in std_lib_functions().iter() {
        scope.insert(bi.name.to_string(), EvalValue::CallableValue(Callable::Internal(bi.callback)).to_ref())
    }
    scope
}

pub type InternalCallback<'ast> = fn(&'_ ScopeRef<'ast>, &'ast [SExpression]) -> EvalResult<'ast>;
impl fmt::Debug for Callable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
pub type EvalResult<'ast> = Result<EvalValueRef<'ast>,EvalError>;

pub fn eval_root<'ast>(ast: &'ast SExpression) -> EvalResult<'ast> {
    let env = env_scope::<'ast>();
    if let SExpression::Block(expressions) = ast {
        eval_block(&env, expressions)
    }else {
        panic!("received invalid ast")
    }
}

pub fn eval_expression<'ast>(scope: &ScopeRef<'ast>, expression: &'ast SExpression) -> EvalResult<'ast> {
    match expression {
        SExpression::Symbol(i) => scope.lookup(i).map_or(
            Err(EvalError::UnknownSymbol(i.clone())),
            |v| Ok(v)
        ),
        SExpression::Number(i) => Ok(EvalValue::IntValue(*i).to_ref()),
        SExpression::List(expressions) => eval_list(scope, expressions),
        SExpression::Block(expressions) => eval_block(scope, expressions),
        _ => todo!(),
    }
}

fn eval_function<'ast>(scope: &ScopeRef<'ast>, args: &'ast [SExpression], function: &Function<'ast>) -> EvalResult<'ast> {
    let function_scope = scope.enter();

    for (identifier, expression) in function.arguments.iter().zip(args) {
        function_scope.insert(identifier.clone(), eval_expression(scope, expression)?);
    }
    eval_expression(&function_scope, function.body)
}

fn eval_callable<'ast>(scope: &ScopeRef<'ast>, callable: &Callable<'ast>, args: &'ast [SExpression]) -> EvalResult<'ast> {
    match callable {
        Callable::Internal(internal_callback) => {
            //flat scope and args are manually evaluated
            internal_callback(scope, args)
        },
        Callable::Function(function) => eval_function(scope, args, function),
    }
}

fn eval_list<'ast>(scope: &ScopeRef<'ast>, expressions: &'ast Vec<SExpression>) -> EvalResult<'ast> {
    if expressions.is_empty(){
        return Ok(EvalValue::Unit.to_ref()); //not sure how well this notation is, but whatever
    }
    let head_value = eval_expression(scope, expressions.first().unwrap())?;
    let tail = &expressions[1..];
    let callable = match head_value.as_ref() {
        EvalValue::CallableValue(c) => {Ok(c)},
        _ => Err(EvalError::CallingNonCallable)
    }?;
    eval_callable(scope, callable, tail)
}

fn eval_block_iter<'ast>(scope: &ScopeRef<'ast>, iterator: &mut Iter<'ast, SExpression>, last: EvalValueRef<'ast>) -> EvalResult<'ast> {
    match iterator.next() {
        None => Ok(last),
        Some(exp) =>
            eval_expression(scope, exp).and_then(|v| eval_block_iter(scope, iterator, v))
    }
}

fn eval_block<'ast>(scope: &ScopeRef<'ast>, expressions: &'ast Vec<SExpression>) -> EvalResult<'ast> {
    let block_scope= scope.enter();
    eval_block_iter(&block_scope, &mut expressions.iter(), EvalValue::Unit.to_ref())
}
