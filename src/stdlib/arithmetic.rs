use crate::ast::SExpression;
use crate::interpreter::{EvalError, EvalResult, EvalValue, Interpreter};
use crate::scope::ScopeRef;
use crate::stdlib::BuiltinFunction;
use crate::stdlib::util::{evaluated_args, func};

fn function_with_reduction<'ast, T>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression], value_mapping: fn(&EvalValue<'ast>) -> Result<T, EvalError>, reduction: fn(T, T) -> T) -> Result<T, EvalError> {
    evaluated_args(interpreter,scope,raw_args)?
        .iter()
        .map(|r| value_mapping(r.as_ref()))
        //TODO: a seemingly unnecessary collect here, but it also does an early terminate on the sream
        .collect::<Result<Vec<T>, EvalError>>()?.into_iter()
        .reduce(reduction)
        .map_or(Err(EvalError::MissingArgument),|v|Ok(v))
}

fn integer_reduction<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression], reduction: fn(i32, i32) -> i32) -> EvalResult<'ast>{
    let value_mapping = |value: &EvalValue| match value {
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType)
    };

    function_with_reduction(
        interpreter, scope, raw_args, value_mapping, reduction
    )
        .map(|i| EvalValue::IntValue(i).to_ref())
}

fn builtin_add<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    integer_reduction(interpreter, scope, raw_args,|a,b| a+b)
}

fn builtin_minus<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    integer_reduction(interpreter, scope, raw_args, |a,b| a-b)
}

fn builtin_modulo<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    integer_reduction(interpreter, scope, raw_args, |a,b| a%b)
}

pub fn std_arithmetic<'ast>() -> Vec<BuiltinFunction<'ast>> {
    vec![
        func("+", builtin_add),
        func("-", builtin_minus),
        func("%", builtin_modulo),
    ]
}
