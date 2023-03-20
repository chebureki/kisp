use std::iter::Map;
use std::ops::Deref;
use std::rc::Rc;
use std::slice::Iter;
use crate::ast::SExpression;
use crate::interpreter::{Callable, EvalError, EvalResult, EvalValue, EvalValueRef, Function, InternalCallback, Interpreter};
use crate::interpreter::EvalValue::CallableValue;
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

fn try_pos_arg<'ast>(raw_args: &'ast [SExpression], pos: usize) -> Result<&'ast SExpression,EvalError> {
    match raw_args.get(pos){
        None => Err(EvalError::MissingArgument),
        Some(v) => Ok(v)
    }
}

//variable assignment, non mutable
fn builtin_let<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    let identifier = match try_pos_arg(raw_args,0) ?{
        SExpression::Symbol(i) => Ok(i),
        _ => Err(EvalError::InvalidType)
    }?;
    if let Some(_) = scope.lookup(&identifier) {
        return Err(EvalError::Reassignment);
    }

    let expression = try_pos_arg(raw_args,1)?;
    let evaluated = interpreter.eval_expression(&scope, expression)?;
    scope.insert(identifier.clone(), evaluated.clone());
    Ok(evaluated)
}

fn builtin_function_get_arguments<'ast>(raw_idents: &Vec<SExpression>) -> Result<Vec<String>, EvalError> {
    raw_idents.iter()
        .map(|exp|
            match exp {
                SExpression::Symbol(i) => Ok(i.clone()),
                _ => Err(EvalError::InvalidType)
            }
        )
        .collect()
}

fn builtin_function<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    let name: String = match try_pos_arg(raw_args, 0)? {
        SExpression::Symbol(i) => i.clone(),
        _ => return Err(EvalError::InvalidType),
    };


    let args: Vec<String> = match try_pos_arg(raw_args, 1)? {
        SExpression::Block(expressions) => builtin_function_get_arguments(expressions),
        _ => Err(EvalError::InvalidType),
    }?;
    let body: &SExpression = try_pos_arg(raw_args, 2)?;
    let function = Function::from(
        scope.clone(),
        name.clone(),
        args,
        body
    );
    let function_value = EvalValue::CallableValue(Callable::Function(function)).to_ref();
    scope.insert(name, function_value.clone());
    Ok(function_value)
}

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

        BuiltinFunction{
            callback: builtin_let,
            name: "let"
        },
        BuiltinFunction{
            callback: builtin_function,
            name: "fn"
        }
    ]
}