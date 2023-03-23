use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{BuiltInFunctionArgs, EvalError, EvalResult, EvalValue, EvalValueRef};

use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::stdlib::util::{func};

// < > = >= <= !=

//TODO: create some quick type conversion macros
fn comparison_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, operation: fn(i32, i32) -> bool) -> EvalResult {
    let head: i32 = match args.try_pos(0)?.evaluated(scope)?.as_ref(){
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType),
    }?;

    let tail = &args.values[1..];

    for v in tail {
        let r_value = match v.evaluated(scope)?.as_ref() {
            EvalValue::IntValue(i) => Ok(*i),
            _ => Err(EvalError::InvalidType),
        }?;

        if !operation(head, r_value){
            return Ok(EvalValue::Unit.to_ref()); //early return, don't even evaluate the rest
        }
    }
    Ok(EvalValue::True.to_ref())
}

fn gt_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h>v)
}

fn gt_eq_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h>=v)
}

fn lt_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h<v)
}

fn lt_eq_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h<=v)
}

fn eq_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
    comparison_reduction(scope, args, |h, v| h==v)
}

fn neq_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
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
