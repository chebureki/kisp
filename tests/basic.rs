use kisp::assert_match;

use kisp::testutils::quick_result;
use kisp::value::{Copyable, EvalResult, EvalValue};
use kisp::value::numeric::Numeric;



#[test]
fn empty_source(){
    let (value, _) = quick_result(
        ""
    ).unwrap();
    assert_match!(value, EvalValue::Copyable(Copyable::Unit));
}

#[test]
fn echo_num(){
    let (value, _) = quick_result(
        "1"
    ).unwrap();
    assert_match!(value, EvalValue::Copyable(Copyable::Numeric(Numeric::Integer(i))) if i==1);
}

#[test]
fn block(){
    let (value, _) = quick_result(
        "[1 2 3 4 5]"
    ).unwrap();
    assert_match!(value, EvalValue::Copyable(Copyable::Numeric(Numeric::Integer(i))) if i==5);
}

#[test]
fn source_block(){
    let (value, _) = quick_result(
        "6 7 8 9 10"
    ).unwrap();
    assert_match!(value, EvalValue::Copyable(Copyable::Numeric(Numeric::Integer(i))) if i==10);
}

#[test]
fn unit(){
    let (value, _) = quick_result(
        "()"
    ).unwrap();
    assert_match!(value, EvalValue::Copyable(Copyable::Unit));
}

#[test]
fn addition(){
    let (value, _) = quick_result(
        "(+ 1 1)"
    ).unwrap();
    assert_match!(value, EvalValue::Copyable(Copyable::Numeric(Numeric::Integer(i))) if i==2);
}