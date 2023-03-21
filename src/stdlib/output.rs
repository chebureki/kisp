use crate::ast::SExpression;
use crate::evalvalue::{EvalResult, EvalValue};
use crate::scope::ScopeRef;
use crate::stdlib::BuiltinFunction;
use crate::stdlib::util::{evaluated_args, func};

fn builtin_print(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    let vals: Vec<String> =
        evaluated_args(scope,raw_args)?.iter()
            .map(|v|v.to_string())
            .collect();
    //.collect::<CollectedResult>()?;
    let payload = vals.join( " ");
    println!("{}", payload);
    Ok(EvalValue::Unit.to_ref())
}

pub fn std_output() -> Vec<BuiltinFunction> {
    vec![
        func("print", builtin_print),
    ]
}
