use crate::expect_copy_type;
use crate::scope::ScopeRef;
use crate::stdlib::util::func;
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};
use crate::value::{Copyable, EvalContext, EvalResult, EvalValue, EvalError, ReferenceValue};
use crate::value::callable::Callable;
use crate::value::numeric::Numeric;


fn num_cast_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs, cast: fn(Numeric) -> Numeric) -> EvalResult {
    let arg = args.try_pos(0)?.evaluated(scope)?.0;
    let v = expect_copy_type!(arg, Copyable::Numeric(n) => n, None)?;
    let casted = cast(v);
    Ok((EvalValue::Copyable(Copyable::Numeric(casted)), EvalContext::none()))
}

fn int_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    num_cast_callback(scope, _ctx, args, |n| n.cast_int())
}

fn float_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    num_cast_callback(scope, _ctx, args, |n| n.cast_fp())
}

fn type_check_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs, check: impl Fn(EvalValue) -> bool) -> EvalResult {
    let arg = args.try_pos(0)?.evaluated(scope)?.0;
    let check_ret = check(arg);
    let ret = if check_ret{
        EvalValue::Copyable(Copyable::True)
    }else{
        EvalValue::Copyable(Copyable::Unit)
    };
    Ok((ret, EvalContext::none()))
}

fn ref_type_check_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs, check: fn(&ReferenceValue) -> bool) -> EvalResult {
    type_check_callback(scope, _ctx, args, |t|
        match &t {
            EvalValue::Reference(r) => check(r.as_ref()),
            _ => false,
        }
    )
}


fn is_unit_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    type_check_callback(scope, _ctx, args, |t| matches!(t, EvalValue::Copyable(Copyable::Unit)))
}

fn is_numeric_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    type_check_callback(scope, _ctx, args, |t| matches!(t, EvalValue::Copyable(Copyable::Numeric(_))))
}

fn is_int_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    type_check_callback(scope, _ctx, args, |t| matches!(t, EvalValue::Copyable(Copyable::Numeric(Numeric::Integer(_)))))
}

fn is_float_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    type_check_callback(scope, _ctx, args, |t| matches!(t, EvalValue::Copyable(Copyable::Numeric(Numeric::Floating(_)))))
}

fn is_list_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    ref_type_check_callback(scope, _ctx, args, |r| matches!(r, ReferenceValue::List(_)))
}

fn is_callable_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    ref_type_check_callback(scope, _ctx, args, |r| matches!(r, ReferenceValue::CallableValue(_)))
}

fn is_builtin_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    ref_type_check_callback(scope, _ctx, args, |r| matches!(r, ReferenceValue::CallableValue(Callable::Internal(_))))
}

fn is_lambda_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    ref_type_check_callback(scope, _ctx, args, |r| matches!(r, ReferenceValue::CallableValue(Callable::Lambda(_))))
}

fn is_function_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    ref_type_check_callback(scope, _ctx, args, |r| matches!(r, ReferenceValue::CallableValue(Callable::Function(_))))
}



pub fn std_types() -> Vec<BuiltinFunction> {
    vec![
        func("int", int_callback),
        func("float", int_callback),

        func("is-unit?", is_unit_callback),
        func("is-unit?", is_unit_callback),
        func("is-numeric?", is_numeric_callback),
        func("is-int?", is_int_callback),
        func("is-float?", is_float_callback),
        func("is-unit?", is_unit_callback),
        func("is_list?", is_list_callback),
        func("is-callable?", is_callable_callback),
        func("is-builtin?", is_builtin_callback),
        func("is-lambda?", is_lambda_callback),
        func("is-function?", is_function_callback),
    ]
}