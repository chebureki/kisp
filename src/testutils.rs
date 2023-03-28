use crate::interpreter::eval;
use crate::lexer::Lexer;
use crate::parser::parse;
use crate::value::EvalResult;
#[macro_export]
macro_rules! assert_match {
     ($expression:expr, $(|)? $( $pattern:pat_param )|+ $( if $guard: expr )? $(,)?) => {
         assert!(
             match $expression {
                $( $pattern )|+ $( if $guard )? => true,
                _ => false
            }
         )
     }

}

pub fn quick_result(input: &'static str) -> EvalResult {
    let lexer = Lexer::from_text(input);
    let mut iter = lexer.into_iter();
    let ast = parse(&mut iter).unwrap();
    eval(&ast, None).0
}
