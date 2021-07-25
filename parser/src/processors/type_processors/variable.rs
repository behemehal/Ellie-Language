use crate::parser;
use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use crate::processors::type_processors;

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_variable(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::VariableType(ref mut variabledata) = itered_data.data.value {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_string(),
        );

        if !variabledata.value_complete {
            if current_reliability.reliable {
                if last_char == " " && !variabledata.value.is_empty() {
                    errors.push(error::Error {
                        scope: "variable_processor".to_string(),
                        debug_message: "4105e1d3ed38797bc155c18fa8ff5ba0".to_string(),
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
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    if variabledata.value.is_empty() {
                        variabledata.pos.range_start = parser.pos;
                    }
                    variabledata.value += letter_char;
                    variabledata.pos.range_end = parser.pos;
                }

                if variabledata.value == "true" || variabledata.value == "false" {
                    itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                        value: variabledata.value == "true",
                        raw: variabledata.value.clone(),
                    });
                } else if variabledata.value == "new" && next_char == " " {
                    itered_data.data.value =
                        types::Types::ClassCall(types::class_call::ClassCallCollector {
                            keyword_collected: true,
                            ..Default::default()
                        });
                }
            } else if !variabledata.value.is_empty() {
                if letter_char == ";" {
                    variabledata.value_complete = true;
                } else if letter_char == "." {
                    variabledata.value_complete = true;
                    itered_data.data.value =
                        types::Types::Reference(types::reference_type::ReferenceType {
                            reference: Box::new(itered_data.data.value.clone()),
                            on_dot: false,
                            chain: Vec::new(),
                        });
                    type_processors::reference::collect_reference(
                        parser.clone(),
                        itered_data,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                } else if letter_char == "(" {
                    itered_data.data.value =
                        types::Types::FunctionCall(types::function_call::FunctionCallCollector {
                            data: types::function_call::FunctionCall {
                                name: variabledata.value.clone(),
                                name_pos: defs::Cursor {
                                    range_start: variabledata.pos.range_start,
                                    range_end: variabledata.pos.range_end.clone().skip_char(1),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        });
                    type_processors::function_call::collect_function_caller(
                        parser.clone(),
                        itered_data,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                } else if types::logical_type::LogicalOpearators::is_logical_opearator(
                    (letter_char.to_string() + &next_char).as_str(),
                ) {
                    variabledata.value_complete = true;
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
                        parser.clone(),
                        itered_data,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                } else if types::comparison_type::ComparisonOperators::is_comparison_opearator(
                    (letter_char.to_string() + &next_char).as_str(),
                ) {
                    variabledata.value_complete = true;
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
                            operator_collect: letter_char.to_string(),
                            ..Default::default()
                        });
                    type_processors::operator::collect_operator(
                        parser.clone(),
                        itered_data,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(
                    (letter_char.to_string() + &next_char).as_str(),
                ) {
                    variabledata.value_complete = true;
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
                            operator_collect: letter_char.to_string(),
                            ..Default::default()
                        });
                    type_processors::operator::collect_operator(
                        parser.clone(),
                        itered_data,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    )
                } else if letter_char != " " {
                    std::println!("| {:#?}", parser.pos);
                    errors.push(error::Error {
                        scope: "variable_processor".to_string(),
                        debug_message: "4c0be946dc33c54145972586dbba089b".to_string(),
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
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "variable_processor".to_string(),
                    debug_message: "4686a0a80f3a9e8dccb8e7c7769d60b1".to_string(),
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
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        }
    }
}
