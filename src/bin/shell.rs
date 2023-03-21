use std::io;
use std::io::BufRead;
use std::ops::Deref;
use std::rc::Rc;
use kisp::ast::SExpression;
use kisp::lexer::Lexer;
use kisp::{interpreter, parser};
use kisp::evalvalue::{EvalValue, EvalValueRef};
use kisp::parser::ParserError;
use kisp::scope::ScopeRef;

fn do_line(env: Option<ScopeRef>, line: String) -> (Result<EvalValueRef, String>, Option<ScopeRef>) {
    let mut lexer = Lexer::from_text(line.as_str());
    let mut iter = lexer.into_iter();
    let ast = match parser::parse(&mut iter) {
        Ok(ast) => ast,
        Err(e) => return (Err(format!("Parser: {:?}", e)),env),
    };
    let (result, modded_env) = interpreter::eval(&ast, env);
    (
        result.map_err(|err| format!("Eval: {:?}", err)),
        Some(modded_env)
    )

}

fn main() {
    println!("kisp shell 0.0.0.0.0.1 pre-alpha RC0 snapshot");
    let in_src = io::stdin();
    let mut env: Option<ScopeRef> = None;

    for line_res in in_src.lock().lines() {
        let line = line_res.unwrap();
        let (result, modded_env) = do_line(env, line);
        match result {
            Ok(v) => {println!("> {}", v)}
            Err(e) => {println!("Err: {}", e)}
        }
        env = modded_env;
    }
}