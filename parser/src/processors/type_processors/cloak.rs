use crate::processors::value_processor;
use crate::syntax::{types, variable, definers};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Cloak(ref mut data) = itered_data.data.value {
        let last_entry = data.clone().collective.len();

        let is_s_n = last_entry == 0 || data.collective[last_entry - 1].value.is_complete();

        if letter_char == "(" && !data.child_start && is_s_n {
            if !data.comma && last_entry != 0 {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/cloak.rs:25" .to_string(),
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
                data.child_start = true;
                if last_entry == 0 {
                    data.collective.push(types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Cloak(
                            types::cloak_type::CloakType::default(),
                        )),
                    });
                } else {
                    data.collective[last_entry - 1] = types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Cloak(
                            types::cloak_type::CloakType::default(),
                        )),
                    };
                }
            }
        } else if letter_char == "," && !data.child_start && is_s_n {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/cloak.rs:63" .to_string(),
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
            } else if data.comma {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/cloak.rs:82" .to_string(),
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
                if last_entry != 0 {
                    data.collective[last_entry - 1].value.make_complete();
                    data.collective[last_entry - 1].value_complete = true;
                }
                data.comma = true;
                data.layer_size += 1;
                data.collective
                    .push(types::cloak_type::CloakEntry::default());
            }
        } else if letter_char == ")" && !data.child_start && is_s_n {
            if data.comma {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/cloak.rs:112" .to_string(),
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
            } else if data.complete {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_processors/cloak.rs:131" .to_string(),
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
                if last_entry != 0 {
                    if data.collective[last_entry - 1].value == Box::new(types::Types::Null) {
                        data.collective.pop();
                    } else {
                        data.collective[last_entry - 1].value_complete = true;
                        data.collective[last_entry - 1].value.make_complete();
                    }
                }
                data.layer_size += 1;
                data.complete = true;
                itered_data.value_complete = true;
            }
        } else if data.complete && letter_char == "." && is_s_n {
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    on_dot: true,
                    chain: vec![],
                })
        } else if data.complete
            && types::logical_type::LogicalOpearators::is_opearator(letter_char)
            && is_s_n
        {
            itered_data.data.value = types::Types::Operator(types::operator_type::OperatorType {
                first: Box::new(types::Types::Cloak(data.clone())),
                first_filled: true,
                operator: types::operator_type::Operators::LogicalType(
                    types::logical_type::LogicalOpearators::Null,
                ),
                operator_collect: letter_char.to_string(),
                ..Default::default()
            });
        } else if data.complete
            && types::comparison_type::ComparisonOperators::is_opearator(letter_char)
            && is_s_n
        {
            itered_data.data.value = types::Types::Operator(types::operator_type::OperatorType {
                first: Box::new(types::Types::Cloak(data.clone())),
                first_filled: true,
                operator: types::operator_type::Operators::ComparisonType(
                    types::comparison_type::ComparisonOperators::Null,
                ),
                operator_collect: letter_char.to_string(),
                ..Default::default()
            });
        } else {
            if letter_char != " " {
                //TODO IS THIS SAFE ?
                data.comma = false;
            }

            let mut will_be_itered : variable::VariableCollector;
            if let definers::Collecting::Cloak(cloak_data) = itered_data.r#type.clone() {
                will_be_itered = if data.collective.is_empty() {
                    variable::VariableCollector::default()
                } else {
                    variable::VariableCollector {
                        r#type: cloak_data.r#type[data.collective.len() - 1].clone(),
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            } else {
                panic!("PANIC");
                will_be_itered = if data.collective.is_empty() {
                    variable::VariableCollector::default()
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            }

            let itered_cloak_vector = Box::new(value_processor::collect(
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
                defs::CursorPosition(0, 0),
            ));

            if let types::Types::Cloak(ref adata) = itered_cloak_vector.itered_data.data.value {
                if adata.complete {
                    data.child_start = false;
                }
            }

            let itered_entry = match itered_cloak_vector.itered_data.data.value {
                types::Types::Number(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Number(match_data)),
                },
                types::Types::Operator(match_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Operator(match_data)),
                },
                types::Types::Bool(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Bool(match_data)),
                },
                types::Types::String(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::String(match_data)),
                },
                types::Types::Char(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Char(match_data)),
                },
                types::Types::Collective => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Refference(_) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Array(match_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Array(match_data)),
                },
                types::Types::Cloak(match_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Cloak(match_data)),
                },
                types::Types::ArrowFunction(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ArrowFunction(match_data)),
                },
                types::Types::FunctionCall(_) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::Void => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
                types::Types::VariableType(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::VariableType(match_data)),
                },
                types::Types::Null => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                },
            };

            if !itered_cloak_vector.errors.is_empty() {
                for returned_error in itered_cloak_vector.errors {
                    //errors.extend(itered_array_vector.errors);
                    let mut edited = returned_error;
                    edited.pos.range_start.0 += pos.0;
                    edited.pos.range_start.1 += pos.1;
                    edited.pos.range_end.0 += pos.0;
                    edited.pos.range_end.1 += pos.1;
                    errors.push(edited);
                }
            }

            if data.collective.is_empty() {
                data.collective.push(itered_entry);
            } else {
                data.collective[last_entry - 1] = itered_entry;
            }
        }
    }
}

