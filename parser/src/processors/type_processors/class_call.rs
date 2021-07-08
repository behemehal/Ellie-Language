use crate::parser;
use crate::syntax::{types, variable};
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::error;

pub fn collect_class_call(
    _parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
) {
    if let types::Types::ClassCall(ref mut _classcalldata) = itered_data.data.value {
        panic!("NOT IMPLEMENTED");
    }
}
