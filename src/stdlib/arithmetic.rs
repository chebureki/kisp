use crate::ast::SExpression;
use crate::evalvalue::{BuiltInFunctionArg, BuiltInFunctionArgs, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef};
use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::stdlib::util::{func};

fn function_with_reduction<T>(scope: &ScopeRef, args: BuiltInFunctionArgs, value_mapping: fn(&EvalValue) -> Result<T, EvalError>, reduction: fn(T, T) -> T) -> Result<T, EvalError> {
    args.eval_all(scope)?
        .iter()
        .map(|r| value_mapping(r.as_ref()))
        //TODO: a seemingly unnecessary collect here, but it also does an early terminate on the sream
        .collect::<Result<Vec<T>, EvalError>>()?.into_iter()
        .reduce(reduction)
        .map_or(Err(EvalError::MissingArgument),|v|Ok(v))
}


fn integer_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, reduction: fn(i32, i32) -> i32) -> EvalResult{
    let value_mapping = |value: &EvalValue| match value {
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType)
    };

    function_with_reduction(
        scope, args, value_mapping, reduction
    )
        .map(|i| (EvalValue::IntValue(i).to_ref(), EvalContext::none()))
}

fn add_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    integer_reduction(scope, args,|a,b| a+b)
}

fn minus_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    integer_reduction(scope, args,|a,b| a-b)
}


pub fn std_arithmetic() -> Vec<BuiltinFunction> {
    vec![
        func("+", add_callback),
        func("-", minus_callback)
    ]
}
