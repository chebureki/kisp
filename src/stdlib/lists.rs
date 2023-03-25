use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, BuiltInFunctionArgs, Callable, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef, List};
use crate::interpreter::eval_call_with_values;
use crate::numeric::Numeric;
use crate::scope::ScopeRef;
use crate::stdlib::util::{func};


fn list_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let vals: Vec<EvalValueRef> = args.eval_all(scope)?;
    let list = List::new(vals);
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

    let as_mono_args: Vec<Vec<EvalValueRef>> = list
        .iter_values()
        .map(|v| vec![v.clone()]).collect();

    let mapped_values = as_mono_args.into_iter()
        .map(|mono_arg|
            eval_call_with_values(EvalContext::none(), scope, callable, mono_arg, None).map(|e| e.0)
        ).collect::<Result<Vec<EvalValueRef>,EvalError>>()?;
    Ok((EvalValue::List(List::new(mapped_values)).to_ref(), EvalContext::none()))
}

fn wrap_opt_to_unit(v: Option<EvalValueRef>) -> EvalValueRef {
    match v {
        None => EvalValue::Unit.to_ref(),
        Some(v) => v
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

    Ok((wrap_opt_to_unit(list.get(pos as usize)), EvalContext::none()))
}


fn car_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    let list = match arg_value.as_ref() {
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;
    Ok((wrap_opt_to_unit(list.get(0usize)), EvalContext::none()))
}

fn cdr_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {

    let (arg_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    let list = match arg_value.as_ref() {
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;
    let sub_list = list.sub_list(1, list.len());
    Ok((EvalValue::List(sub_list).to_ref(), EvalContext::none()))
}

fn cons_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(1)?.evaluated(scope)?;
    let list = match arg_value.as_ref() {
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;
    let (con_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    //TODO: this is beyond retarded, just clone it directly
    let values: Vec<EvalValueRef> = list.iter_values().collect();
    let mut joined = Vec::with_capacity(values.len()+1);
    joined.push(con_value);
    joined.extend(values);
    Ok((EvalValue::List(List::new(joined)).to_ref(), EvalContext::none()))
}


pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![
        func("list", list_callback),
        func("nth", nth_callback),
        func("map", map_callback),
        func("car", car_callback),
        func("cdr", cdr_callback),
        func("cons", cons_callback),
    ]
}
