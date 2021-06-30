use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_class_call(
    itered_data: &mut variable::VariableCollector,
    _errors: &mut Vec<error::Error>,
    _letter_char: &str,
    _next_char: String,
    _last_char: String,
    _pos: defs::CursorPosition,
    _options: defs::ParserOptions,
) {
    if let types::Types::ClassCall(ref mut _classcalldata) = itered_data.data.value {
        panic!("NOT IMPLEMENTED");
    }
}
