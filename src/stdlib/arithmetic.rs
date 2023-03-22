use crate::ast::SExpression;
use crate::evalvalue::{EvalError, EvalResult, EvalValue, EvalValueRef};
use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::stdlib::util::{func, evaluated_args};

fn function_with_reduction<T>(scope: &ScopeRef, raw_args: Vec<EvalValueRef>, value_mapping: fn(&EvalValue) -> Result<T, EvalError>, reduction: fn(T, T) -> T) -> Result<T, EvalError> {
    evaluated_args(scope, raw_args)?
        .iter()
        .map(|r| value_mapping(r.as_ref()))
        //TODO: a seemingly unnecessary collect here, but it also does an early terminate on the sream
        .collect::<Result<Vec<T>, EvalError>>()?.into_iter()
        .reduce(reduction)
        .map_or(Err(EvalError::MissingArgument),|v|Ok(v))
}

fn integer_reduction(scope: &ScopeRef, raw_args: Vec<EvalValueRef>, reduction: fn(i32, i32) -> i32) -> EvalResult{
    let value_mapping = |value: &EvalValue| match value {
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType)
    };

    function_with_reduction(
        scope, raw_args, value_mapping, reduction
    )
        .map(|i| EvalValue::IntValue(i).to_ref())
}

fn add_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    integer_reduction(scope, raw_args,|a,b| a+b)
}

fn minus_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    integer_reduction(scope, raw_args,|a,b| a-b)
}

pub fn std_arithmetic() -> Vec<BuiltinFunction> {
    vec![
        func("+", add_callback),
        func("-", minus_callback)
    ]
}
