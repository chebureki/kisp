use std::cell::{Ref, RefCell};
use std::fmt;
use std::fmt::{Debug, Display, Formatter, Octal, Write};
use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;
use std::thread::scope;
use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, BuiltInFunctionArg, BuiltInFunctionArgs, Callable, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef, Function, Lambda};
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
        SExpression::Block(entries) => eval_block(EvalContext::none(), &env, entries, true),
        e => eval_expression(EvalContext::none(), &env, e)
    };
    (res, env)
}

pub(crate) fn eval_expression(ctx: EvalContext, scope: &ScopeRef, expression: &'_ SExpression) -> EvalResult {
    match expression {
        SExpression::Symbol(i) => scope.lookup(i).map_or(
            Err(EvalError::UnknownSymbol(i.clone())),
            |v| Ok((v, EvalContext::none()))
        ),
        SExpression::Number(i) => Ok((EvalValue::IntValue(*i).to_ref(), EvalContext::none())),
        SExpression::List(expressions) => eval_list(ctx, scope, expressions),
        SExpression::Block(expressions) => eval_block(ctx, scope, expressions, false),
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

fn is_tail_call(ctx: &EvalContext, origin: &Option<EvalValueRef>, inside: &Option<EvalValueRef>) -> bool {
    match (ctx, origin, inside) {
        (c,_,_) if !c.possible_tail => false,
        (c,Some(o),Some(i)) if c.possible_tail =>  Rc::ptr_eq(o,i),
        (_, _, _) => false,
    }
}


enum TailState{
    Tail,
    Done(EvalResult)
}
pub fn handle_tail_call(ctx: EvalContext, scope: &ScopeRef, passed_in: Vec<EvalValueRef>, arg_names: &Vec<String>, expression: &SExpression, origin: Option<EvalValueRef>) -> EvalResult {
    let tc = is_tail_call(&ctx, &scope.origin,&origin);
    if tc{
        println!("tail call detected");
    }
    eval_with_args(ctx, scope, passed_in, arg_names, expression, origin)
}

pub(crate) fn eval_with_args_flat(ctx: EvalContext, scope: &ScopeRef, passed_in: Vec<EvalValueRef>, arg_names: &Vec<String>, expression: &SExpression, origin: Option<EvalValueRef>) -> EvalResult {
    populate_scope_with_args(&scope, passed_in, arg_names);
    eval_expression(
        EvalContext{possible_tail: true}, //there we go, tail recursion
        &scope,
        expression
    )
}

pub(crate) fn eval_with_args(ctx: EvalContext, scope: &ScopeRef, passed_in: Vec<EvalValueRef>, arg_names: &Vec<String>, expression: &SExpression, origin: Option<EvalValueRef>) -> EvalResult {
    let func_scope = scope.enter(origin.clone())?;
    eval_with_args_flat(ctx, &func_scope, passed_in, arg_names, expression, origin)
}


fn eval_all(_ctx: EvalContext, scope: &ScopeRef, exps: & [SExpression]) -> Result<Vec<EvalValueRef>, EvalError> {
    exps.iter()
        .map(|exp| eval_expression(EvalContext::none(), scope, exp).map(|t| t.0))
        .collect()
}

pub(crate) fn eval_call_with_values(ctx: EvalContext, scope: &ScopeRef, callable: &Callable, args: Vec<EvalValueRef>, origin: Option<EvalValueRef>) -> EvalResult {
    match callable {
        Callable::Internal(BuiltinFunction{callback,..}) => callback(
            scope,
            ctx,
            BuiltInFunctionArgs::from(args.into_iter().map(|v| BuiltInFunctionArg::Val(v)).collect())
        ),
        Callable::Function(func) =>
            handle_tail_call(ctx, scope, args, &func.arguments, &func.body, origin),
        Callable::Lambda(lam) => eval_with_args(EvalContext::none(), scope, args, &lam.arguments, &lam.body, None),
    }
}



pub(crate) fn eval_callable(ctx: EvalContext, scope: &ScopeRef, callable: &Callable, args: &'_ [SExpression], origin: Option<EvalValueRef>) -> EvalResult {
    match callable {
        Callable::Internal(bi) => {
            let exp_args: Vec<BuiltInFunctionArg> = args.iter().map(|exp| BuiltInFunctionArg::Exp(exp.clone())).collect();
            (bi.callback)(scope, ctx, BuiltInFunctionArgs::from(exp_args))
        },
        Callable::Function(Function{arguments, body,..}) =>
            eval_call_with_values(ctx, scope, callable, eval_all(EvalContext::none(), scope, args)?, origin),

        Callable::Lambda(Lambda{arguments, body, ..}) =>
            eval_call_with_values(ctx, scope, callable, eval_all(EvalContext::none(), scope, args)?, origin),
    }
}

pub(crate) fn eval_list(ctx: EvalContext, scope: &ScopeRef, expressions: &'_ Vec<SExpression>) -> EvalResult {
    if expressions.is_empty(){
        return Ok((EvalValue::Unit.to_ref(), EvalContext::none())); //not sure how well this notation is, but whatever
    }
    let (head_value, _) = eval_expression(EvalContext::none(), scope, expressions.first().unwrap())?;
    let tail = &expressions[1..];
    let callable = match head_value.as_ref() {
        EvalValue::CallableValue(c) => {Ok(c)},
        _ => Err(EvalError::CallingNonCallable)
    }?;
    eval_callable(ctx, scope, callable, tail, Some(head_value.clone()))
}

fn eval_block_iter(ctx: EvalContext, scope: &ScopeRef, iterator: &mut Peekable<Iter<'_, SExpression>>, last: (EvalValueRef, EvalContext)) -> EvalResult {
    match iterator.next() {
        None => Ok(last),
        Some(exp) =>
            eval_expression(
                //last element could be a tail
                EvalContext{possible_tail: iterator.peek().is_none() && ctx.possible_tail},
                scope,
                exp
            ).and_then(
                |v|
                    eval_block_iter(EvalContext::none(), scope, iterator, v)
            )
    }
}

pub(crate) fn eval_block(ctx: EvalContext, scope: &ScopeRef, expressions: &'_ Vec<SExpression>, flat: bool) -> EvalResult {
    let block_scope = scope.enter(None)?;
    eval_block_iter(ctx, if flat {scope} else {&block_scope}, &mut expressions.iter().peekable(), (EvalValue::Unit.to_ref(), EvalContext::none()))
}
