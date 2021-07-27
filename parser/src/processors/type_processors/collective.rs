use crate::parser;
use crate::syntax::{types, variable};
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::error;

pub fn collect_collective(
    _parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    _errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
) {
    if let types::Types::Collective(ref mut _data) = itered_data.data.value {
        if letter_char == "{" {}
    }
}
