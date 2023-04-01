use crate::value::builtin::BuiltinFunction;
use crate::value::builtin::InternalCallback;
use crate::value::{ReferenceValue, EvalValue};
pub fn func(name: &'static str, callback: InternalCallback) -> BuiltinFunction{
    BuiltinFunction{ callback, name }
}

#[macro_export]
macro_rules! expect_ref_type {
     ($value: expr, $pattern: pat_param => $to: expr, $scope: expr) => {
         match &$value {
             EvalValue::Reference(r) => match r.as_ref() {
                $pattern => Ok($to),
                _ => Err(EvalError::InvalidType.trace($scope))
             },
             _ => Err(EvalError::InvalidType.trace($scope))
         }
     }
}

#[macro_export]
macro_rules! expect_copy_type {
    ($value: expr, $pattern: pat_param => $to: expr, $scope: expr) => {
        match $value{
            $pattern => Ok($to),
            _ => Err(EvalError::InvalidType.trace($scope))
        }
    }
}