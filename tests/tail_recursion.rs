use kisp::assert_match;
use kisp::testutils::quick_result;
use kisp::value::{EvalResult, EvalValue};
use kisp::value::numeric::Numeric;

#[test]
fn sum_of_n(){
    let (value, _) = quick_result(
        "
        (fn sum [n]
            [
                (fn iter [n acc]
                    (if (>= 0 n)
                        acc
                        (iter (- n 1) (+ acc n))
                    )
                )
                (iter n 0)
            ]
        )
        (sum 100)
        "
    ).unwrap();
    assert_match!(value, EvalValue::Numeric(Numeric::Integer(i)) if i==5050);
}