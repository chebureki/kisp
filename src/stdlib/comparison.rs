use crate::ast::SExpression;
use crate::interpreter::{eval_expression, EvalError, EvalResult, EvalValue};
use crate::scope::ScopeRef;
use crate::stdlib::BuiltinFunction;
use crate::stdlib::util::{func, try_pos_arg};

// < > = >= <= !=

//TODO: create some quick type conversion macros
fn comparison_reduction<'ast>(scope: &ScopeRef<'ast>, args: &'ast [SExpression], operation: fn(i32, i32) -> bool) -> EvalResult<'ast> {
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

fn builtin_gt<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    comparison_reduction(scope, raw_args, |h, v| h>v)
}
fn builtin_gt_eq<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    comparison_reduction( scope, raw_args, |h, v| h>=v)
}

fn builtin_lt<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    comparison_reduction(scope, raw_args, |h, v| h<v)
}

fn builtin_lt_eq<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    comparison_reduction(scope, raw_args, |h, v| h<=v)
}

fn builtin_eq<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    comparison_reduction(scope, raw_args, |h, v| h==v)
}

fn builtin_neq<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    comparison_reduction(scope, raw_args, |h, v| h!=v)
}

pub fn std_comparison<'ast>() -> Vec<BuiltinFunction<'ast>> {
    vec![
        func(">", builtin_gt),
        func(">=", builtin_gt_eq),
        func("<", builtin_lt),
        func("<=", builtin_lt_eq),
        func("=", builtin_eq),
        func("!=", builtin_neq),
    ]
}
