use crate::evalvalue::{BuiltinFunction};
use crate::stdlib::arithmetic::std_arithmetic;
use crate::stdlib::comparison::std_comparison;
use crate::stdlib::lang::std_lang;
use crate::stdlib::output::std_output;

mod arithmetic;
mod util;
mod output;
mod comparison;
mod lang;


pub fn std_lib_functions() -> Vec<BuiltinFunction> {
    vec![
        std_lang(),
        std_arithmetic(),
        std_comparison(),
        std_output(),

    ].into_iter().flatten().collect()
}
