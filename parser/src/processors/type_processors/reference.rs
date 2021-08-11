use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_reference(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::Reference(ref mut reference_data) = itered_data.data.value {

        if letter_char == "." && !reference_data.on_dot{
            reference_data.on_dot = true;
        } else {}

    }
}
