use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Octal, Write};
use std::rc::Rc;
use std::slice::Iter;
use crate::ast::SExpression;
use crate::builtin_functions::builtin_functions;
use crate::scope::{Scope, ScopeRef};

pub struct Interpreter<'ast> {
    ast: &'ast SExpression,
    //test: RefCell<u32>,
}


enum Callable<'ast>{
    Internal(InternalCallback<'ast>),
    Function(&'ast SExpression),
    //Expression(&'ast SExpression),
}

#[derive(Debug)]
pub enum EvalValue<'ast>{
    IntValue(i32),
    StringValue(String),
    Unit,
    ExpressionRef(&'ast SExpression),
    CallableValue(Callable<'ast>),
}

impl Display for EvalValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::IntValue(i) => f.write_str(i.to_string().as_str()),
            EvalValue::StringValue(s) => f.write_str(s.as_str()),
            EvalValue::Unit => f.write_str("unit"),
            EvalValue::ExpressionRef(_) => f.write_str("<expression>"),
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
    scope.insert("answer_to_all".to_string(),Rc::new(EvalValue::IntValue(42)));

    for bi in builtin_functions().iter() {
        scope.insert(bi.name.to_string(),
                     Rc::new(EvalValue::CallableValue(Callable::Internal(bi.callback)))
        )
    }
    scope
}

pub type InternalCallback<'ast> = fn(&Interpreter<'ast>,ScopeRef<'ast>) -> EvalResult<'ast>;
impl fmt::Debug for Callable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
pub type EvalResult<'ast> = Result<Rc<EvalValue<'ast>>,EvalError>;

impl <'ast> Interpreter<'ast>{
    pub fn new(ast: &'ast SExpression) -> Interpreter<'ast> {
        Interpreter{ast}
    }

    pub fn eval(&self) -> EvalResult<'ast> {
        let env = env_scope::<'ast>();
        if let SExpression::List(expressions) = self.ast {
            self.eval_list_block(&env, expressions)
        }else {
            panic!("received invalid ast")
        }
    }

    pub fn eval_expression(&self, scope: &ScopeRef<'ast>, expression: &'ast SExpression) -> EvalResult<'ast> {
        match expression {
            SExpression::Symbol(i) => scope.lookup(i).map_or(
                Err(EvalError::UnknownSymbol(i.clone())),
                |v| Ok(v)
            ),
            SExpression::Number(i) => Ok(Rc::new(EvalValue::IntValue(*i))),
            SExpression::List(expressions) => self.eval_list(scope, expressions),
            _ => todo!(),
        }
    }

    fn eval_callable(&self, scope: &ScopeRef<'ast>, callable: &Callable<'ast>, args: &'ast [SExpression]) -> EvalResult<'ast> {
        match callable {
            Callable::Internal(callback) => {
                let vararg = args.iter().map(|e| Rc::new(EvalValue::ExpressionRef(e))).collect();
                let local_scope = scope.enter_with_vararg(vararg);
                callback(self, local_scope)
            },
            Callable::Function(_) => todo!()
        }
    }

    fn eval_list(&self, scope: &ScopeRef<'ast>, expressions: &'ast Vec<SExpression>) -> EvalResult<'ast> {
        if expressions.is_empty(){
            return Ok(Rc::new(EvalValue::Unit)); //not sure how well this notation is, but whatever
        }
        let head_value = self.eval_expression(scope, expressions.first().unwrap())?;
        let tail = &expressions[1..];
        let callable = match head_value.as_ref() {
            EvalValue::CallableValue(c) => {Ok(c)},
            _ => Err(EvalError::CallingNonCallable)
        }?;
        self.eval_callable(scope, callable, tail)
    }

    fn eval_list_block_iter(&self, scope: &ScopeRef<'ast>, iterator: &mut Iter<'ast, SExpression>, last: Rc<EvalValue<'ast>>) -> EvalResult<'ast> {
        match iterator.next() {
            None => Ok(last),
            Some(exp) =>
                self.eval_expression(scope, exp).and_then(|v| self.eval_list_block_iter(scope, iterator, v))
        }
    }

    fn eval_list_block(&self, scope: &ScopeRef<'ast>, expressions: &'ast Vec<SExpression>) -> EvalResult<'ast> {
        self.eval_list_block_iter(scope, &mut expressions.iter(), Rc::new(EvalValue::Unit))
    }
}