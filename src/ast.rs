use std::fmt::{Display, Formatter};
use crate::lexer;
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

fn joined(v: &Vec<PosExpression>) -> String {
    let strings: Vec<String> = v.iter().map(|e| e.exp.to_string()).collect();
    strings.join( " ")
}

impl Display for SExpression{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            SExpression::Symbol(i) => f.write_str(i.as_str()),
            SExpression::Number(i) => f.write_fmt(format_args!("{}", i)),
            SExpression::List(l) => f.write_fmt(format_args!("{}{}{}", lexer::langchars::PARENTHESIS_OPEN, joined(l), lexer::langchars::PARENTHESIS_CLOSE)),
            SExpression::Block(l) => f.write_fmt(format_args!("{}{}{}", lexer::langchars::BRACKET_OPEN, joined(l), lexer::langchars::BRACKET_CLOSE)),
        }
    }
}