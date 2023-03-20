mod lexer;
mod parser;
mod ast;
mod interpreter;
mod builtin_functions;
mod scope;

use lexer::{Lexer, TokenValue};
use crate::interpreter::{EvalResult, Interpreter};

fn main()->(){
  /*
  (let x (+ 5 5))
    (+ x x)
    (let add +)
    (print 1 (print 2 (print 3 (print 4))))
    (print)
    (print)
   */
  let mut lexer = Lexer::from_text("\

    (print (+ 2 2))
    ");
  let mut iter = lexer.into_iter();
  let ast = parser::parse(&mut iter).expect("failed ast");
  let interpreter = Interpreter::new(&ast);
  let result = interpreter.eval();

  match result {
    Ok(data) => {println!("result: {}", data);}
    Err(err) => {println!("error: {:?}", err);}
  }
  //println!("result: {}", result.unwrap());
}