use crate::expect_ref_type;
use crate::interpreter::eval_call_with_values;
use crate::scope::ScopeRef;
use crate::stdlib::util::func;
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};
use crate::value::{EvalContext, EvalError, EvalResult, EvalValue, ReferenceValue};

fn map_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {

    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let callable = expect_ref_type!(evaluated_left, ReferenceValue::CallableValue(c) => c, None)?;

    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = expect_ref_type!(evaluated_right, ReferenceValue::List(list) => list, None)?;

    let list = list.iterator()
        .map(|mono_arg|
            eval_call_with_values(EvalContext::none(), scope, callable, vec![mono_arg], None).map(|e| e.0)
        )
        //TODO: this collect annoys me, but fine for now
        //terminate early on error
        .collect::<Result<Vec<EvalValue>,EvalError>>()?
        .into_iter()
        .rev()
        .collect();
    Ok((EvalValue::Reference(ReferenceValue::List(list).to_rc()), EvalContext::none()))
}

pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![
        func("map", map_callback),
    ]
}