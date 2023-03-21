use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{BuiltinFunction, EvalError, EvalValueRef, InternalCallback};
use crate::scope::ScopeRef;

pub fn func(name: &'static str, callback: InternalCallback) -> BuiltinFunction{
    BuiltinFunction{ callback, name }
}

pub fn try_pos_arg(raw_args: &'_ [SExpression], pos: usize) -> Result<&'_ SExpression, EvalError> {
    match raw_args.get(pos){
        None => Err(EvalError::MissingArgument),
        Some(v) => Ok(v)
    }
}


//TODO: make this part of an iterable
pub fn evaluated_args(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> Result<Vec<EvalValueRef>, EvalError> {
    raw_args.iter()
        .map(|exp| eval_expression(scope, exp))
        .collect::<Result<Vec<EvalValueRef>, EvalError>>()
}