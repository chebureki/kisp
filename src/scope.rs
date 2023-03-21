use std::cell::RefCell;
use std::collections::HashMap;
use std::process::id;
use std::rc::{Rc, Weak};
use crate::ast::SExpression;
use crate::evalvalue::{EvalError, EvalValue, EvalValueRef};

const MAX_STACK_DEPTH: usize = 420;

pub type ScopeRef = Rc<Scope>;
pub struct Scope {
    depth: usize,
    pub parent: Option<ScopeRef>,
    entries: RefCell<HashMap<String, EvalValueRef>>,
    vararg: Vec<Rc<EvalValue>>
}

impl Scope {
    pub fn new() -> Rc<Self> {
        Rc::new(Scope{depth:0,parent: None, entries: Default::default(), vararg: Default::default()})
    }

    pub fn enter(self: &Rc<Self>) -> Result<Rc<Self>, EvalError> {
        self.enter_with_vararg(vec![])
    }

    pub fn vararg<'scope>(self: &'scope Rc<Self>) -> &'scope Vec<Rc<EvalValue>> {
        &self.vararg
    }

    pub fn enter_with_vararg(self: &Rc<Self>, vararg: Vec<Rc<EvalValue>>) -> Result<Rc<Self>,EvalError> {
        if (self.depth >= MAX_STACK_DEPTH) {
            Err(EvalError::StackOverflow)
        } else{
            Ok(
                Rc::new(Self{depth: self.depth+1, parent: Some(self.clone()), entries: Default::default(), vararg})
            )
        }
    }

    pub fn lookup(&self, identifier: &String) -> Option<Rc<EvalValue>> {
        if let Some(value) = self.entries.borrow().get(identifier) {
            Some(value.clone())
        }else if let Some(parent) = &self.parent {
            parent.lookup(identifier)
        }else {
            None
        }
    }

    pub fn insert(&self, identifier: String, value: Rc<EvalValue>) -> () {
        let mut map = self.entries.borrow_mut();
        map.insert(identifier, value);
    }
}