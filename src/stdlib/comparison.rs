use crate::value::{EvalContext, EvalError, EvalResult, EvalValue};
use crate::scope::ScopeRef;
use crate::expect_type;
use crate::value::numeric::Numeric;
use crate::stdlib::util::{func};
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};

// < > = >= <= !=

//TODO: create some quick type conversion macros
fn comparison_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, operation: fn(Numeric, Numeric) -> bool) -> EvalResult {
    let head = expect_type!(args.try_pos(0)?.evaluated(scope)?.0, EvalValue::Numeric(i) => i.clone(), None)?;
    let tail = &args.values[1..];

    for v in tail {
        let r_value = expect_type!(v.evaluated(scope)?.0, EvalValue::Numeric(i) => i.clone(), None)?;
        if !operation(head.clone(), r_value){
            return Ok((EvalValue::Unit.to_rc(), EvalContext::none())); //early return, don't even evaluate the rest
        }
    }
    Ok((EvalValue::True.to_rc(), EvalContext::none()))
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
