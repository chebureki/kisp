use crate::ast::{PosExpression, SExpression};
use crate::value::{EvalContext, EvalError, EvalResult, EvalValue, ReferenceValue};
use crate::interpreter::eval_expression;
use crate::scope::ScopeRef;
use crate::stdlib::util::{func};
use crate::value::builtin::{BuiltinFunction, BuiltInFunctionArg, BuiltInFunctionArgs};
use crate::value::callable::{Callable, Function, Lambda};


//variable assignment, non mutable
fn let_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let identifier = match args.try_pos(0)?.try_expression()?{
        PosExpression{exp: SExpression::Symbol(i), ..}=> Ok(i),
        PosExpression{ ..} => Err(EvalError::InvalidType(None)),
    }?;
    //if let Some(_) = scope.lookup(identifier) {
    //    return Err(EvalError::Reassignment);
    //}

    let (evaluated, _) = args.try_pos(1)?.evaluated(scope)?;
    scope.insert(identifier.clone(), evaluated.clone());
    Ok((evaluated, EvalContext::none()))
}

fn get_argument_names(possible_args: &BuiltInFunctionArg) -> Result<Vec<String>, EvalError> {
    let block_content = match possible_args.try_expression()? {
        PosExpression{exp: SExpression::Block(c), ..} => Ok(c),
        _ => Err(EvalError::InvalidType(None)),
    }?;
    block_content.iter()
        .map(|exp|
            match exp {
                PosExpression{exp: SExpression::Symbol(i), ..} => Ok(i.clone()),
                _ => Err(EvalError::InvalidType(None))
            }
        )
        .collect()
}


fn function_declaration_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let name: String = match args.try_pos(0)?.try_expression()? {
        PosExpression{exp: SExpression::Symbol(i), ..} => Ok(i.clone()),
        _ => Err(EvalError::InvalidType(None)),
    }?;

    let arg_names: Vec<String> = get_argument_names(args.try_pos(1)?)?;
    let body = args.try_pos(2)?.try_expression()?;
    let function = Function::from(
        scope.clone(),
        name.clone(),
        arg_names,
        body
    );
    let function_value =  EvalValue::Reference(ReferenceValue::CallableValue(Callable::Function(function)).to_rc());
    scope.insert(name, function_value.clone());
    Ok((function_value, EvalContext::none()))
}

fn lambda_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let arguments: Vec<String> =  get_argument_names(args.try_pos(0)?)?;
    let body=  args.try_pos(1)?.try_expression()?.clone();
    let lambda = Lambda{
        in_scope: scope.clone(),
        arguments,
        body,
    };
    let lambda_value = EvalValue::Reference(ReferenceValue::CallableValue(Callable::Lambda(lambda)).to_rc());
    Ok((lambda_value, EvalContext::none()))
}

fn if_callback(scope: &ScopeRef, ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {

    let (condition, _) = args.try_pos(0)?.evaluated(scope)?;
    let else_expression = args.try_pos(2)
        .ok()
        .map(|v| v.try_expression())
        ;
    let then_expression = args.try_pos(1)?.try_expression()?;
    match condition {
        EvalValue::Unit if else_expression.is_none()  => Ok((EvalValue::Unit,EvalContext::none())),
        EvalValue::Unit if else_expression.is_some()  => eval_expression(
            ctx,// could be a tail call
            scope,
            else_expression.unwrap()?
        ),
        _ => eval_expression(ctx, scope, then_expression),
    }
}

fn quote_callback(_scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {

    let exp = args.try_pos(0)?.try_expression()?;
    Ok( (EvalValue::Reference(ReferenceValue::Expression(exp.clone()).to_rc()), EvalContext::none()) )
}

fn eval_callback(scope: &ScopeRef, _ctx: EvalContext, args: BuiltInFunctionArgs) -> EvalResult {
    let (arg, _) = args.try_pos(0)?.evaluated(scope)?;

    match &arg {
        EvalValue::Reference(r) => {
            match r.as_ref() {
                ReferenceValue::Expression(exp) => eval_expression(EvalContext::none(), scope, exp),
                 _ => Ok((arg, EvalContext::none())),

            }
        }
        v => Ok((arg, EvalContext::none())),
    }
}


pub fn std_lang() -> Vec<BuiltinFunction> {
    vec![
        func("let", let_callback),
        func("fn", function_declaration_callback),
        func("lambda", lambda_callback),
        func("if", if_callback),
        func("quote", quote_callback),
        func("eval", eval_callback),

    ]
}
