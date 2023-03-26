use crate::value::{EvalContext, EvalResult, EvalValue};
use crate::scope::ScopeRef;
use crate::stdlib::util::{func};
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};

fn print_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let vals = args.eval_all(scope)?;
    let string = vals.iter()
        .map(|v|v.to_string())
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", string);
    Ok((EvalValue::Unit.to_ref(), EvalContext::none()))
}

pub fn std_output() -> Vec<BuiltinFunction> {
    vec![

        func("print", print_callback)
    ]
}
