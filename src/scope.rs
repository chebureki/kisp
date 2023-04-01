use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use crate::value::{EvalValue, ReferenceValue};
use crate::value::error::{ErrorContext, EvalError};

const MAX_STACK_DEPTH: usize = 420;

pub type ScopeRef = Rc<Scope>;
#[derive(Debug)]
pub struct Scope {
    pub origin: Option<Rc<ReferenceValue>>, //TODO: it should really only expect function value
    pub depth: usize,
    pub parent: Option<ScopeRef>,
    entries: RefCell<HashMap<String, EvalValue>>,
    vararg: Vec<EvalValue>
}

impl Scope {
    pub fn new() -> Rc<Self> {
        Rc::new(Scope{origin: None,depth:0,parent: None, entries: Default::default(), vararg: Default::default()})
    }

    pub fn enter(self: &Rc<Self>, origin: Option<Rc<ReferenceValue>>) -> Result<Rc<Self>, ErrorContext> {
        self.enter_with_vararg(vec![], origin)
    }

    pub fn vararg<'scope>(self: &'scope Rc<Self>) -> &'scope Vec<EvalValue> {
        &self.vararg
    }

    pub fn enter_with_vararg(self: &Rc<Self>, vararg: Vec<EvalValue>, origin: Option<Rc<ReferenceValue>>) -> Result<Rc<Self>, ErrorContext> {
        if self.depth >= MAX_STACK_DEPTH {
            Err(EvalError::StackOverflow.trace(self))
        } else{
            Ok(
                Rc::new(Self{origin, depth: self.depth+1, parent: Some(self.clone()), entries: Default::default(), vararg})
            )
        }
    }

    pub fn lookup(&self, identifier: &String) -> Option<EvalValue> {
        if let Some(value) = self.entries.borrow().get(identifier) {
            Some(value.clone())
        }else if let Some(parent) = &self.parent {
            parent.lookup(identifier)
        }else {
            None
        }
    }

    pub fn clear(&self) -> (){
        let mut map = self.entries.borrow_mut();
        map.clear();
    }

    pub fn insert(&self, identifier: String, value: EvalValue) -> () {
        let mut map = self.entries.borrow_mut();
        map.insert(identifier, value);
    }
}