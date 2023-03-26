use std::fmt;
use std::fmt::{Debug, Display, Formatter, Pointer, Write};
use std::slice::Iter;
use std::rc::Rc;
use std::task::Context;
use crate::ast::{PosExpression, SExpression};
use crate::interpreter::eval_expression;
use crate::lexer::Cursor;
use crate::numeric::Numeric;
use crate::scope::ScopeRef;
use crate::list::List;

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

pub enum BuiltInFunctionArg{
    Val(EvalValueRef),
    Exp(PosExpression),
}

pub struct BuiltInFunctionArgs{
    pub values: Vec<BuiltInFunctionArg>,
}

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


#[derive(Debug)]
pub struct TailCall{
    pub function: EvalValueRef,
    pub args: Vec<EvalValueRef>,
}

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


pub type InternalCallback = fn(&'_ ScopeRef, EvalContext, BuiltInFunctionArgs) -> EvalResult;
impl fmt::Debug for Callable{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Callable::Internal(i) => f.write_fmt(format_args!("<internal: {}>", i.name)),
            Callable::Function(func) => f.write_fmt(format_args!("<function: {}>", func.name)),
            Callable::Lambda(lambda) => f.write_str("<lambda>"),
        }
    }
}
pub type EvalResult = Result<(EvalValueRef, EvalContext),EvalError>;


//used only for tail recursion ... for now
pub struct EvalContext{
    pub possible_tail: bool
}
/*
impl Default for EvalContext{
    fn default() -> Self {
        EvalContext{possible_tail: false}
    }
}

 */

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
