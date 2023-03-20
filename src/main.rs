mod lexer;
mod parser;
mod ast;
mod interpreter;
mod scope;
mod stdlib;

use lexer::{Lexer, TokenValue};
use crate::interpreter::{eval_root, EvalResult};
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
  let result = eval_root(&ast);

  match result {
    Ok(data) => {println!("result: {}", data);}
    Err(err) => {println!("error: {:?}", err);}
  }
}