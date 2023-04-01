use crate::expect_copy_type;
use crate::value::{EvalContext, EvalResult, EvalValue, ReferenceValue};
use crate::scope::ScopeRef;
use crate::value::numeric::Numeric;
use crate::stdlib::util::func;
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};
use crate::value::error::{ErrorContext, EvalError};

fn function_with_reduction<T>(scope: &ScopeRef, args: BuiltInFunctionArgs, value_mapping: impl Fn(&EvalValue) -> Result<T, ErrorContext>, reduction: impl Fn(T, T) -> T) -> Result<T, ErrorContext> {
    args.eval_all(scope)?
        .iter()
        .map(value_mapping)
        //TODO: a seemingly unnecessary collect here, but it also does an early terminate on the sream
        .collect::<Result<Vec<T>, ErrorContext>>()?.into_iter()
        .reduce(reduction)
        .map_or(Err(EvalError::MissingArgument.trace(scope)),|v|Ok(v))
}


fn numeric_reduction(scope: &ScopeRef, args: BuiltInFunctionArgs, reduction: impl Fn(Numeric, Numeric) -> Numeric) -> EvalResult{
    let value_mapping =
        |value: &EvalValue| expect_copy_type!(value, EvalValue::Numeric(n) => n.clone(), scope);
    function_with_reduction(scope, args, value_mapping, reduction)
        .map(|i| (EvalValue::Numeric(i), EvalContext::none()))
}

fn addition_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a+b)
}

fn subtraction_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a-b)
}

fn multiplication_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    numeric_reduction(scope, args, |a, b| a*b)
}

fn division_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
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
