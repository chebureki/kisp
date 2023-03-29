use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use std::ops;

#[derive(Debug, Clone)]
pub enum Numeric{
    Integer(i32),
    Floating(f64),
    //BigInt
}

impl Numeric{
    pub fn cast_int(&self) -> Self{
        match self {
            Numeric::Integer(i) => Numeric::Integer(*i),
            Numeric::Floating(i) => Numeric::Integer(*i as i32),
        }
    }

    pub fn cast_fp(&self) -> Self{
        match self {
            Numeric::Integer(i) => Numeric::Floating(*i as f64),
            Numeric::Floating(i) => Numeric::Floating(*i),
        }
    }
}

//TODO: create a nice macro, this is really repetitive
impl ops::Add for Numeric{
    type Output = Numeric;

    fn add(self, rhs: Self) -> Self::Output {
        match (self, rhs) {
            //identity
            (Numeric::Integer(a), Numeric::Integer(b)) => a
                .checked_add(b)
                .map_or_else(|| Numeric::Floating(a as f64 + b as f64), |v| Numeric::Integer(v), ),
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
            (Numeric::Integer(a), Numeric::Integer(b)) => a
                .checked_sub(b)
                .map_or_else(|| Numeric::Floating(a as f64 - b as f64), |v| Numeric::Integer(v)),
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
            (Numeric::Integer(a), Numeric::Integer(b)) => a
                .checked_mul(b)
                .map_or_else(|| Numeric::Floating(a as f64 * b as f64), |res| Numeric::Integer(res)),
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

//not sure why, but these are the magic limits
const FLOAT_SCIENTIFIC_NOTATION_MAX: f64 = 1e+16;
const FLOAT_SCIENTIFIC_NOTATION_MIN: f64 = 1e-5;

impl Display for Numeric {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Numeric::Integer(i) => Display::fmt(i, f),
            Numeric::Floating(i) =>
                if *i>= FLOAT_SCIENTIFIC_NOTATION_MAX  || *i <= FLOAT_SCIENTIFIC_NOTATION_MIN{
                    f.write_fmt(format_args!("{:e}", i))
                }else{
                    f.write_fmt(format_args!("{}", i))
                }
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