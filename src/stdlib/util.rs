use crate::value::builtin::BuiltinFunction;
use crate::value::builtin::InternalCallback;
use crate::value::{ReferenceValue, EvalValue};
pub fn func(name: &'static str, callback: InternalCallback) -> BuiltinFunction{
    BuiltinFunction{ callback, name }
}

#[macro_export]
macro_rules! expect_ref_type {
     ($value: expr, $pattern: pat_param => $to: expr, $cursor: expr) => {
         match &$value {
             EvalValue::Reference(r) => match r.as_ref() {
                $pattern => Ok($to),
                _ => Err(EvalError::InvalidType($cursor))
             },
             _ => Err(EvalError::InvalidType($cursor))
         }
     }
}

#[macro_export]
macro_rules! expect_copy_type {
    ($value: expr, $pattern: pat_param => $to: expr, $cursor: expr) => {
        match $value{
            $pattern => Ok($to),
            _ => Err(EvalError::InvalidType($cursor))
        }
    }
}