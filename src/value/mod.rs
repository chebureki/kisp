use std::fmt;
use std::fmt::{Debug, Display, Formatter};

use std::rc::Rc;

use crate::ast::{PosExpression};
use crate::interpreter::eval_expression;
use crate::lexer::Cursor;
use crate::value::numeric::Numeric;
use crate::scope::ScopeRef;
use crate::value::callable::{Callable, TailCall};
use crate::value::list::List;

pub mod list;
pub mod callable;
pub mod builtin;
pub mod numeric;


#[derive(Debug)]
pub enum EvalValue{
    //IntValue(i32), //TODO: remove
    Numeric(Numeric),
    StringValue(String),
    Unit,
    True, // anything non nil
    //False, //really just nil
    //ExpressionRef(&'_ SExpression),
    CallableValue(Callable),
    List(List),

    //TODO: does this even fit here? I don't wanna complicate the code too much though
    TailCallValue(TailCall)
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
            //EvalValue::IntValue(i) => f.write_str(i.to_string().as_str()),
            EvalValue::StringValue(s) => f.write_str(s.as_str()),
            EvalValue::Unit => f.write_str("unit"),
            EvalValue::True => f.write_str("true"),
            EvalValue::CallableValue(c) => c.fmt(f),
            EvalValue::List(list) => Display::fmt(list,f),
            EvalValue::TailCallValue(_) => f.write_str("<tail-call>"),
            EvalValue::Numeric(n) => Display::fmt(n, f),
        }
    }
}


#[derive(Debug)]
pub enum EvalError{
    Other(String),
    UnknownSymbol(String),
    CallingNonCallable,
    InvalidType(Option<Cursor>), //TODO: this should NOT be optional
    MissingArgument,
    NotImplemented,
    Reassignment,
    StackOverflow,
}


pub type EvalResult = Result<(EvalValueRef, EvalContext),EvalError>;

//used only for tail recursion ... for now
pub struct EvalContext{
    pub possible_tail: bool
}


impl EvalContext{
    //should be removed, once fully integrated
    pub fn tmp() -> EvalContext {
        dbg!("TODO: stupid context");
        EvalContext{possible_tail: false}
        //Default::default()
    }

    pub fn none() -> EvalContext{
        EvalContext{possible_tail: false}
    }
}
