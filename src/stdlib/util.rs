use crate::ast::SExpression;
use crate::interpreter::{EvalError, EvalValueRef, InternalCallback, Interpreter};
use crate::scope::ScopeRef;
use crate::stdlib::BuiltinFunction;

pub fn func<'ast>(name: &'static str, callback: InternalCallback<'ast>) -> BuiltinFunction<'ast>{
    BuiltinFunction{ callback, name }
}

pub fn try_pos_arg<'ast>(raw_args: &'ast [SExpression], pos: usize) -> Result<&'ast SExpression, EvalError> {
    match raw_args.get(pos){
        None => Err(EvalError::MissingArgument),
        Some(v) => Ok(v)
    }
}


//TODO: make this part of an iterable
pub fn evaluated_args<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> Result<Vec<EvalValueRef<'ast>>, EvalError> {
    raw_args.iter()
        .map(|exp| interpreter.eval_expression(scope, exp))
        .collect::<Result<Vec<EvalValueRef>, EvalError>>()
}