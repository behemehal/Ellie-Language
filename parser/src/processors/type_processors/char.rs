use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use alloc::vec;

pub fn collect_char(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Char(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "char".to_string(),
                },
            );
        }

        if letter_char == "'" && last_char != "\\" {
            if data.value == '\0' {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/char.rs:26".to_string(),
                    title: error::errorList::error_s14.title.clone(),
                    code: error::errorList::error_s14.code,
                    message: error::errorList::error_s14.message.clone(),
                    builded_message: error::errorList::error_s14.message.clone(),
                    pos: defs::Cursor {
                        range_start: pos.clone().popChar(1),
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
            data.complete = true;
        } else if letter_char == "." && data.complete {
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    on_dot: true,
                    chain: Vec::new(),
                });
        } else if !data.complete {
            if data.value != '\0' {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/char.rs:48".to_string(),
                    title: error::errorList::error_s15.title.clone(),
                    code: error::errorList::error_s15.code,
                    message: error::errorList::error_s15.message.clone(),
                    builded_message: error::errorList::error_s15.message.clone(),
                    pos: defs::Cursor {
                        range_start: pos.clone().popChar(1),
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                data.value = letter_char.chars().next().unwrap();
            }
        } else if letter_char != " " {
            errors.push(error::Error {
                debug_message: "./parser/src/processors/type_processors/char.rs:63".to_string(),
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
                    range_start: pos,
                    range_end: pos.clone().skipChar(1),
                },
            });
        }
    }
}

