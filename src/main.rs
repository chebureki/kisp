mod lexer;
mod parser;
mod ast;
mod interpreter;
mod scope;
mod stdlib;

use lexer::{Lexer, TokenValue};
use crate::interpreter::{EvalResult, Interpreter};
use crate::lexer::Token;

fn main()->(){

  let mut lexer = Lexer::from_text("\
    (fn sum [n]
      [
        (if (<= n 0)
          0
          (+ n (sum (- n 1)))
        )
      ]
    )
    (sum 100)
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