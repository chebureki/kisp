use crate::ast::SExpression;
use crate::interpreter::{Callable, EvalError, EvalResult, EvalValue, Function, Interpreter};
use crate::scope::ScopeRef;
use crate::stdlib::BuiltinFunction;
use crate::stdlib::util::{func, try_pos_arg};

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

fn builtin_function_declaration<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
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

fn builtin_if_declarative<'ast>(interpreter: &Interpreter<'ast>, scope: &ScopeRef<'ast>, raw_args: &'ast [SExpression]) -> EvalResult<'ast> {
    let condition = interpreter.eval_expression(scope, try_pos_arg(raw_args, 0)?)?;
    let else_expression = try_pos_arg(raw_args, 2).ok();
    let then_expression = try_pos_arg(raw_args, 1)?;
    match condition.as_ref() {
        EvalValue::Unit if else_expression.is_some() => interpreter.eval_expression(scope, else_expression.unwrap()),
        EvalValue::Unit if else_expression.is_none() => Ok(EvalValue::Unit.to_ref()),
        _ => interpreter.eval_expression(scope, then_expression)
    }
}

pub fn std_lang<'ast>() -> Vec<BuiltinFunction<'ast>> {
    vec![
        func("let", builtin_let),
        func("fn", builtin_function_declaration),
        func("if", builtin_if_declarative),
    ]
}
