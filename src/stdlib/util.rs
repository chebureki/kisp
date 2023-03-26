use crate::ast::SExpression;
use crate::interpreter::eval_expression;
use crate::evalvalue::{BuiltinFunction, EvalError, EvalResult, EvalValue, EvalValueRef, InternalCallback};
use crate::scope::ScopeRef;

pub fn func(name: &'static str, callback: InternalCallback) -> BuiltinFunction{
    BuiltinFunction{ callback, name }
}

#[macro_export]
macro_rules! expect_type {
    ($value: expr, $pattern: pat_param => $to: expr, $cursor: expr) => {
        match $value.as_ref(){
            $pattern => Ok($to),
            _ => Err(EvalError::InvalidType($cursor))
        }
        /*
        let list = match arg_value.as_ref() {
            EvalValue::List(l) => Ok(l),
            _ => Err(EvalError::InvalidType(None)),
        }?
         */


    }
}