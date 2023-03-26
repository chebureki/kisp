use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::value::{EvalError, EvalValue, EvalValueRef};

const MAX_STACK_DEPTH: usize = 420;

pub type ScopeRef = Rc<Scope>;
pub struct Scope {
    pub origin: Option<EvalValueRef>, //TODO: it should really only expect function value
    depth: usize,
    pub parent: Option<ScopeRef>,
    entries: RefCell<HashMap<String, EvalValueRef>>,
    vararg: Vec<Rc<EvalValue>>
}

impl Scope {
    pub fn new() -> Rc<Self> {
        Rc::new(Scope{origin: None,depth:0,parent: None, entries: Default::default(), vararg: Default::default()})
    }

    pub fn enter(self: &Rc<Self>, origin: Option<EvalValueRef>) -> Result<Rc<Self>, EvalError> {
        self.enter_with_vararg(vec![], origin)
    }

    pub fn vararg<'scope>(self: &'scope Rc<Self>) -> &'scope Vec<Rc<EvalValue>> {
        &self.vararg
    }

    pub fn enter_with_vararg(self: &Rc<Self>, vararg: Vec<Rc<EvalValue>>, origin: Option<EvalValueRef>) -> Result<Rc<Self>,EvalError> {
        if self.depth >= MAX_STACK_DEPTH {
            Err(EvalError::StackOverflow)
        } else{
            Ok(
                Rc::new(Self{origin, depth: self.depth+1, parent: Some(self.clone()), entries: Default::default(), vararg})
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