use crate::value::{Copyable, EvalContext, EvalError, EvalResult, EvalValue, ReferenceValue};
use crate::scope::ScopeRef;
use crate::value::numeric::Numeric;
use crate::stdlib::util::{func};
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};

// < > = >= <= !=

//TODO: create some quick type conversion macros
fn comparison_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, operation: fn(Numeric, Numeric) -> bool) -> EvalResult {
    let head = match args.try_pos(0)?.evaluated(scope)?.0 {
        EvalValue::Copyable(Copyable::Numeric(n)) => Ok(n),
        _ => Err(EvalError::InvalidType(None))
    }?;
    //expect_copy_type!(args.try_pos(0)?.evaluated(scope)?.0, Numeric(n) => i.clone(), None)?;
    let tail = &args.values[1..];

    for v in tail {
        let r_value = match v.evaluated(scope)?.0 {
            EvalValue::Copyable(Copyable::Numeric(n)) => Ok(n.clone()),
            _ => Err(EvalError::InvalidType(None))
        }?;
        if !operation(head.clone(), r_value){
            return Ok((EvalValue::Copyable(Copyable::Unit), EvalContext::none())); //early return, don't even evaluate the rest
        }
    }
    Ok((EvalValue::Copyable(Copyable::True), EvalContext::none()))
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
