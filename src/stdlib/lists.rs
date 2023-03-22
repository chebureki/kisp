use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, Callable, EvalError, EvalResult, EvalValue, EvalValueRef, List};
use crate::interpreter::eval_call_with_values;
use crate::scope::ScopeRef;
use crate::stdlib::util::{func, evaluated_args, eval_arg, try_pos_arg};

fn list_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let vals: Vec<EvalValueRef> = evaluated_args(scope, raw_args)?;
    let list = List(vals);
    Ok(EvalValue::List(list).to_ref())
}

fn map_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let evaluated_left = eval_arg(scope, try_pos_arg(&raw_args, 0)?)?;
    let callable = match evaluated_left.as_ref(){
        EvalValue::CallableValue(c) => Ok(c),
        _ => Err(EvalError::InvalidType),
    }?;

    let evaluated_right = eval_arg(scope, try_pos_arg(&raw_args, 1)?)?;
    let list = match evaluated_right.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;

    let as_mono_args: Vec<Vec<EvalValueRef>> = list.0
        .iter()
        .map(|v| vec![v.clone()]).collect();

    let mapped_values = as_mono_args.into_iter()
        .map(|mono_arg|
            eval_call_with_values(scope, callable, mono_arg)
        ).collect::<Result<Vec<EvalValueRef>,EvalError>>()?;
    Ok(EvalValue::List(List(mapped_values)).to_ref())
}


fn list_nth(list: &List, n: usize) -> EvalValueRef {
    match list.0.get(n) {
        None => EvalValue::Unit.to_ref(),
        Some(v) => v.clone(),
    }
}

fn nth_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let list_value = eval_arg(scope, try_pos_arg(&raw_args, 1)?)?;
    let list = match list_value.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;

    let pos = match eval_arg(scope, try_pos_arg(&raw_args, 0)?)?.as_ref(){
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType),
    }?;

    Ok(list_nth(list, pos as usize))
}

pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![

        func("list", list_callback),
        func("nth", nth_callback),
        func("map", map_callback),

    ]
}
