use std::fmt;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
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
    Lambda(Lambda),
}

pub struct Lambda {
    pub in_scope: ScopeRef,
    pub arguments: Vec<String>,
    pub body: SExpression,
}

pub struct BuiltinFunction{
    pub callback: InternalCallback,
    pub name: &'static str
}

#[derive(Debug)]
pub struct List(pub Vec<EvalValueRef>);

#[derive(Debug)]
pub enum EvalValue{
    IntValue(i32),
    StringValue(String),
    Unit,
    True, // anything non nil
    //False, //really just nil
    //ExpressionRef(&'_ SExpression),
    CallableValue(Callable),
    List(List),
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
            EvalValue::CallableValue(c) => c.fmt(f),
            EvalValue::List(list) => Display::fmt(list,f),
        }
    }
}

impl Display for List {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        //TODO: this seems inefficient
        let inner = self.0.iter().map(|v| v.to_string()).collect::<Vec<String>>().join(" ");
        f.write_fmt( format_args!("<list: {}>", inner))
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
            Callable::Lambda(lambda) => f.write_str("<lambda>"),
        }
    }
}
pub type EvalResult = Result<EvalValueRef,EvalError>;
