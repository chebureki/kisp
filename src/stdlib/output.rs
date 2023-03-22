use crate::ast::SExpression;
use crate::evalvalue::{EvalResult, EvalValue, EvalValueRef};
use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::interpreter::eval_expression;
use crate::stdlib::util::{func, evaluated_args};

fn print_callback(scope: &ScopeRef, args: Vec<EvalValueRef>) -> EvalResult {
    let vals = evaluated_args(scope, args)?;
    let string = vals.iter()
        .map(|v|v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", string);
    Ok(EvalValue::Unit.to_ref())
}

pub fn std_output() -> Vec<BuiltinFunction> {
    vec![
        func("print", print_callback)
    ]
}
