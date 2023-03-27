use std::fmt;
use std::fmt::{Debug, Formatter};
use std::rc::Rc;
use crate::ast::PosExpression;
use crate::scope::ScopeRef;
use crate::value::builtin::BuiltinFunction;
use crate::value::{EvalValue, ReferenceValue};

pub struct Function{
    pub in_scope: ScopeRef,
    pub name: String,
    pub arguments: Vec<String>,
    pub body: PosExpression,
}

impl Function{
    pub fn from(in_scope: ScopeRef, name: String, arguments: Vec<String>, body: &PosExpression) -> Function {
        Function{in_scope, name, arguments, body: body.clone()}
    }
}

pub enum Callable{
    Internal(BuiltinFunction),
    Function(Function),
    Lambda(Lambda),
}

pub struct Lambda {
    pub in_scope: ScopeRef,
    pub arguments: Vec<String>,
    pub body: PosExpression,
}


#[derive(Debug)]
pub struct TailCall{
    pub function: Rc<ReferenceValue>,
    pub args: Vec<EvalValue>,
}


impl Debug for Callable{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Callable::Internal(i) => f.write_fmt(format_args!("<internal: {}>", i.name)),
            Callable::Function(func) => f.write_fmt(format_args!("<function: {}>", func.name)),
            Callable::Lambda(_lambda) => f.write_str("<lambda>"),
        }
    }
}