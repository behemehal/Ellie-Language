use ellie_core::{defs, error};
use crate::syntax::{variable, types};
use crate::processors::type_processors;

use alloc::vec::Vec;
use alloc::string::String;

#[derive(Debug, PartialEq)]
pub struct CollectorResponse {
    pub itered_data: variable::VariableCollector,
    pub errors: Vec<error::Error>,
}

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) -> CollectorResponse {
    let mut errors: Vec<error::Error> = Vec::new();
    match &mut itered_data.data.value {
        types::Types::Number(_) => {
            type_processors::number::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        },
        types::Types::Bool(_) => (),
        types::Types::String(_) => {
            type_processors::string::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::Char(_) => {
            type_processors::char::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        },
        types::Types::Collective => (),
        types::Types::Refference(_) => {
            type_processors::refference::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::Operator(_) => {
            type_processors::operator::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::Array(_) => {
            type_processors::array::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::Cloak(_) => {
            type_processors::cloak::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::ArrowFunction(_) => {
            type_processors::arrow_function::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        },
        types::Types::FunctionCall(_) => {
            type_processors::function_call::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::Void => (),
        types::Types::VariableType(_) => {
            type_processors::variable::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
        types::Types::Null => {
            type_processors::null::collect(itered_data, &mut errors, letter_char, next_char, last_char, pos)
        }
    }
    CollectorResponse {
        itered_data: itered_data.clone(),
        errors,
    }
}
