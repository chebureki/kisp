use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use crate::ast::SExpression;
use crate::scope::ScopeRef;

pub struct Function<'ast>{
    pub in_scope: ScopeRef<'ast>,
    pub name: String,
    pub arguments: Vec<String>,
    pub body: &'ast SExpression,
}

impl <'ast> Function<'ast> {
    pub fn from(in_scope: ScopeRef<'ast>, name: String, arguments: Vec<String>, body: &'ast SExpression) -> Function<'ast> {
        Function{in_scope, name, arguments, body}
    }
}

pub enum Callable<'ast>{
    Internal(InternalCallback<'ast>),
    Function(Function<'ast>),
    //Expression(&'ast SExpression),
}

#[derive(Debug)]
pub enum EvalValue<'ast>{
    IntValue(i32),
    StringValue(String),
    Unit,
    True, // anything non nil
    //False, //really just nil
    //ExpressionRef(&'ast SExpression),
    CallableValue(Callable<'ast>),
}

pub type EvalValueRef<'ast> = Rc<EvalValue<'ast>>;

impl <'ast> EvalValue<'ast>{
    pub fn to_ref(self) -> EvalValueRef<'ast>{
        Rc::new(self)
    }
}

impl Display for EvalValue<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::IntValue(i) => f.write_str(i.to_string().as_str()),
            EvalValue::StringValue(s) => f.write_str(s.as_str()),
            EvalValue::Unit => f.write_str("unit"),
            EvalValue::True => f.write_str("true"),
            //EvalValue::ExpressionRef(_) => f.write_str("<expression>"),
            EvalValue::CallableValue(_) => f.write_str("<callable>")
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


pub type InternalCallback<'ast> = fn(&'_ ScopeRef<'ast>, &'ast [SExpression]) -> EvalResult<'ast>;
impl fmt::Debug for Callable<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        todo!()
    }
}
pub type EvalResult<'ast> = Result<EvalValueRef<'ast>,EvalError>;
