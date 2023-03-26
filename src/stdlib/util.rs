use crate::value::builtin::BuiltinFunction;
use crate::value::builtin::InternalCallback;

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
    }
}