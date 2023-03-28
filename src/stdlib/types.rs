use crate::expect_copy_type;
use crate::scope::ScopeRef;
use crate::stdlib::util::func;
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArgs};
use crate::value::{Copyable, EvalContext, EvalResult, EvalValue, EvalError};
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

pub fn std_types() -> Vec<BuiltinFunction> {
    vec![
        func("int", int_callback),
        func("float", int_callback),
    ]
}