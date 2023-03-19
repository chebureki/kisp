mod lexer;
mod parser;
mod ast;
mod interpreter;
mod builtin_functions;
mod scope;
//mod scope;

use lexer::{Lexer, TokenValue};
use crate::interpreter::Interpreter;
//use crate::interpreter::eval;
//use crate::parser::ParserResult;
//use crate::scope::ScopeContainer;

fn main()->(){
  let mut lexer = Lexer::from_text("\
    (print 1 (+ 1 1)) \
    (print (+ 2 2)) \
    ");
  let mut iter = lexer.into_iter();
  let ast = parser::parse(&mut iter).expect("failed ast");
  //dbg!(ast);
  let interpreter = Interpreter::new(&ast);
  let result = interpreter.eval();
  dbg!(&result);
  //let mut interpreter = Interpreter::new(&ast);
  //dbg!(interpreter.eval());
  //let mut scope_container = ScopeContainer::new();
  //let res = eval(&ast, &mut interpreter::env_scope(&mut scope_container));
  //dbg!(res);
}