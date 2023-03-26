use crate::lexer::Cursor;
use crate::value::numeric::Numeric;

//https://iamwilhelm.github.io/bnf-examples/lisp
/*
s_expression = atomic_symbol \
               / "(" s_expression "."s_expression ")" \
               / list

list = "(" s_expression < s_expression > ")"

atomic_symbol = letter atom_part

atom_part = empty / letter atom_part / number atom_part

letter = "a" / "b" / " ..." / "z"

number = "1" / "2" / " ..." / "9"

empty = " "
 */
#[derive(Debug, Clone)]
pub enum SExpression{
    Symbol(String),
    Number(Numeric),
    //DotExpression(Box<SExpression>,Box<SExpression>),
    List(Vec<PosExpression>),
    Block(Vec<PosExpression>),
}

#[derive(Debug, Clone)]
pub struct PosExpression{
    pub cursor: Cursor,
    pub exp: SExpression
}