use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, EvalResult, EvalValue, EvalValueRef, List};
use crate::scope::ScopeRef;
use crate::stdlib::util::{evaluated_args, func};

fn builtin_list(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    let vals: Vec<EvalValueRef> = evaluated_args(scope, raw_args)?;
    let list = List(vals);
    Ok(EvalValue::List(list).to_ref())
}


pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![
        func("list", builtin_list),
    ]
}
