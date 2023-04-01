use std::fmt;
use std::fmt::{Debug, Display, Formatter, Write};

use std::rc::Rc;
use error::EvalError;
use crate::ast::{PosExpression, SExpression};


use crate::lexer::Cursor;
use crate::stacktrace::StackTrace;
use crate::value::numeric::Numeric;

use crate::value::callable::{Callable, TailCall};
use crate::value::error::ErrorContext;
use crate::value::list::List;

pub mod list;
pub mod callable;
pub mod builtin;
pub mod numeric;
pub mod error;

#[derive(Debug, Clone)]
pub enum EvalValue{
    //Copyable types
    Numeric(Numeric),
    Unit,
    True,

    //wrapped in RC
    Reference(Rc<ReferenceValue>)
}

#[derive(Debug)]
pub enum ReferenceValue {
    //False, //really just nil
    CallableValue(Callable),
    List(List),
    Expression(PosExpression), //used for macros and builtins

    //TODO: does this even fit here? I don't wanna complicate the code too much though
    TailCallValue(TailCall),
}

//pub type EvalValueRef = Rc<ReferenceValue>;

impl ReferenceValue {
    pub fn to_rc(self) -> Rc<ReferenceValue>{
        Rc::new(self)
    }
}

impl Display for ReferenceValue {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            ReferenceValue::CallableValue(c) => Display::fmt(c, f),
            ReferenceValue::List(list) => Display::fmt(list, f),
            ReferenceValue::TailCallValue(_) => f.write_str("<tail-call>"),
            ReferenceValue::Expression(PosExpression{exp,..}) => f.write_fmt(format_args!("'{}", exp)),
        }
    }
}


impl Display for EvalValue{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            EvalValue::Reference(r) => Display::fmt(r,f),
            EvalValue::Numeric(n) => Display::fmt(n, f),
            EvalValue::Unit => f.write_str("unit"),
            EvalValue::True => f.write_str("true"),
        }
    }
}


pub type EvalResult = Result<(EvalValue, EvalContext), ErrorContext>;

//used only for tail recursion ... for now
pub struct EvalContext{
    pub possible_tail: bool
}


impl EvalContext{
    pub fn none() -> EvalContext{
        EvalContext{possible_tail: false}
    }
}
