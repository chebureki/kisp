use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Octal, Write};
use std::rc::Rc;
use std::slice::Iter;
use std::thread::scope;
use crate::ast::SExpression;
use crate::evalvalue::{Callable, EvalError, EvalResult, EvalValue, EvalValueRef, Function};
use crate::scope::{Scope, ScopeRef};
use crate::stdlib::std_lib_functions;

fn env_scope() -> ScopeRef {
    let scope = Scope::new();
    scope.insert("answer_to_all".to_string(),EvalValue::IntValue(42).to_ref());
    scope.insert("true".to_string(), EvalValue::True.to_ref());
    for bi in std_lib_functions().iter() {
        scope.insert(bi.name.to_string(), EvalValue::CallableValue(Callable::Internal(bi.callback)).to_ref())
    }
    scope
}

pub fn eval(ast: &'_ SExpression, provided_scope: Option<ScopeRef>) -> (EvalResult, ScopeRef) {
    let env = if let Some(provided) = provided_scope{
        provided
    }else{
        env_scope()
    };
    let res = match ast {
        //don't create a new scope!
        SExpression::Block(entries) => eval_block(&env, entries, true),
        e => eval_expression(&env, e)
    };
    (res, env)
}

pub(crate) fn eval_expression(scope: &ScopeRef, expression: &'_ SExpression) -> EvalResult {
    match expression {
        SExpression::Symbol(i) => scope.lookup(i).map_or(
            Err(EvalError::UnknownSymbol(i.clone())),
            |v| Ok(v)
        ),
        SExpression::Number(i) => Ok(EvalValue::IntValue(*i).to_ref()),
        SExpression::List(expressions) => eval_list(scope, expressions),
        SExpression::Block(expressions) => eval_block(scope, expressions, false),
        _ => todo!(),
    }
}

pub(crate) fn eval_function(scope: &ScopeRef, args: &'_ [SExpression], function: &Function) -> EvalResult {
    let function_scope = scope.enter()?;

    for (identifier, expression) in function.arguments.iter().zip(args) {
        function_scope.insert(identifier.clone(), eval_expression(scope, expression)?);
    }
    eval_expression(&function_scope, &function.body)
}

pub(crate) fn eval_callable(scope: &ScopeRef, callable: &Callable, args: &'_ [SExpression]) -> EvalResult {
    match callable {
        Callable::Internal(internal_callback) => {
            //flat scope and args are manually evaluated
            internal_callback(scope, args)
        },
        Callable::Function(function) => eval_function(scope, args, function),
    }
}

pub(crate) fn eval_list(scope: &ScopeRef, expressions: &'_ Vec<SExpression>) -> EvalResult {
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

fn eval_block_iter(scope: &ScopeRef, iterator: &mut Iter<'_, SExpression>, last: EvalValueRef) -> EvalResult {
    match iterator.next() {
        None => Ok(last),
        Some(exp) =>
            eval_expression(scope, exp).and_then(|v| eval_block_iter(scope, iterator, v))
    }
}

pub(crate) fn eval_block(scope: &ScopeRef, expressions: &'_ Vec<SExpression>, flat: bool) -> EvalResult {
    let block_scope = scope.enter()?;
    eval_block_iter(if flat {scope} else {&block_scope}, &mut expressions.iter(), EvalValue::Unit.to_ref())
}
