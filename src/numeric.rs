use std::cmp::Ordering;
use std::fmt::{Display, Formatter, Octal, Pointer};
use std::ops;

#[derive(Debug, Clone)]
pub enum Numeric{
    Integer(i32),
    Floating(f64),
    //BigInt
}
//TODO: create a nice macro, this is really repetitive
impl ops::Add for Numeric{
    type Output = Numeric;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            //identity
            (Numeric::Integer(a), Numeric::Integer(b)) => Numeric::Integer(a+b),
            (Numeric::Floating(a), Numeric::Floating(b)) => Numeric::Floating(a+b),

            //implicit casts
            (Numeric::Integer(a), Numeric::Floating(b)) => Numeric::Floating((a as f64)+b),
            (Numeric::Floating(a), Numeric::Integer(b)) => Numeric::Floating(a+(b as f64)),

        }
    }
}

//TODO: create a nice macro, this is really repetitive
impl ops::Sub for Numeric{
    type Output = Numeric;

    fn sub(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            //identity
            (Numeric::Integer(a), Numeric::Integer(b)) => Numeric::Integer(a-b),
            (Numeric::Floating(a), Numeric::Floating(b)) => Numeric::Floating(a-b),

            //implicit casts
            (Numeric::Integer(a), Numeric::Floating(b)) => Numeric::Floating((a as f64)-b),
            (Numeric::Floating(a), Numeric::Integer(b)) => Numeric::Floating(a-(b as f64)),

        }
    }
}

impl ops::Mul for Numeric{
    type Output = Numeric;

    fn mul(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            //identity
            (Numeric::Integer(a), Numeric::Integer(b)) => Numeric::Integer(a*b),
            (Numeric::Floating(a), Numeric::Floating(b)) => Numeric::Floating(a*b),

            //implicit casts
            (Numeric::Integer(a), Numeric::Floating(b)) => Numeric::Floating((a as f64)*b),
            (Numeric::Floating(a), Numeric::Integer(b)) => Numeric::Floating(a*(b as f64)),

        }
    }
}

impl ops::Div for Numeric{
    type Output = Numeric;

    fn div(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            //identity
            (Numeric::Integer(a), Numeric::Integer(b)) => Numeric::Floating((a as f64)/(b as f64)),
            (Numeric::Floating(a), Numeric::Floating(b)) => Numeric::Floating(a/b),

            //implicit casts
            (Numeric::Integer(a), Numeric::Floating(b)) => Numeric::Floating((a as f64)/b),
            (Numeric::Floating(a), Numeric::Integer(b)) => Numeric::Floating(a/(b as f64)),

        }
    }
}

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric::Integer(i) => Display::fmt(i, f),
            Numeric::Floating(i) => Display::fmt(i, f),
        }
    }
}


impl PartialEq<Self> for Numeric {
    fn eq(&self, other: &Self) -> bool {
        match(self, other) {
            (Numeric::Integer(a), Numeric::Integer(b)) if *a == *b => true,
            (Numeric::Floating(a), Numeric::Floating(b)) if *a == *b => true,
            (Numeric::Integer(a), Numeric::Floating(b)) if (*a as f64) == *b => true,
            (Numeric::Floating(a), Numeric::Integer(b)) if *a == (*b as f64) => true,
            _ => false

        }
    }
}


impl PartialOrd for Numeric{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match (self, other) {
            (Numeric::Integer(a), Numeric::Integer(b)) if *a == *b => Some(Ordering::Equal),
            (Numeric::Floating(a), Numeric::Floating(b)) if *a == *b => Some(Ordering::Equal),
            (Numeric::Integer(a), Numeric::Floating(b)) if (*a as f64) == *b => Some(Ordering::Equal),
            (Numeric::Floating(a), Numeric::Integer(b)) if *a == (*b as f64) => Some(Ordering::Equal),

            (Numeric::Integer(a), Numeric::Integer(b)) if *a > *b => Some(Ordering::Greater),
            (Numeric::Floating(a), Numeric::Floating(b)) if *a > *b => Some(Ordering::Greater),
            (Numeric::Integer(a), Numeric::Floating(b)) if (*a as f64) > *b => Some(Ordering::Greater),
            (Numeric::Floating(a), Numeric::Integer(b)) if *a > (*b as f64) => Some(Ordering::Greater),

            (Numeric::Integer(a), Numeric::Integer(b)) if *a < *b => Some(Ordering::Less),
            (Numeric::Floating(a), Numeric::Floating(b)) if *a < *b => Some(Ordering::Less),
            (Numeric::Integer(a), Numeric::Floating(b)) if (*a as f64) < *b => Some(Ordering::Less),
            (Numeric::Floating(a), Numeric::Integer(b)) if *a < (*b as f64) => Some(Ordering::Less),
            _ => panic!("uncovered comparison")
        }
    }
}