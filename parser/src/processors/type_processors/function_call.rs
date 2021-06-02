use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_function_caller(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
<<<<<<< HEAD
    options: defs::ParserOptions,
=======
    options: defs::ParserOptions
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
) {
    if let types::Types::FunctionCall(ref mut data) = itered_data.data.value {
        let mut last_param = data.params.len();
        if last_param == 0 {
            data.params
                .push(types::function_call::FunctionCallParameter::default());
            last_param = data.params.len();
        }

        let is_s_n =
            !(last_param == 0 && data.params[last_param - 1].value.is_string_non_complete());

        if letter_char == "," && is_s_n && !data.params[last_param - 1].value.is_array() {
            if data.params[last_param - 1].value.is_type_complete() {
                data.comma = true;
                data.params
                    .push(types::function_call::FunctionCallParameter::default())
            } else {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/function_call.rs:35" .to_string(),
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
                        range_start: pos.clone().skipChar(1),
                        range_end: pos.clone().skipChar(2),
                    },
                });
            }
        } else if letter_char == ")" && is_s_n {
            if data.comma {
                errors.push(error::Error {
<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
                    debug_message: "./parser/src/processors/type_processors/function_call.rs:56"
                        .to_string(),
=======
                    debug_message: "./parser/src/processors/type_processors/function_call.rs:56" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
                        range_start: pos.clone().skipChar(1),
                        range_end: pos.clone().skipChar(2),
                    },
                });
            } else {
                errors.push(error::Error {
<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
                    debug_message: "./parser/src/processors/type_processors/function_call.rs:75"
                        .to_string(),
=======
                    debug_message: "./parser/src/processors/type_processors/function_call.rs:75" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
                        range_start: pos.clone().skipChar(1),
                        range_end: pos.clone().skipChar(2),
                    },
                });
                /* TODO: Figure out what is this
                if data.params[last_param - 1].value.is_complete() || true {
                    //W?
                    data.complete = true
                } else {
                }
                */
            }
        } else {
            let mut last_param_value = variable::VariableCollector {
                data: variable::Variable {
                    value: data.params[last_param - 1].value.clone(),
                    ..Default::default()
                },
                ..variable::VariableCollector::default()
            };

            data.comma = false;

            let itered_param_value = Box::new(value_processor::collect_value(
                &mut last_param_value,
                letter_char,
                next_char,
                last_char,
                defs::CursorPosition(0, 0),
<<<<<<< HEAD
                options,
=======
<<<<<<< HEAD
                options,
=======
                options
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
            ));

            let _itered_entry = match itered_param_value.itered_data.data.value.clone() {
                types::Types::Number(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Number(match_data)),
                },
                types::Types::Operator(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Operator(match_data)),
                },
                types::Types::Bool(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Bool(match_data)),
                },
                types::Types::String(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::String(match_data)),
                },
                types::Types::Char(match_data) => types::array_type::ArrayEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Char(match_data)),
                },
                types::Types::Collective => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Refference(_) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Array(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Array(match_data)),
                },
                types::Types::Cloak(match_data) => types::array_type::ArrayEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Cloak(match_data)),
                },
                types::Types::ArrowFunction(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ArrowFunction(match_data)),
                },
                types::Types::FunctionCall(_) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Void => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::VariableType(match_data) => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::VariableType(match_data)),
                },
                types::Types::Null => types::array_type::ArrayEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
            };

            if itered_param_value.errors.is_empty() {
                for returned_error in itered_param_value.errors {
                    //errors.extend(itered_array_vector.errors);
                    let mut edited = returned_error;
                    edited.pos.range_start.0 += pos.0;
                    edited.pos.range_start.1 += pos.1;
                    edited.pos.range_end.0 += pos.0;
                    edited.pos.range_end.1 += pos.1;
                    errors.push(edited);
                }
            }
            data.params[last_param - 1].value = itered_param_value.itered_data.data.value;
        }
    }
}

