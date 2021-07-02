use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_cloak(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::Cloak(ref mut data) = itered_data.data.value {
        let last_entry = data.clone().collective.len();

        let is_s_n = last_entry == 0 || data.collective[last_entry - 1].value.is_type_complete();

        if letter_char == "(" && !data.child_start && is_s_n {
            if !data.comma && last_entry != 0 {
                errors.push(error::Error {
                    scope: "cloak_processor".to_string(),
                    debug_message: "63a1c0196c1202caef4c731333b663c7".to_string(),
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
                    scope: "cloak_processor".to_string(),
                    debug_message: "08636af58e1da043654184cd04a1f0fc".to_string(),
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
                    scope: "cloak_processor".to_string(),
                    debug_message: "4c55ff64fabda927e3184ddee0c2d70d".to_string(),
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
                    scope: "cloak_processor".to_string(),
                    debug_message: "ebb4d1b5f6040a9731bcc1315973e91c".to_string(),
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
                    scope: "cloak_processor".to_string(),
                    debug_message: "be673ee952946f7fdc51b3d9f05c647b".to_string(),
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
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::refference::collect_refference(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if data.complete
            && types::logical_type::LogicalOpearators::is_logical_opearator(letter_char)
            && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOpearators::Null,
                        ),
                        ..Default::default()
                    },
                    first_filled: true,
                    ..Default::default()
                });
            type_processors::operator::collect_operator(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if data.complete
            && types::comparison_type::ComparisonOperators::is_comparison_opearator(letter_char)
            && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::ComparisonType(
                            types::comparison_type::ComparisonOperators::Null,
                        ),
                        ..Default::default()
                    },
                    first_filled: true,
                    ..Default::default()
                });
            type_processors::operator::collect_operator(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if data.complete
            && types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(letter_char)
            && is_s_n
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::ArithmeticType(
                            types::arithmetic_type::ArithmeticOperators::Null,
                        ),
                        ..Default::default()
                    },
                    first_filled: true,
                    ..Default::default()
                });
            type_processors::operator::collect_operator(
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else {
            if letter_char != " " {
                //TODO IS THIS SAFE ?
                data.comma = false;
            }

            let mut will_be_itered: variable::VariableCollector;
            if let definers::DefinerCollecting::Cloak(cloak_data) = itered_data.data.rtype.clone() {
                will_be_itered = if data.collective.is_empty() {
                    variable::VariableCollector {
                        data: variable::Variable {
                            rtype: cloak_data.rtype[0].clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            rtype: cloak_data.rtype[data.collective.len() - 1].clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                };
            } else {
                
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
                #[cfg(feature = "std")]
                std::println!("[ParserError:0x2]: This shouldn't have happened");
            }

            let itered_cloak_vector = Box::new(value_processor::collect_value(
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
                defs::CursorPosition(0, 0),
                options,
            ));

            if let types::Types::Cloak(ref adata) = itered_cloak_vector.itered_data.data.value {
                if adata.complete {
                    data.child_start = false;
                }
            }

            let itered_entry = match itered_cloak_vector.itered_data.data.value {
                types::Types::Integer(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Integer(match_data)),
                },
                types::Types::Float(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Float(match_data)),
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
                types::Types::ClassCall(_) => types::cloak_type::CloakEntry {
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
