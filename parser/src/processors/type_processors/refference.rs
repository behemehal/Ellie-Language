use crate::parser;
use crate::syntax::{types, variable};
use ellie_core::error;

use alloc::string::String;
//use alloc::vec;
use alloc::vec::Vec;

pub fn collect_refference(
    _parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
) {
    if let types::Types::Refference(ref mut _data) = itered_data.data.value {
        panic!("To-Do");
    }
}
