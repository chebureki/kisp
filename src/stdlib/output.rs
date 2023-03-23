use crate::ast::SExpression;
use crate::evalvalue::{BuiltInFunctionArgs, EvalResult, EvalValue, EvalValueRef};
use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::interpreter::eval_expression;
use crate::stdlib::util::{func};

fn print_callback(scope: &ScopeRef, args: BuiltInFunctionArgs) -> EvalResult {
    let vals = args.eval_all(scope)?;
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
