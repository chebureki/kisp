use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Octal, Write};
use std::rc::Rc;
use std::slice::Iter;
use crate::ast::SExpression;
use crate::evalvalue::{Callable, EvalError, EvalResult, EvalValue, EvalValueRef, Function};
use crate::scope::{Scope, ScopeRef};
use crate::stdlib::std_lib_functions;

fn env_scope<'ast>() -> ScopeRef<'ast> {
    let scope = Scope::new();
    scope.insert("answer_to_all".to_string(),EvalValue::IntValue(42).to_ref());
    scope.insert("true".to_string(), EvalValue::True.to_ref());
    for bi in std_lib_functions().iter() {
        scope.insert(bi.name.to_string(), EvalValue::CallableValue(Callable::Internal(bi.callback)).to_ref())
    }
    scope
}

pub fn eval<'ast>(ast: &'ast SExpression) -> EvalResult<'ast> {
    let env = env_scope::<'ast>();
    if let SExpression::Block(expressions) = ast {
        eval_block(&env, expressions)
    }else {
        panic!("received invalid ast")
    }
}

pub(crate) fn eval_expression<'ast>(scope: &ScopeRef<'ast>, expression: &'ast SExpression) -> EvalResult<'ast> {
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

pub(crate) fn eval_function<'ast>(scope: &ScopeRef<'ast>, args: &'ast [SExpression], function: &Function<'ast>) -> EvalResult<'ast> {
    let function_scope = scope.enter()?;

    for (identifier, expression) in function.arguments.iter().zip(args) {
        function_scope.insert(identifier.clone(), eval_expression(scope, expression)?);
    }
    eval_expression(&function_scope, function.body)
}

pub(crate) fn eval_callable<'ast>(scope: &ScopeRef<'ast>, callable: &Callable<'ast>, args: &'ast [SExpression]) -> EvalResult<'ast> {
    match callable {
        Callable::Internal(internal_callback) => {
            //flat scope and args are manually evaluated
            internal_callback(scope, args)
        },
        Callable::Function(function) => eval_function(scope, args, function),
    }
}

pub(crate) fn eval_list<'ast>(scope: &ScopeRef<'ast>, expressions: &'ast Vec<SExpression>) -> EvalResult<'ast> {
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

pub(crate) fn eval_block<'ast>(scope: &ScopeRef<'ast>, expressions: &'ast Vec<SExpression>) -> EvalResult<'ast> {
    let block_scope= scope.enter()?;
    eval_block_iter(&block_scope, &mut expressions.iter(), EvalValue::Unit.to_ref())
}
