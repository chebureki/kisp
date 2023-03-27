
use crate::value::{EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef};
use crate::expect_type;
use crate::interpreter::eval_call_with_values;
use crate::value::list::List;
use crate::value::numeric::Numeric;
use crate::scope::ScopeRef;
use crate::stdlib::util::{func};
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};


fn list_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let vals: Vec<EvalValueRef> = args.eval_all(scope)?;
    let list = List::from(vals.clone());
    Ok((EvalValue::List(list).to_rc(), EvalContext::none()))
}

fn map_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let callable = expect_type!(evaluated_left, EvalValue::CallableValue(c) => c, None)?;

    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = expect_type!(evaluated_right, EvalValue::List(list) => list, None)?;

    let list = list.iterator()
        .map(|mono_arg|
            eval_call_with_values(EvalContext::none(), scope, callable, vec![mono_arg], None).map(|e| e.0)
        )
        //TODO: this collect annoys me, but fine for now
        //terminate early on error
        .collect::<Result<Vec<EvalValueRef>,EvalError>>()?
        .into_iter()
        .rev()
        .collect();
    Ok((EvalValue::List(list).to_rc(), EvalContext::none()))
}

fn wrap_opt_to_unit(v: Option<EvalValueRef>) -> EvalValueRef {
    match v {
        None => EvalValue::Unit.to_rc(),
        Some(v) => v
    }
}
fn nth_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (list_value, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = expect_type!(list_value, EvalValue::List(l) => l, None)?;
    let (arg_value, _) = args.try_pos(0)?.evaluated(scope)?;
    let pos = expect_type!(arg_value, EvalValue::Numeric(Numeric::Integer(i)) => *i, None)?;
    Ok((wrap_opt_to_unit(list.get(pos as usize)), EvalContext::none()))
}


fn car_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    let list = expect_type!(arg_value, EvalValue::List(v) => v, None)?;
    Ok((wrap_opt_to_unit(list.head()), EvalContext::none()))
}

fn cdr_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    let list = expect_type!(arg_value, EvalValue::List(l) => l, None)?;
    Ok((EvalValue::List(list.tail()).to_rc(), EvalContext::none()))
}


fn cons_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(1)?.evaluated(scope)?;
    let list = expect_type!(arg_value, EvalValue::List(l) => l, None)?;
    let (con_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    Ok((EvalValue::List(list.prepended(con_value)).to_rc(), EvalContext::none()))
}




pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![
        func("list", list_callback),
        func("car", car_callback),
        func("cdr", cdr_callback),
        func("cons", cons_callback),
        func("nth", nth_callback),
        func("map", map_callback),
    ]
}
