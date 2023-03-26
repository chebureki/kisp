use std::fmt;
use std::fmt::{Display, Formatter, Write};
use std::ops::Deref;
use std::slice::Iter;
use crate::evalvalue::{EvalValue, EvalValueRef};

#[derive(Debug, Clone)]
struct Con {
    value: EvalValueRef,
    next: Option<Box<Con>>,
}

#[derive(Debug)]
pub struct List(Option<Box<Con>>); //Box is unnecessary, makes code simpler, but is less efficient TODO!

impl List{
    pub fn from(v: Vec<EvalValueRef>) -> List {
        let head = v.into_iter()
            .rev() // start from end
            .fold(None, |next,value|
                Some(Box::new(Con{value, next}))
            );
        List(head)
    }

    pub fn get(&self, n: usize) -> Option<EvalValueRef> {
        self.iterator().nth(n).clone()
    }

    pub fn tail(&self) -> List{
        match &self.0 {
            None => List(None),
            Some(con) => List(con.next.clone())
        }
    }

    pub fn head(&self) -> Option<EvalValueRef> {
        match &self.0 {
            None => None,
            Some(con) => Some(con.value.clone()),

        }
    }

    pub fn iterator(&self ) -> ListIterator{
        ListIterator(self.0.clone())
    }

    pub fn prepended(&self, value: EvalValueRef) -> List{
        List(Some(Box::new(Con{ value, next: self.0.clone()})))
    }
}


pub struct ListIterator(Option<Box<Con>>);
impl Iterator for ListIterator{
    type Item = EvalValueRef;

    fn next(&mut self) -> Option<Self::Item> {
        match &self.0 {
            None => None,
            Some(con) => {
                let value = con.value.clone();
                self.0 = con.next.clone();
                Some(value)
            }
        }
    }
}


impl Display for List{
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let strings: Vec<String> = self.iterator().map(|v| v.to_string()).collect();
        let joined = strings.join( " ");
        f.write_fmt( format_args!("<list: {}>", joined))
    }
}