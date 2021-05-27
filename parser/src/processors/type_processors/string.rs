use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[no_mangle]
pub extern "C" fn collect_string(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::String(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.r#type = crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    r#type: "string".to_string(),
                },
            );
        }

        if letter_char == "\"" && last_char != "\\" {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/string.rs:28"
                        .to_string(),
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
            } else {
                data.complete = true;
            }
        } else if letter_char == "." && data.complete {
            // String prototype
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    on_dot: true,
                    chain: Vec::new(),
                });
        } else if utils::is_opearators(letter_char) {
            //itered_data.data.value = types::Types::Operators(types::OperatorType {
            //    first: Box::new(itered_data.data.value.clone()),
            //    operator_collect: letter_char.to_string(),
            //    collecting_operator: true,
            //    ..Default::default()
            //});
        } else if !data.complete {
            data.value += letter_char;
        } else if letter_char != " " {
            errors.push(error::Error {
                debug_message: "./parser/src/processors/type_processors/string.rs:67".to_string(),
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
