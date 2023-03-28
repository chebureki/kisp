use crate::stdlib::arithmetic::std_arithmetic;
use crate::stdlib::comparison::std_comparison;
use crate::stdlib::lang::std_lang;
use crate::stdlib::lists::std_lists;
use crate::stdlib::output::std_output;
use crate::stdlib::types::std_types;
use crate::value::builtin::BuiltinFunction;

mod arithmetic;
mod util;
mod output;
mod comparison;
mod lang;
mod lists;
mod types;
mod functional;


pub fn std_lib_functions() -> Vec<BuiltinFunction> {
    vec![
        std_lang(),
        std_arithmetic(),
        std_comparison(),
        std_output(),
        std_lists(),
        std_types(),

    ].into_iter().flatten().collect()
}
