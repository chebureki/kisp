use std::cell::RefCell;
use std::collections::HashMap;
use std::process::id;
use std::rc::{Rc, Weak};
use crate::ast::SExpression;
use crate::interpreter::EvalValue;


pub struct Scope<'ast> {
    parent: Option<Rc<Scope<'ast>>>,
    entries: RefCell<HashMap<String, Rc<EvalValue<'ast>>>>,
    vararg: Vec<Rc<EvalValue<'ast>>>
}

impl <'ast> Scope<'ast> {
    pub fn new() -> Rc<Self> {
        Rc::new(Scope{parent: None, entries: Default::default(), vararg: Default::default()})
    }

    pub fn enter(self: &Rc<Self>) -> Rc<Self> {
        self.enter_with_vararg(vec![])
    }

    pub fn vararg<'scope>(self: &'scope Rc<Self>) -> &'scope Vec<Rc<EvalValue<'ast>>> {
        &self.vararg
    }

    pub fn enter_with_vararg(self: &Rc<Self>, vararg: Vec<Rc<EvalValue<'ast>>>) -> Rc<Self> {
        Rc::new(Self{parent: Some(self.clone()), entries: Default::default(), vararg})
    }

    pub fn lookup(&self, identifier: &String) -> Option<Rc<EvalValue<'ast>>> {
        if let Some(value) = self.entries.borrow().get(identifier) {
            Some(value.clone())
        }else if let Some(parent) = &self.parent {
            parent.lookup(identifier)
        }else {
            None
        }
    }

    pub fn insert(&self, identifier: String, value: EvalValue<'ast>) -> () {
        let mut map = self.entries.borrow_mut();
        map.insert(identifier, Rc::new(value));
    }
}