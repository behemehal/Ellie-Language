use crate::parser;
use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;


pub fn collect_collective(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::Collective(ref mut data) = itered_data.data.value {
        

    }
}