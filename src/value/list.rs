use std::fmt;
use std::fmt::{Display, Formatter};
use std::rc::Rc;


use crate::value::{EvalValueRef};

#[derive(Debug)]
struct Con {
    value: EvalValueRef,
    next: Option<Rc<Con>>, //RC, so multiple lists can have the same values
}

#[derive(Debug)]
pub struct List(Option<Rc<Con>>);

impl List{
    pub fn from(v: Vec<EvalValueRef>) -> List {
        v.into_iter().rev().collect()
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
        List(Some(Rc::new(Con{ value, next: self.0.clone()})))
    }
}

impl FromIterator<EvalValueRef> for List{
    //TODO: due to my atrocious programming skills, it will be collected in reverse
    fn from_iter<T: IntoIterator<Item=EvalValueRef>>(iter: T) -> Self {
        let iterator = iter.into_iter();
        let head_opt = iterator
            .fold(None, |next,value|
                Some(Rc::new(Con{value, next}))
            );
        List(head_opt)
    }
}


pub struct ListIterator(Option<Rc<Con>>);
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