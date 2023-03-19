mod lexer;
mod parser;
mod ast;
mod interpreter;
mod builtin_functions;
mod scope;

use lexer::{Lexer, TokenValue};
use crate::interpreter::Interpreter;

fn main()->(){
  let mut lexer = Lexer::from_text("\
    (let x (+ 5 5))
    (+ x x)
    ");
  let mut iter = lexer.into_iter();
  let ast = parser::parse(&mut iter).expect("failed ast");
  let interpreter = Interpreter::new(&ast);
  let result = interpreter.eval();
  dbg!(&result);
}