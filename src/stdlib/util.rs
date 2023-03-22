use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{BuiltinFunction, EvalError, EvalResult, EvalValue, EvalValueRef, InternalCallback};
use crate::scope::ScopeRef;

pub fn func(name: &'static str, callback: InternalCallback) -> BuiltinFunction{
    BuiltinFunction{ callback, name }
}

pub fn eval_arg(scope: &ScopeRef, arg: &EvalValueRef) -> EvalResult {
    match arg.as_ref() {
        EvalValue::ExpressionValue(e) => eval_expression(scope, e),
        other => Ok(arg.clone()),
    }
}

pub fn evaluated_args(scope: &ScopeRef, args: Vec<EvalValueRef>) -> Result<Vec<EvalValueRef>, EvalError> {
    args.into_iter()
        .map(|e| eval_arg(scope, &e))
        .collect()
}

pub fn try_pos_arg<'args>(raw_args: &'args Vec<EvalValueRef>, pos: usize) -> Result<&'args EvalValueRef, EvalError> {
    match raw_args.get(pos){
        None => Err(EvalError::MissingArgument),
        Some(v) => Ok(v)
    }
}

pub fn expect_expression_value<'v>(v: &'v EvalValueRef) -> Result<&'v SExpression, EvalError> {
    match v.as_ref() {
        EvalValue::ExpressionValue(e) => Ok(e),
        _ => Err(EvalError::InvalidType),
    }
}