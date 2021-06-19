use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use crate::processors::type_processors;

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_variable(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::VariableType(ref mut data) = itered_data.data.value {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if current_reliability.reliable {
            data.value += letter_char;
        } else if !data.value_complete {
            if (data.value.clone() + letter_char) == "false" || (data.value.clone() + letter_char) == "true" {
                itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                    value: (data.value.clone() + letter_char).parse::<bool>().unwrap(),
                });
                type_processors::bool::collect_bool(
                    itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                    pos,
                    options,
                )
            } else {
                data.value_complete = true;
            }
        } else if letter_char == "\"" {
            itered_data.data.value = types::Types::String(types::string_type::StringType::default())
        } else if letter_char == "'" {
            itered_data.data.value = types::Types::Char(types::char_type::CharType::default())
        } else {
            errors.push(error::Error {
                debug_message: "cf055258126e88c9917f6b3c76363c88".to_string(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_string(),
                        value: current_reliability.found.to_string(),
                    }],
                ),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        pos.0,
                        (pos.1 - itered_data.raw_value.len() as i64)
                            + current_reliability.at as i64,
                    ),
                    range_end: defs::CursorPosition(
                        pos.0,
                        ((pos.1 - itered_data.raw_value.len() as i64)
                            + current_reliability.at as i64)
                            + 1,
                    ),
                },
            });
        }
    }
}






