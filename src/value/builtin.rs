use crate::ast::PosExpression;
use crate::interpreter::eval_expression;
use crate::scope::ScopeRef;
use crate::value::{EvalContext, EvalError, EvalResult, EvalValueRef};

pub enum BuiltInFunctionArg{
    Val(EvalValueRef),
    Exp(PosExpression),
}

pub struct BuiltInFunctionArgs{
    pub values: Vec<BuiltInFunctionArg>,
}
pub type InternalCallback = fn(&'_ ScopeRef, EvalContext, BuiltInFunctionArgs) -> EvalResult;


impl BuiltInFunctionArg{
    pub fn evaluated(&self, scope: &ScopeRef) -> EvalResult {
        match self {
            BuiltInFunctionArg::Val(v) => Ok((v.clone(), EvalContext::none())),
            BuiltInFunctionArg::Exp(e) => eval_expression(EvalContext::none(), scope, e),
        }
    }

    pub fn try_expression<'c>(&'c self) -> Result<&'c PosExpression, EvalError> {
        match self {
            BuiltInFunctionArg::Val(_) => Err(EvalError::InvalidType(None)),
            BuiltInFunctionArg::Exp(e) => Ok(e)
        }
    }
}

impl BuiltInFunctionArgs{
    pub fn from(values: Vec<BuiltInFunctionArg>) -> BuiltInFunctionArgs{
        BuiltInFunctionArgs{values}
    }

    pub fn eval_all(self, scope: &ScopeRef) -> Result<Vec<EvalValueRef>, EvalError> {
        self.values
            .into_iter()
            .map(|a|
                match a {
                    BuiltInFunctionArg::Val(a) => Ok(a),
                    BuiltInFunctionArg::Exp(e) => eval_expression(EvalContext::none(), scope, &e).map(|v| v.0),
                }
            ).collect()
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
