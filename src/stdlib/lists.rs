use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, BuiltInFunctionArgs, Callable, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef, List};
use crate::interpreter::eval_call_with_values;
use crate::numeric::Numeric;
use crate::scope::ScopeRef;
use crate::stdlib::util::{func};


fn list_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let vals: Vec<EvalValueRef> = args.eval_all(scope)?;
    let list = List(vals);
    Ok((EvalValue::List(list).to_ref(), EvalContext::none()))
}

fn map_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let callable = match evaluated_left.as_ref(){
        EvalValue::CallableValue(c) => Ok(c),
        _ => Err(EvalError::InvalidType),
    }?;

    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = match evaluated_right.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;

    let as_mono_args: Vec<Vec<EvalValueRef>> = list.0
        .iter()
        .map(|v| vec![v.clone()]).collect();

    let mapped_values = as_mono_args.into_iter()
        .map(|mono_arg|
            eval_call_with_values(EvalContext::none(), scope, callable, mono_arg, None).map(|e| e.0)
        ).collect::<Result<Vec<EvalValueRef>,EvalError>>()?;
    Ok((EvalValue::List(List(mapped_values)).to_ref(), EvalContext::none()))
}


fn list_nth(list: &List, n: usize) -> EvalValueRef {
    match list.0.get(n) {
        None => EvalValue::Unit.to_ref(),
        Some(v) => v.clone(),
    }
}


fn nth_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (list_value, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = match list_value.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;

    let pos = match args.try_pos(0)?.evaluated(scope)?.0.as_ref(){
        EvalValue::Numeric(Numeric::Integer(i)) => Ok(*i),
        _ => Err(EvalError::InvalidType),
    }?;

    Ok((list_nth(list, pos as usize), EvalContext::none()))
}

pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![
        func("list", list_callback),
        func("nth", nth_callback),
        func("map", map_callback),
    ]
}
