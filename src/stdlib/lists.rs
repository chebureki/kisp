use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, BuiltInFunctionArgs, Callable, EvalContext, EvalError, EvalResult, EvalValue, EvalValueRef};
use crate::expect_type;
use crate::interpreter::eval_call_with_values;
use crate::list::List;
use crate::numeric::Numeric;
use crate::scope::ScopeRef;
use crate::stdlib::util::{func};


fn list_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let vals: Vec<EvalValueRef> = args.eval_all(scope)?;
    let list = List::from(vals.clone());
    Ok((EvalValue::List(list).to_ref(), EvalContext::none()))
}

fn map_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let callable = match evaluated_left.as_ref(){
        EvalValue::CallableValue(c) => Ok(c),
        _ => Err(EvalError::InvalidType(None)),
    }?;

    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = match evaluated_right.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType(None)),
    }?;

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
    Ok((EvalValue::List(list).to_ref(), EvalContext::none()))
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
        _ => Err(EvalError::InvalidType(None)),
    }?;

    let pos = match args.try_pos(0)?.evaluated(scope)?.0.as_ref(){
        EvalValue::Numeric(Numeric::Integer(i)) => Ok(*i),
        _ => Err(EvalError::InvalidType(None)),
    }?;

    Ok((wrap_opt_to_unit(list.get(pos as usize)), EvalContext::none()))
}


fn car_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    let list = match arg_value.as_ref() {
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType(None)),
    }?;
    Ok((wrap_opt_to_unit(list.head()), EvalContext::none()))
}

fn cdr_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {

    let (arg_value, _ )  = args.try_pos(0)?.evaluated(scope)?;
    let list = match arg_value.as_ref() {
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType(None)),
    }?;
    Ok((EvalValue::List(list.tail()).to_ref(), EvalContext::none()))
}


fn cons_callback(scope: &ScopeRef, _ctx: EvalContext, args:  BuiltInFunctionArgs) -> EvalResult {
    let (arg_value, _ )  = args.try_pos(1)?.evaluated(scope)?;
    let list = match arg_value.as_ref() {
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType(None)),
    }?;



    let a = expect_type!(arg_value, EvalValue::List);
    let (con_value, _ )  = args.try_pos(0)?.evaluated(scope)?;

    Ok((EvalValue::List(list.prepended(con_value)).to_ref(), EvalContext::none()))
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
