mod lexer;
mod parser;
mod ast;
mod interpreter;
mod builtin_functions;
mod scope;

use lexer::{Lexer, TokenValue};
use crate::interpreter::{EvalResult, Interpreter};
use crate::lexer::Token;

fn main()->(){

  let mut lexer = Lexer::from_text("\
    (let x 4)
    [\
      (let y 5)
      (+ x y)
    ]\
    (fn test [a b c] (+ a b x))
    (fn double [n]
      [
        (let sum (+ n n))
        sum
      ]
     )
     (double 8)
    ");
  let mut iter = lexer.into_iter();

  let ast = parser::parse(&mut iter).expect("failed ast");
  let interpreter = Interpreter::new(&ast);
  let result = interpreter.eval();

  match result {
    Ok(data) => {println!("result: {}", data);}
    Err(err) => {println!("error: {:?}", err);}
  }
}