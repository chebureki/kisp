use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{BuiltInFunctionArgs, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef};

use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::numeric::Numeric;
use crate::stdlib::util::{func};

// < > = >= <= !=

//TODO: create some quick type conversion macros
fn comparison_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, operation: fn(Numeric, Numeric) -> bool) -> EvalResult {
    let head: Numeric = match args.try_pos(0)?.evaluated(scope)?.0.as_ref(){
        EvalValue::Numeric(i) => Ok(i.clone()),
        _ => Err(EvalError::InvalidType(None)),
    }?;

    let tail = &args.values[1..];

    for v in tail {
        let r_value = match v.evaluated(scope)?.0.as_ref() {
            EvalValue::Numeric(n) => Ok(n.clone()),
            _ => Err(EvalError::InvalidType(None)),
        }?;

        if !operation(head.clone(), r_value){
            return Ok((EvalValue::Unit.to_ref(), EvalContext::none())); //early return, don't even evaluate the rest
        }
    }
    Ok((EvalValue::True.to_ref(), EvalContext::none()))
}

fn gt_callback(scope: &ScopeRef, _ctx: EvalContext,  args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h>v)
}

fn gt_eq_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h>=v)
}

fn lt_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h<v)
}

fn lt_eq_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h<=v)
}

fn eq_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h==v)
}

fn neq_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h!=v)
}

pub fn std_comparison() -> Vec<BuiltinFunction> {
    vec![

        func(">", gt_callback),
        func(">=", gt_eq_callback),
        func("<", lt_callback),
        func("<=", lt_eq_callback),
        func("=", eq_callback),
        func("!=", neq_callback),
    ]
}
