use std::iter::Peekable;
use std::rc::Rc;
use std::slice::Iter;
use crate::ast::{PosExpression,SExpression};
use crate::value::{Copyable, EvalContext, EvalError, EvalResult, EvalValue, ReferenceValue};


use crate::scope::{Scope, ScopeRef};
use crate::stdlib::std_lib_functions;
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArg, BuiltInFunctionArgs};
use crate::value::callable::{Callable, Function, Lambda, TailCall};

fn env_scope() -> ScopeRef {
    let scope = Scope::new();
    scope.insert("true".to_string(), EvalValue::Copyable(Copyable::True));
    for bi in std_lib_functions().into_iter() {
        //ReferenceValue::CallableValue(Callable::Internal(bi)).to_rc()
        scope.insert(bi.name.to_string(), EvalValue::Reference(ReferenceValue::CallableValue(Callable::Internal(bi)).to_rc()))
    }
    scope
}

pub fn eval(ast: &'_ PosExpression, provided_scope: Option<ScopeRef>) -> (EvalResult, ScopeRef) {
    let env = if let Some(provided) = provided_scope{
        provided
    }else{
        env_scope()
    };
    let res = match &ast.exp {
        //don't create a new scope!
        SExpression::Block(entries) => eval_block(EvalContext::none(), &env, entries, true),
        _ => eval_expression(EvalContext::none(), &env, ast)
    };
    (res, env)
}

pub(crate) fn eval_expression(ctx: EvalContext, scope: &ScopeRef, expression: &'_ PosExpression) -> EvalResult {
    match &expression.exp {
        SExpression::Symbol(i) => scope.lookup(i).map_or(
            Err(EvalError::UnknownSymbol(i.clone())),
            |v| Ok((v.clone(), EvalContext::none()))
        ),
        SExpression::Number(i) => Ok((EvalValue::Copyable(Copyable::Numeric(i.clone())), EvalContext::none())),
        SExpression::List(expressions) => eval_list(ctx, scope, expressions),
        SExpression::Block(expressions) => eval_block(ctx, scope, expressions, false),
    }
}

fn populate_scope_with_args(scope: &ScopeRef, values: Vec<EvalValue>, arg_names: &Vec<String>) -> () {
    arg_names.iter()
        .zip(values)
        .for_each(
            |(ident, val)|
            scope.insert(ident.clone(), val)
        );
}

fn is_tail_call(ctx: &EvalContext, origin: &Option<Rc<ReferenceValue>>, inside: &Option<Rc<ReferenceValue>>) -> bool {
    match (origin, inside) {
        (Some(o), Some(i) )if ctx.possible_tail =>  Rc::ptr_eq(o,i),
        (_, _) => false,
    }
}


pub fn wrap_tail_call(ctx: EvalContext, scope: &ScopeRef, passed_in: Vec<EvalValue>, arg_names: &Vec<String>, expression: &PosExpression, origin: Option<Rc<ReferenceValue>>) -> EvalResult {
    let tc_detected = is_tail_call(&ctx, &scope.origin,&origin);
    if tc_detected{
        let tc: TailCall = TailCall{ function: origin.unwrap().clone(), args: passed_in };
        Ok((EvalValue::Reference(ReferenceValue::TailCallValue(tc).to_rc()), ctx))
    }else {
        eval_with_args(ctx, scope, passed_in, arg_names, expression, origin)
    }
}

pub(crate) fn eval_with_args_flat(given_ctx: EvalContext, scope: &ScopeRef, passed_in: Vec<EvalValue>, arg_names: &Vec<String>, expression: &PosExpression, _origin: Option<Rc<ReferenceValue>>) -> EvalResult {
    populate_scope_with_args(&scope, passed_in, arg_names);
    let (mut res, mut res_ctx) = eval_expression(
        EvalContext{possible_tail: true}, //there we go, tail recursion
        &scope,
        expression
    )?;
    while let EvalValue::Reference(r)= &res {
        match r.as_ref(){
            ReferenceValue::TailCallValue(tc) => {
                populate_scope_with_args(&scope, tc.args.clone(), arg_names);
                (res, res_ctx) = eval_expression(EvalContext{possible_tail: true}, scope, expression)?;
            },
            _=> break
        }

    }
    Ok((res, EvalContext{possible_tail: given_ctx.possible_tail && res_ctx.possible_tail}))
}

pub(crate) fn eval_with_args(ctx: EvalContext, scope: &ScopeRef, passed_in: Vec<EvalValue>, arg_names: &Vec<String>, expression: &PosExpression, origin: Option<Rc<ReferenceValue>>) -> EvalResult {
    let func_scope = scope.enter(origin.clone())?;
    eval_with_args_flat(ctx, &func_scope, passed_in, arg_names, expression, origin)
}


fn eval_all(_ctx: EvalContext, scope: &ScopeRef, exps: & [PosExpression]) -> Result<Vec<EvalValue>, EvalError> {
    exps.iter()
        .map(|exp| eval_expression(EvalContext::none(), scope, exp).map(|t| t.0))
        .collect()
}

pub(crate) fn eval_call_with_values(ctx: EvalContext, scope: &ScopeRef, callable: &Callable, args: Vec<EvalValue>, origin: Option<Rc<ReferenceValue>>) -> EvalResult {
    match callable {
        Callable::Internal(BuiltinFunction{callback,..}) => callback(
            scope,
            ctx,
            BuiltInFunctionArgs::from(args),
        ),
        Callable::Function(func) =>
            wrap_tail_call(ctx, scope, args, &func.arguments, &func.body, origin),
        Callable::Lambda(lam) => eval_with_args(EvalContext::none(), scope, args, &lam.arguments, &lam.body, None),
    }
}


pub(crate) fn eval_callable(ctx: EvalContext, scope: &ScopeRef, callable: &Callable, args: &'_ [PosExpression], origin: Option<Rc<ReferenceValue>>) -> EvalResult {
    match callable {
        Callable::Internal(bi) => {
            let exp_args: Vec<EvalValue> = args.iter().map(|exp| EvalValue::Reference(ReferenceValue::Expression(exp.clone()).to_rc())).collect();
            (bi.callback)(scope, ctx, BuiltInFunctionArgs::from(exp_args))
        },
        Callable::Function(Function{arguments: _, body: _,..}) =>
            eval_call_with_values(ctx, scope, callable, eval_all(EvalContext::none(), scope, args)?, origin),

        Callable::Lambda(Lambda{arguments: _, body: _, ..}) =>
            eval_call_with_values(ctx, scope, callable, eval_all(EvalContext::none(), scope, args)?, origin),
    }
}

pub(crate) fn eval_list(ctx: EvalContext, scope: &ScopeRef, expressions: &'_ Vec<PosExpression>) -> EvalResult {

    if expressions.is_empty(){
        return Ok((EvalValue::Copyable(Copyable::Unit), EvalContext::none())); //not sure how well this notation is, but whatever
    }
    let (head_value, _) = eval_expression(EvalContext::none(), scope, expressions.first().unwrap())?;
    let tail = &expressions[1..];
    let (r, callable) = match &head_value {
        EvalValue::Reference(r) => {
            match r.as_ref(){
                ReferenceValue::CallableValue(c) => Ok((r.clone(),c)),
                _ => Err(EvalError::CallingNonCallable)
            }
        }
        _ => Err(EvalError::CallingNonCallable)
    }?;
    eval_callable(ctx, scope, callable, tail, Some(r.clone()))
}

fn eval_block_iter(ctx: EvalContext, scope: &ScopeRef, iterator: &mut Peekable<Iter<'_, PosExpression>>, last: (EvalValue, EvalContext)) -> EvalResult {
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

pub(crate) fn eval_block(ctx: EvalContext, scope: &ScopeRef, expressions: &'_ Vec<PosExpression>, flat: bool) -> EvalResult {
    let block_scope = scope.enter(None)?;
    eval_block_iter(ctx, if flat {scope} else {&block_scope}, &mut expressions.iter().peekable(), (EvalValue::Copyable(Copyable::Unit), EvalContext::none()))
}
