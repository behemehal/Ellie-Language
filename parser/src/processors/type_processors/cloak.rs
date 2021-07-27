use crate::parser;
use crate::processors::{type_processors, value_processor};
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_cloak(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::Cloak(ref mut data) = itered_data.data.value {
        let mut last_entry = data.clone().collective.len();

        let is_s_n = last_entry == 0 || data.collective[last_entry - 1].value.is_type_complete();

        if letter_char == "(" && !data.child_start && is_s_n {
            if !data.comma && last_entry != 0 {
                errors.push(error::Error {
                    scope: "cloak_processor".to_string(),
                    debug_message: "b93c4c2866fab93a854dde36a3992078".to_string(),
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
            } else {
                data.child_start = true;
                if last_entry == 0 {
                    data.collective.push(types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Cloak(
                            types::cloak_type::CloakType::default(),
                        )),
                        location: defs::Cursor {
                            range_start: parser.pos,
                            ..Default::default()
                        },
                    });
                } else {
                    data.collective[last_entry - 1] = types::cloak_type::CloakEntry {
                        value_complete: false,
                        value: Box::new(types::Types::Cloak(
                            types::cloak_type::CloakType::default(),
                        )),
                        location: defs::Cursor {
                            range_start: parser.pos,
                            ..Default::default()
                        },
                    };
                }
            }
        } else if letter_char == "," && !data.child_start && is_s_n {
            if data.complete {
                errors.push(error::Error {
                    scope: "cloak_processor".to_string(),
                    debug_message: "fa7bf3bcaac45f88cfc5580c46d3ce94".to_string(),
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
            } else if data.comma {
                errors.push(error::Error {
                    scope: "cloak_processor".to_string(),
                    debug_message: "ea5986f313f57d1683016348f6322979".to_string(),
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
            } else {
                if last_entry != 0 {
                    data.collective[last_entry - 1].value.make_complete();
                    data.collective[last_entry - 1].value_complete = true;
                }

                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Cloak(cloak_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type =
                            parser.resolve_variable(*data.collective[last_entry - 1].value.clone());
                        if cloak_defining.rtype.len() > last_entry - 1
                            && cloak_defining.rtype[last_entry - 1].raw_name() != entry_type
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "replace_array_116".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: cloak_defining.rtype[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: entry_type,
                                        },
                                    ],
                                ),
                                pos: data.collective[last_entry - 1].location,
                            });
                        }
                    }
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
                    debug_message: "3663022fd6f03c95cd832f425930bfce".to_string(),
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
            } else if data.complete {
                errors.push(error::Error {
                    scope: "cloak_processor".to_string(),
                    debug_message: "cdc16c4fefb43e11086863e34a38bcac".to_string(),
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
            } else {
                if last_entry != 0 {
                    if data.collective[last_entry - 1].value == Box::new(types::Types::Null) {
                        data.collective.pop();
                    } else {
                        data.collective[last_entry - 1].value_complete = true;
                        data.collective[last_entry - 1].value.make_complete();
                    }
                }

                if !itered_data.data.dynamic {
                    if let definers::DefinerCollecting::Cloak(cloak_defining) =
                        itered_data.data.rtype.clone()
                    {
                        let entry_type =
                            parser.resolve_variable(*data.collective[last_entry - 1].value.clone());
                        if cloak_defining.rtype.len() > last_entry - 1
                            && cloak_defining.rtype[last_entry - 1].raw_name() != entry_type
                        {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "replace_array_116".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: cloak_defining.rtype[last_entry - 1].raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: entry_type,
                                        },
                                    ],
                                ),
                                pos: data.collective[last_entry - 1].location,
                            });
                        }
                    }
                }

                data.layer_size += 1;
                data.complete = true;
                itered_data.value_complete = true;
            }
        } else if data.complete && letter_char == "." && is_s_n {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceType {
                    reference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::reference::collect_reference(
                parser.clone(),
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
        } else if data.complete
            && is_s_n
            && types::logical_type::LogicalOperators::is_logical_operator(letter_char)
            || types::logical_type::LogicalOperators::is_logical_operator(
                &(letter_char.to_string() + &next_char),
            )
        {
            itered_data.data.value =
                types::Types::Operator(types::operator_type::OperatorTypeCollector {
                    data: types::operator_type::OperatorType {
                        first: Box::new(itered_data.data.value.clone()),
                        operator: types::operator_type::Operators::LogicalType(
                            types::logical_type::LogicalOperators::Null,
                        ),
                        ..Default::default()
                    },
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if data.complete
            && is_s_n
            && types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
            || types::comparison_type::ComparisonOperators::is_comparison_operator(
                &(letter_char.to_string() + &next_char),
            )
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
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if data.complete
            && is_s_n
            && types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
            || types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                &(letter_char.to_string() + &next_char),
            )
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
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
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
                } else if cloak_data.rtype.len() == data.collective.len() {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            rtype: cloak_data.rtype[data.collective.len() - 1].clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                } else {
                    variable::VariableCollector {
                        data: variable::Variable {
                            value: *data.collective[data.collective.len() - 1].value.clone(),
                            ..Default::default()
                        },
                        ..variable::VariableCollector::default()
                    }
                }
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
                parser.clone(),
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
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
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                            && letter_char != " "
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Float(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Float(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                            && letter_char != " "
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Operator(match_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Operator(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Bool(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Bool(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                            && letter_char != " "
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::String(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::String(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Char(match_data) => types::cloak_type::CloakEntry {
                    value_complete: match_data.complete,
                    value: Box::new(types::Types::Char(match_data.clone())),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                            && match_data.value.clone() != '\0'
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Collective(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Collective(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Reference(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Reference(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::BraceReference(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::BraceReference(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Negative(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Negative(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Array(match_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Array(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Cloak(match_data) => types::cloak_type::CloakEntry {
                    value_complete: false,
                    value: Box::new(types::Types::Cloak(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::ArrowFunction(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ArrowFunction(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::FunctionCall(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::FunctionCall(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::ClassCall(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::ClassCall(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Void => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::VariableType(match_data) => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::VariableType(match_data)),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                            && letter_char != " "
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
                types::Types::Null => types::cloak_type::CloakEntry {
                    value_complete: true,
                    value: Box::new(types::Types::Null),
                    location: defs::Cursor {
                        range_start: if data.collective.len() != 0
                            && !data.collective[last_entry - 1].location.is_zero()
                        {
                            data.collective[last_entry - 1].location.range_start
                        } else {
                            parser.pos
                        },
                        ..Default::default()
                    },
                },
            };

            if !itered_cloak_vector.errors.is_empty() {
                errors.extend(itered_cloak_vector.errors);
            }

            if data.collective.is_empty() {
                data.collective.push(itered_entry);
                last_entry = 1;
            } else {
                data.collective[last_entry - 1] = itered_entry;
            }

            data.collective[last_entry - 1].location.range_end = parser.pos.clone().skip_char(1);

            if let definers::DefinerCollecting::Cloak(cloak_def) = itered_data.data.rtype.clone() {
                if cloak_def.rtype.len() < data.collective.len() && letter_char != " " {
                    errors.push(error::Error {
                        scope: "cloak_processor".to_string(),
                        debug_message: "replace_cloak_579".to_string(),
                        title: error::errorList::error_s19.title.clone(),
                        code: error::errorList::error_s19.code,
                        message: error::errorList::error_s19.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s19.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: cloak_def.rtype.len().to_string(),
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: data.collective.len().to_string(),
                                },
                            ],
                        ),
                        pos: data.collective[last_entry - 1].location,
                    });
                }
            }
        }
    }
}
