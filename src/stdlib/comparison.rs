use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{EvalError, EvalResult, EvalValue};

use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::stdlib::util::{func, try_pos_arg};

// < > = >= <= !=

//TODO: create some quick type conversion macros
fn comparison_reduction(scope: &ScopeRef, args: &'_ [SExpression], operation: fn(i32, i32) -> bool) -> EvalResult {
    let head_value = match eval_expression(scope, try_pos_arg(args, 0)?)?.as_ref() {
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType),
    }?;
    let tail = &args[1..];
    for expression in tail {
        let evaluated = match eval_expression(scope, expression)?.as_ref() {
            EvalValue::IntValue(i) => Ok(*i),
            _ => Err(EvalError::InvalidType),
        }?;
        if !operation(head_value, evaluated){
            return Ok(EvalValue::Unit.to_ref()); //early return, don't even evaluate the rest
        }
    }
    Ok(EvalValue::True.to_ref())
}

fn builtin_gt(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    comparison_reduction(scope, raw_args, |h, v| h>v)
}
fn builtin_gt_eq(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    comparison_reduction( scope, raw_args, |h, v| h>=v)
}

fn builtin_lt(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    comparison_reduction(scope, raw_args, |h, v| h<v)
}

fn builtin_lt_eq(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    comparison_reduction(scope, raw_args, |h, v| h<=v)
}

fn builtin_eq(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    comparison_reduction(scope, raw_args, |h, v| h==v)
}

fn builtin_neq(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    comparison_reduction(scope, raw_args, |h, v| h!=v)
}

pub fn std_comparison() -> Vec<BuiltinFunction> {
    vec![
        func(">", builtin_gt),
        func(">=", builtin_gt_eq),
        func("<", builtin_lt),
        func("<=", builtin_lt_eq),
        func("=", builtin_eq),
        func("!=", builtin_neq),
    ]
}
