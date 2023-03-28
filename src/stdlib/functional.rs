use std::slice::Iter;
use crate::expect_ref_type;
use crate::interpreter::eval_call_with_values;
use crate::scope::ScopeRef;
use crate::stdlib::util::func;
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};
use crate::value::{Copyable, EvalContext, EvalError, EvalResult, EvalValue, ReferenceValue};
use crate::value::callable::Callable;
use crate::value::numeric::Numeric;
use crate::value::list::List;

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

fn filter_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {

    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let callable = expect_ref_type!(evaluated_left, ReferenceValue::CallableValue(c) => c, None)?;

    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = expect_ref_type!(evaluated_right, ReferenceValue::List(list) => list, None)?;

    let list = list.iterator()
        .map(|mono_arg|
            (eval_call_with_values(EvalContext::none(), scope, callable, vec![mono_arg.clone()], None).map(|e| e.0)
                .map(|cond| (cond, mono_arg)))
        )
        //TODO: this collect annoys me, but fine for now
        //terminate early on error
        .collect::<Result<Vec<(EvalValue, EvalValue)>,EvalError>>()?
        .into_iter()
        .filter(|(cond ,v)| !matches!(cond, EvalValue::Copyable(Copyable::Unit)))
        .map(|(_,v)| v)
        .rev()
        .collect();
    Ok((EvalValue::Reference(ReferenceValue::List(list).to_rc()), EvalContext::none()))
}

fn enumerate_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (evaluated, _) = args.try_pos(0)?.evaluated(scope)?;
    let list = expect_ref_type!(evaluated, ReferenceValue::List(list) => list, None)?;

    let list = list.iterator()
        .enumerate()
        .map(|(pos, v)|
            EvalValue::Reference(
                ReferenceValue::List(
                    List::from(vec![
                        EvalValue::Copyable(Copyable::Numeric(Numeric::Integer(pos as i32))),
                        v
                    ])
                ).to_rc()
            )
        )
        .collect::<Vec<EvalValue>>()
        .into_iter()
        .rev()
        .collect();
    Ok((EvalValue::Reference(ReferenceValue::List(list).to_rc()), EvalContext::none()))
}

fn zip_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;

    let list_left = expect_ref_type!(evaluated_left, ReferenceValue::List(list) => list, None)?;
    let list_right = expect_ref_type!(evaluated_right, ReferenceValue::List(list) => list, None)?;

    let values_left: Vec<EvalValue> = list_left.iterator().collect();
    let values_right: Vec<EvalValue> = list_right.iterator().collect();
    let zipped: Vec<(EvalValue, EvalValue)> = values_left.into_iter().zip(values_right).collect();
    let list = zipped.into_iter()
        .map( |(l,r)|
            EvalValue::Reference(
                ReferenceValue::List(
                    List::from(vec![l,r])
                ).to_rc()
            )
        )
        .rev()
        .collect();
    Ok((EvalValue::Reference(ReferenceValue::List(list).to_rc()), EvalContext::none()))

}

fn reduce_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {

    let (evaluated_left, _) = args.try_pos(0)?.evaluated(scope)?;
    let callable = expect_ref_type!(evaluated_left, ReferenceValue::CallableValue(c) => c, None)?;

    let (evaluated_right, _) = args.try_pos(1)?.evaluated(scope)?;
    let list = expect_ref_type!(evaluated_right, ReferenceValue::List(list) => list, None)?;

    //just makes it simpler to use rust's reduce function
    let oks: Vec<EvalResult> = list.iterator().map(|v| Ok((v, EvalContext::none()))).collect();
    let ret = oks.into_iter()
        .reduce(|acc, v|
            match acc {
                Ok((acc_value, _)) => eval_call_with_values(EvalContext::none(), scope, callable, vec![acc_value, v.unwrap().0], None),
                Err(e) => Err(e), //just pass it down
            }
        )
        .unwrap_or(Ok((EvalValue::Copyable(Copyable::Unit), EvalContext::none())))
        ;
    ret
}




pub fn std_functional() -> Vec<BuiltinFunction> {
    vec![
        func("map", map_callback),
        func("filter", filter_callback),
        func("enumerate", enumerate_callback),
        func("zip", zip_callback),
        func("reduce", reduce_callback),

    ]
}