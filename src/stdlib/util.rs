use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{BuiltinFunction, EvalError, EvalResult, EvalValue, EvalValueRef, InternalCallback};
use crate::scope::ScopeRef;

pub fn func(name: &'static str, callback: InternalCallback) -> BuiltinFunction{
    BuiltinFunction{ callback, name }
}