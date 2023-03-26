use crate::ast::SExpression;
use crate::evalvalue::{BuiltInFunctionArg, BuiltInFunctionArgs, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef};
use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::expect_type;
use crate::numeric::Numeric;
use crate::stdlib::util::{func};

fn function_with_reduction<T>(scope: &ScopeRef, args: BuiltInFunctionArgs, value_mapping: fn(&EvalValueRef) -> Result<T, EvalError>, reduction: fn(T, T) -> T) -> Result<T, EvalError> {
    args.eval_all(scope)?
        .iter()
        .map(value_mapping)
        //TODO: a seemingly unnecessary collect here, but it also does an early terminate on the sream
        .collect::<Result<Vec<T>, EvalError>>()?.into_iter()
        .reduce(reduction)
        .map_or(Err(EvalError::MissingArgument),|v|Ok(v))
}


fn numeric_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, reduction: fn(Numeric, Numeric) -> Numeric) -> EvalResult{
    let value_mapping =
        |value: &EvalValueRef| expect_type!(value, EvalValue::Numeric(n) => n.clone(), None);
    function_with_reduction(scope, args, value_mapping, reduction)
        .map(|i| (EvalValue::Numeric(i).to_ref(), EvalContext::none()))
}

fn addition_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a+b)
}

fn subtraction_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a-b)
}

fn multiplication_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a*b)
}

fn division_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a/b)
}



pub fn std_arithmetic() -> Vec<BuiltinFunction> {
    vec![
        func("+", addition_callback),
        func("-", subtraction_callback),
        func("*", multiplication_callback),
        func("/", division_callback),
    ]
}
