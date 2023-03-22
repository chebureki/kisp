use crate::ast::SExpression;
use crate::evalvalue::{Callable, EvalError, EvalResult, EvalValue, EvalValueRef, Function, Lambda};
use crate::interpreter::eval_expression;
use crate::scope::ScopeRef;
use crate::evalvalue::BuiltinFunction;
use crate::stdlib::util::{eval_arg, expect_expression_value, func, try_pos_arg};

//variable assignment, non mutable
fn let_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let identifier = match expect_expression_value(try_pos_arg(&raw_args, 0) ?)?{
        SExpression::Symbol(i) => Ok(i),
        _ => Err(EvalError::InvalidType),
    }?;
    if let Some(_) = scope.lookup(identifier) {
        return Err(EvalError::Reassignment);
    }

    let evaluated = eval_arg(scope, try_pos_arg(&raw_args, 1)?)?;
    scope.insert(identifier.clone(), evaluated.clone());
    Ok(evaluated)
}

fn get_argument_names(possible_args: &EvalValueRef) -> Result<Vec<String>, EvalError> {
    let block_content = match expect_expression_value(&possible_args)? {
        SExpression::Block(c) => Ok(c),
        _ => Err(EvalError::InvalidType),
    }?;
    block_content.iter()
        .map(|exp|
            match exp {
                SExpression::Symbol(i) => Ok(i.clone()),
                _ => Err(EvalError::InvalidType)
            }
        )
        .collect()
}

fn function_declaration_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let name: String = match expect_expression_value(try_pos_arg(&raw_args, 0)?)? {
        SExpression::Symbol(i) => Ok(i.clone()),
        _ => Err(EvalError::InvalidType),
    }?;

    let args: Vec<String> =  get_argument_names(try_pos_arg(&raw_args, 1)?)?;
    let body: &SExpression = expect_expression_value(try_pos_arg(&raw_args, 2)?)?;
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

fn lambda_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let arguments: Vec<String> =  get_argument_names(try_pos_arg(&raw_args, 0)?)?;
    let body=  expect_expression_value(try_pos_arg(&raw_args, 1)?)?.clone();
    let lambda = Lambda{
        in_scope: scope.clone(),
        arguments,
        body,
    };
    let lambda_value = EvalValue::CallableValue(Callable::Lambda(lambda)).to_ref();
    Ok(lambda_value)
}
fn if_callback(scope: &ScopeRef, raw_args: Vec<EvalValueRef>) -> EvalResult {
    let condition = eval_arg(scope,try_pos_arg(&raw_args, 0)?)?;
    let else_expression = try_pos_arg(&raw_args, 2)
        .ok()
        .map(|v| expect_expression_value(v))
        ;
    let then_expression = expect_expression_value(try_pos_arg(&raw_args, 1)?)?;
    match condition.as_ref() {
        EvalValue::Unit if else_expression.is_some() => eval_expression(scope, else_expression.unwrap()?),
        EvalValue::Unit if else_expression.is_none() => Ok(EvalValue::Unit.to_ref()),
        _ => eval_expression(scope, then_expression)
    }
}

pub fn std_lang() -> Vec<BuiltinFunction> {
    vec![
        func("let", let_callback),
        func("fn", function_declaration_callback),
        func("lambda", lambda_callback),
        func("if", if_callback),
    ]
}
