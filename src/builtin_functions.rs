use std::rc::Rc;
use std::slice::Iter;
use crate::ast::SExpression;
use crate::interpreter::{EvalError, EvalResult, EvalValue, InternalCallback, Interpreter};
use crate::scope::Scope;

pub struct BuiltinFunction<'ast>{
    pub callback: InternalCallback<'ast>,
    pub name: &'static str
}

fn get_ref_val<'ast>(arg: &Rc<EvalValue<'ast>>) -> &'ast SExpression {
    match arg.as_ref() {
        EvalValue::ExpressionRef(exp) => exp,
        _ => panic!("expected expression ref")
    }
}

fn builtin_print<'ast>(interpreter: &Interpreter<'ast>, scope: Rc<Scope<'ast>>) -> EvalResult<'ast> {
    todo!()
}

fn function_with_reduction<'ast, T>(interpreter: &Interpreter<'ast>, scope: &Rc<Scope<'ast>>, value_mapping: fn(&EvalValue<'ast>) -> Result<T, EvalError>, reduction: fn(T, T) -> T) -> Result<T, EvalError>
    //where T: Copy
{
    let values: Result<Vec<T>, EvalError>=
        scope.vararg().iter()
            .map(get_ref_val)
            .map(|exp| interpreter.eval_expression(&scope, exp))
            .map(|result| result.and_then(|v| value_mapping(v.as_ref())))
            .collect();
    match values?.into_iter().reduce(reduction) {
        None => Err(EvalError::MissingArgument),
        Some(v) => Ok(v)
    }
}

fn integer_reduction<'ast>(interpreter: &Interpreter<'ast>, scope: Rc<Scope<'ast>>, reduction: fn(i32, i32) -> i32) -> EvalResult<'ast>{
    let value_mapping = |value: &EvalValue| match value {
        EvalValue::IntValue(i) => Ok(*i),
        _ => Err(EvalError::InvalidType)
    };

    function_with_reduction(
        interpreter, &scope, value_mapping, reduction
    )
        .map(|i| Rc::new(EvalValue::IntValue(i)))
}

fn builtin_add<'ast>(interpreter: &Interpreter<'ast>, scope: Rc<Scope<'ast>>) -> EvalResult<'ast> {
    integer_reduction(interpreter,scope,|a,b| a+b)
}

fn builtin_minus<'ast>(interpreter: &Interpreter<'ast>, scope: Rc<Scope<'ast>>) -> EvalResult<'ast> {
    integer_reduction(interpreter,scope,|a,b| a-b)
}

fn builtin_modulo<'ast>(interpreter: &Interpreter<'ast>, scope: Rc<Scope<'ast>>) -> EvalResult<'ast> {
    integer_reduction(interpreter,scope,|a,b| a%b)
}

fn foo(){
    let v = vec![1usize, 2, 3, 4, 5];
    let sum = v.into_iter().reduce(|a, b| a + b);
    dbg!(sum);
    /*
    let data = vec![1,2,3,4];
    data.iter().reduce(|a,b| a+b);
    dbg!(data);

     */
}

pub fn builtin_functions<'ast>() -> Vec<BuiltinFunction<'ast>> {
    foo(); //TODO: remove me
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
    ]
}