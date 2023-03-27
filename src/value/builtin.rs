use crate::ast::PosExpression;
use crate::interpreter::eval_expression;
use crate::scope::ScopeRef;
use crate::value::{EvalContext, EvalError, EvalResult, EvalValue, ReferenceValue};
use crate::value::EvalValue::Reference;

//wrappers for helper functions

pub struct BuiltInFunctionArg{
    pub value: EvalValue,
}

pub struct BuiltInFunctionArgs{
    pub values: Vec<BuiltInFunctionArg>,
}
pub type InternalCallback = fn(&'_ ScopeRef, EvalContext, BuiltInFunctionArgs) -> EvalResult;


impl BuiltInFunctionArg{
    pub fn evaluated(&self, scope: &ScopeRef) -> EvalResult {
        match &self.value {
            Reference(rc) => match rc.as_ref(){
                ReferenceValue::Expression(e) => eval_expression(EvalContext::none(), scope, e),
                _ => Ok((self.value.clone(), EvalContext::none()))
            },
            e => Ok((self.value.clone(), EvalContext::none())),

        }
    }

    pub fn try_expression<'c>(&'c self) -> Result<&'c PosExpression, EvalError> {
        match &self.value {
            Reference(rc) => match rc.as_ref(){
                ReferenceValue::Expression(e) => Ok(e),
                _ => Err(EvalError::InvalidType(None))
            },
            _ => Err(EvalError::InvalidType(None)),

        }
    }
}

impl BuiltInFunctionArgs{
    pub fn from(values: Vec<EvalValue>) -> BuiltInFunctionArgs{
        BuiltInFunctionArgs{
            values: values.into_iter().map(|value| BuiltInFunctionArg{value}).collect()
        }
    }

    pub fn eval_all(self, scope: &ScopeRef) -> Result<Vec<EvalValue>, EvalError> {
        self.values
            .into_iter()
            //discard ctx, cuz who cares
            .map(|a| a.evaluated(scope).map(|v| v.0))
            .collect()
    }

    pub fn try_pos<'c>(&'c self, pos: usize) -> Result<&'c BuiltInFunctionArg, EvalError> {
        match self.values.get(pos) {
            Some(v) => Ok(v),
            None => Err(EvalError::MissingArgument),
        }
    }
}

pub struct BuiltinFunction{
    pub callback: InternalCallback,
    pub name: &'static str
}
