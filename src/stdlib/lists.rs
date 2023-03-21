use crate::ast::SExpression;
use crate::evalvalue::{BuiltinFunction, Callable, EvalError, EvalResult, EvalValue, EvalValueRef, List};
use crate::scope::ScopeRef;
use crate::stdlib::util::{evaluated_args, func, try_pos_arg, try_pos_evaluated};

fn builtin_list(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    let vals: Vec<EvalValueRef> = evaluated_args(scope, raw_args)?;
    let list = List(vals);
    Ok(EvalValue::List(list).to_ref())
}

fn builtin_map(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    let callable = match try_pos_evaluated(scope, raw_args, 0)?.as_ref(){
        EvalValue::CallableValue(c) => Ok(c),
        _ => Err(EvalError::InvalidType),
    }?;

    let list = match try_pos_evaluated(scope, raw_args, 1)?.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;
    //TODO: there needs to be some sort of call with evaluated args intermediary
    todo!()
}

fn list_nth(list: &List, n: usize) -> EvalValueRef {
    match list.0.get(n) {
        None => EvalValue::Unit.to_ref(),
        Some(v) => v.clone(),
    }
}

fn builtin_nth(scope: &ScopeRef, raw_args: &'_ [SExpression]) -> EvalResult {
    let l_value = try_pos_evaluated(scope, raw_args, 0)?;
    let list = match l_value.as_ref(){
        EvalValue::List(l) => Ok(l),
        _ => Err(EvalError::InvalidType),
    }?;

    let pos = match try_pos_evaluated(scope, raw_args, 1)?.as_ref(){
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType),
    }?;

    Ok(list_nth(list, pos as usize))
}


pub fn std_lists() -> Vec<BuiltinFunction> {
    vec![
        func("list", builtin_list),
        func("map", builtin_map),
        func("nth", builtin_nth),

    ]
}
