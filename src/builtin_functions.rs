use std::iter::Map;
use std::ops::Deref;
use std::slice::Iter;
use crate::ast::SExpression;
use crate::interpreter::{EvalError, EvalResult, EvalValue, EvalValueRef, InternalCallback, Interpreter};
use crate::scope::{Scope, ScopeRef};

pub struct BuiltinFunction<'ast>{
    pub callback: InternalCallback<'ast>,
    pub name: &'static str
}

type CollectedResult<'ast> = Result<Vec<EvalValueRef<'ast>>, EvalError>;

//TODO: make this part of an iterable
fn evaluated_args<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> Result<Vec<EvalValueRef<'ast>>, EvalError> {
    raw_args.iter()
        .map(|exp| interpreter.eval_expression(scope, exp))
        .collect::<Result<Vec<EvalValueRef>, EvalError>>()
}

fn builtin_print<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    let vals: Vec<String> =
        evaluated_args(interpreter,scope,raw_args)?.iter()
            .map(|v|v.to_string())
            .collect();
            //.collect::<CollectedResult>()?;
    let payload = vals.join( " ");
    println!("{}", payload);
    Ok(EvalValue::Unit.to_ref())
}

fn function_with_reduction<'ast, T>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression], value_mapping: fn(&EvalValue<'ast>) -> Result<T, EvalError>, reduction: fn(T, T) -> T) -> Result<T, EvalError> {
    evaluated_args(interpreter,scope,raw_args)?
        .iter()
        .map(|r| value_mapping(r.as_ref()))
        //TODO: a seemingly unnecessary collect here, but it also does an early terminate on the sream
        .collect::<Result<Vec<T>, EvalError>>()?.into_iter()
        .reduce(reduction)
        .map_or(Err(EvalError::MissingArgument),|v|Ok(v))
}

fn integer_reduction<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression], reduction: fn(i32, i32) -> i32) -> EvalResult<'ast>{
    let value_mapping = |value: &EvalValue| match value {
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType)
    };

    function_with_reduction(
        interpreter, scope, raw_args, value_mapping, reduction
    )
        .map(|i| EvalValue::IntValue(i).to_ref())
}

fn builtin_add<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    integer_reduction(interpreter, scope, raw_args,|a,b| a+b)
}

fn builtin_minus<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    integer_reduction(interpreter, scope, raw_args, |a,b| a-b)
}

fn builtin_modulo<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    integer_reduction(interpreter, scope, raw_args, |a,b| a%b)
}
/*
fn try_get_arg<'ast>(scope: ScopeRef<'ast>)

//variable assignment, non mutable
fn builtin_let<'ast>(interpreter: &Interpreter<'ast>, scope: ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    let identifier = match get_ref_val(scope.vararg().get(0).unwrap()) {
        SExpression::Symbol(i) => Ok(i),
        _ => Err(EvalError::InvalidType)
    }?;
    if let Some(_) = scope.lookup(identifier) {
        return Err(EvalError::Reassignment);
    }

    let expression = get_ref_val(scope.vararg().get(1).unwrap());
    let evaluated = interpreter.eval_expression(&scope, expression)?;
    //TODO: something is fishy here
    scope.parent.clone().unwrap().insert(identifier.clone(), evaluated.clone());
    Ok(evaluated)
}

 */

pub fn builtin_functions<'ast>() -> Vec<BuiltinFunction<'ast>> {
    vec![

        BuiltinFunction{
            callback: builtin_add,
            name: "+",
        },
        BuiltinFunction{
            callback: builtin_minus,
            name: "-",
        },
        BuiltinFunction{
            callback: builtin_modulo,
            name: "%",
        },
        BuiltinFunction{
            callback: builtin_print,
            name: "print"
        },
        /*
        BuiltinFunction{
            callback: builtin_let,
            name: "let"
        }

         */
    ]
}