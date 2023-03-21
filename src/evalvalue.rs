use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write};
use std::rc::Rc;
use crate::ast::SExpression;
use crate::scope::ScopeRef;

pub struct Function{
    pub in_scope: ScopeRef,
    pub name: String,
    pub arguments: Vec<String>,
    pub body: SExpression,
}

impl Function{
    pub fn from(in_scope: ScopeRef, name: String, arguments: Vec<String>, body: &SExpression) -> Function {
        Function{in_scope, name, arguments, body: body.clone()}
    }
}

pub enum Callable{
    Internal(BuiltinFunction),
    Function(Function),
    //Expression(&'_ SExpression),
}

pub struct BuiltinFunction{
    pub callback: InternalCallback,
    pub name: &'static str
}

#[derive(Debug)]
pub enum EvalValue{
    IntValue(i32),
    StringValue(String),
    Unit,
    True, // anything non nil
    //False, //really just nil
    //ExpressionRef(&'_ SExpression),
    CallableValue(Callable),
}

pub type EvalValueRef = Rc<EvalValue>;

impl EvalValue{
    pub fn to_ref(self) -> EvalValueRef{
        Rc::new(self)
    }
}

impl Display for EvalValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::IntValue(i) => f.write_str(i.to_string().as_str()),
            EvalValue::StringValue(s) => f.write_str(s.as_str()),
            EvalValue::Unit => f.write_str("unit"),
            EvalValue::True => f.write_str("true"),
            //EvalValue::ExpressionRef(_) => f.write_str("<expression>"),
            EvalValue::CallableValue(c) => c.fmt(f),
        }
    }
}

#[derive(Debug)]
pub enum EvalError{
    Other(String),
    UnknownSymbol(String),
    CallingNonCallable,
    InvalidType,
    MissingArgument,
    NotImplemented,
    Reassignment,
    StackOverflow,
}


pub type InternalCallback = fn(&'_ ScopeRef, &[SExpression]) -> EvalResult;
impl fmt::Debug for Callable{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Callable::Internal(i) => f.write_fmt(format_args!("<internal: {}>", i.name)),
            Callable::Function(func) => f.write_fmt(format_args!("<function: {}>", func.name)),
        }
    }
}
pub type EvalResult = Result<EvalValueRef,EvalError>;
