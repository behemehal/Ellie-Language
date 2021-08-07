use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::defs;
use ellie_core::error;

pub fn collect_new_call(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::NewCall(ref mut new_call_data) = itered_data.data.value {
        if !new_call_data.keyword_collected {
            if new_call_data.keyword_index == 0 && letter_char == "n" {
                new_call_data.data.keyword_pos.range_start = parser.pos.clone();
                new_call_data.keyword_index = 1;
            } else if new_call_data.keyword_index == 1 && letter_char == "e" {
                new_call_data.keyword_index = 2
            } else if new_call_data.keyword_index == 2 && letter_char == "w" {
                new_call_data.keyword_index = 3;
                new_call_data.data.keyword_pos.range_end = parser.pos.clone();
            } else if new_call_data.keyword_index == 3 && letter_char == " " {
                new_call_data.keyword_collected = true;
            } else if (letter_char == " " && new_call_data.keyword_index == 0) || letter_char != " "
            {
                errors.push(error::Error {
                    scope: "function_call_processor".to_string(),
                    debug_message: "6aa096bcb938801b2397782e181af52b".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char.to_string(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !new_call_data.value_collected {
            if new_call_data.data.value.is_type_complete() && letter_char == "(" {
                new_call_data.value_collected = true;

                let resolved_new_call = parser.resolve_new_call(new_call_data.clone());
                if let Err(e) = resolved_new_call {
                    errors.extend(e);
                } else if let Ok(e) = resolved_new_call {
                }
            } else {
                if new_call_data.data.value_pos.is_zero() {
                    new_call_data.data.value_pos.range_start = parser.pos.clone();
                }

                let mut will_be_itered = variable::VariableCollector {
                    data: variable::Variable {
                        value: *new_call_data.data.value.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let itered_ncall_vector = Box::new(value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    letter_char,
                    next_char.clone(),
                    last_char,
                ));

                if !itered_ncall_vector.errors.is_empty() {
                    errors.extend(itered_ncall_vector.errors);
                }

                new_call_data.raw_value += letter_char;
                new_call_data.data.value = Box::new(itered_ncall_vector.itered_data.data.value);
                new_call_data.data.value_pos.range_end = parser.pos.clone().skip_char(1);
            }
        }
    }
}
