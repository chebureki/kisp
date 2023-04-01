use std::rc::Rc;
use crate::scope::{Scope, ScopeRef};
use crate::value::callable::Callable;
use crate::value::ReferenceValue;

#[derive(Debug)]
pub struct StackTrace{
    pub trace: Vec<String>
}

fn name_from_callable(c: &Callable) -> String {
    c.to_string()
}

fn trace_iter(scope: &ScopeRef, acc: Vec<String>) -> StackTrace {
    let mut acc = acc;
    if let Some(o_rc) = &scope.origin {
        let ReferenceValue::CallableValue(callable) = o_rc.as_ref() else {panic!("invalid scope origin")};
        acc.push(name_from_callable(callable));
    }
    if let Some(parent) = &scope.parent{
        trace_iter(parent, acc)
    }else{
        StackTrace{trace: acc}
    }
}

impl StackTrace{
    pub fn from_scope(scope: &ScopeRef) -> StackTrace{
        trace_iter(scope, Vec::with_capacity(scope.depth))
    }
}