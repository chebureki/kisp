use crate::ast::SExpression;
use crate::interpreter::{EvalResult, EvalValue};
use crate::scope::ScopeRef;
use crate::stdlib::BuiltinFunction;
use crate::stdlib::util::{evaluated_args, func};

fn builtin_print<'ast>(scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    let vals: Vec<String> =
        evaluated_args(scope,raw_args)?.iter()
            .map(|v|v.to_string())
            .collect();
    //.collect::<CollectedResult>()?;
    let payload = vals.join( " ");
    println!("{}", payload);
    Ok(EvalValue::Unit.to_ref())
}

pub fn std_output<'ast>() -> Vec<BuiltinFunction<'ast>> {
    vec![
        func("print", builtin_print),
    ]
}
