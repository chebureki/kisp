use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Octal, Write};
use std::rc::Rc;
use std::slice::Iter;
use std::thread::scope;
use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, BuiltInFunctionArg, BuiltInFunctionArgs, Callable, EvalError, EvalResult, EvalValue, EvalValueRef, Function, Lambda};
use crate::scope::{Scope, ScopeRef};
use crate::stdlib::std_lib_functions;

fn env_scope() -> ScopeRef {
    let scope = Scope::new();
    scope.insert("answer_to_all".to_string(),EvalValue::IntValue(42).to_ref());
    scope.insert("true".to_string(), EvalValue::True.to_ref());
    for bi in std_lib_functions().into_iter() {
        scope.insert(bi.name.to_string(), EvalValue::CallableValue(Callable::Internal(bi)).to_ref())
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

fn populate_scope_with_args(scope: &ScopeRef, values: Vec<EvalValueRef>, arg_names: &Vec<String>) -> () {
    arg_names.iter()
        .zip(values)
        .for_each(
            |(ident, val)|
            scope.insert(ident.clone(), val)
        );
}

pub(crate) fn eval_with_args(scope: &ScopeRef, passed_in: Vec<EvalValueRef>, arg_names: &Vec<String>, expression: &SExpression) -> EvalResult {
    let new_scope = scope.enter()?;
    populate_scope_with_args(&new_scope, passed_in, arg_names);
    eval_expression(&new_scope, expression)
}


fn eval_all(scope: &ScopeRef, exps: & [SExpression]) -> Result<Vec<EvalValueRef>, EvalError> {
    exps.iter()
        .map(|exp| eval_expression(scope, exp))
        .collect()
}

pub(crate) fn eval_call_with_values(scope: &ScopeRef, callable: &Callable, args: Vec<EvalValueRef> ) -> EvalResult {
    match callable {
        Callable::Internal(BuiltinFunction{callback,..}) => callback(
            scope,
            BuiltInFunctionArgs::from(args.into_iter().map(|v| BuiltInFunctionArg::Val(v)).collect())
        ),
        Callable::Function(func) => eval_with_args(scope, args, &func.arguments, &func.body),
        Callable::Lambda(lam) => eval_with_args(scope, args, &lam.arguments, &lam.body),
    }
}



pub(crate) fn eval_callable(scope: &ScopeRef, callable: &Callable, args: &'_ [SExpression]) -> EvalResult {
    match callable {
        Callable::Internal(bi) => {
            let exp_args: Vec<BuiltInFunctionArg> = args.iter().map(|exp| BuiltInFunctionArg::Exp(exp.clone())).collect();
            (bi.callback)(scope, BuiltInFunctionArgs::from(exp_args))
        },
        Callable::Function(Function{arguments, body,..}) => eval_with_args(scope, eval_all(scope, args)?, arguments, body),
        Callable::Lambda(Lambda{arguments, body, ..}) => eval_with_args(scope, eval_all(scope, args)?, arguments, body),
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
