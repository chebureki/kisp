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
    Number(i32),
    DotExpression(Box<SExpression>,Box<SExpression>),
    List(Vec<SExpression>),
    Block(Vec<SExpression>),
}