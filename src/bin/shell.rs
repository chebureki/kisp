use std::io;
use std::io::BufRead;
use std::ops::Deref;
use std::rc::Rc;
use kisp::ast::SExpression;
use kisp::lexer::Lexer;
use kisp::{interpreter, parser};
use kisp::evalvalue::EvalValueRef;
use kisp::scope::ScopeRef;

fn main() {
    println!("kisp shell 0.0.0.0.0.1 pre-alpha RC0 snapshot");
    let in_src = io::stdin();
    let mut env: Option<ScopeRef> = None;

    for line_res in in_src.lock().lines() {
        let line = line_res.unwrap();
        let mut lexer = Lexer::from_text(line.as_str());
        let mut iter = lexer.into_iter();
        let ast = match parser::parse(&mut iter).expect("failed ast") {
            SExpression::Block(data) => {
                let mut data = data;
                data.pop().unwrap()
            },
            x => panic!("TODO: received weird ast")
        };

        let (result, modded_env) = interpreter::eval(&ast, env);
        match result {
            Ok(data) => {
                println!("> {}", data);
            }
            Err(e) => {
                println!("ERR: {:?}", e);
            }
        }
        env = Some(modded_env);
    }
}