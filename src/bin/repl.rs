use std::io;
use std::io::BufRead;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use linefeed::{Interface, ReadResult};
use kisp::ast::SExpression;
use kisp::lexer::Lexer;
use kisp::{interpreter, parser};
use kisp::evalvalue::{EvalValue, EvalValueRef};
use kisp::parser::ParserError;
use kisp::scope::ScopeRef;

const HISTORY_FILE: &str = "/Users/kirill/.ksp";
fn main() -> io::Result<()>{
    let interface = Arc::new(Interface::new("REPL for Kirill's Lisp")?);
    println!("wazzup faggot");
    interface.set_prompt("kisp> ")?;
    let mut env: Option<ScopeRef> = None;

    if let Err(e) = interface.load_history(HISTORY_FILE) {
        if e.kind() == io::ErrorKind::NotFound {
            println!("History file {} doesn't exist, not loading history.", HISTORY_FILE);
        } else {
            eprintln!("Could not load history file {}: {}", HISTORY_FILE, e);
        }
    }

    while let ReadResult::Input(line) = interface.read_line()? {
        if !line.trim().is_empty() {
            interface.add_history_unique(line.clone());
            let (result, new_env) = do_line(env, line);
            match result{
                Ok(v) => {
                    println!("{}", v);
                }
                Err(e) => {
                    println!("Err: {}", e);
                }
            }
            env = new_env;
        }

    }
    interface.save_history(HISTORY_FILE)?;
    Ok(() )
}

fn do_line(env: Option<ScopeRef>, line: String) -> (Result<EvalValueRef, String>, Option<ScopeRef>) {
    let mut lexer = Lexer::from_text(line.as_str());
    let mut iter = lexer.into_iter();
    let ast = match parser::parse(&mut iter) {
        Ok(ast) => ast,
        Err(e) => return (Err(format!("Parser: {:?}", e)),env),
    };
    let (result, modded_env) = interpreter::eval(&ast, env);
    (
        result.map_err(|err| format!("Eval: {:?}", err)).map(|v|v.0),
        Some(modded_env)
    )

}