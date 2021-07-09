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
                        debug_message: "replace".to_string(),
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
                    variabledata.value += letter_char;
                }

                if variabledata.value == "true" || variabledata.value == "false" {
                    itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                        value: variabledata.value == "true",
                        raw: variabledata.value.clone(),
                    });
                }
            } else if !variabledata.value.is_empty() {
                if letter_char == ";" {
                    variabledata.value_complete = true;
                } else if letter_char == "." {
                    variabledata.value_complete = true;
                    itered_data.data.value =
                        types::Types::Refference(types::refference_type::RefferenceType {
                            refference: Box::new(itered_data.data.value.clone()),
                            on_dot: false,
                            chain: Vec::new(),
                        });
                    type_processors::refference::collect_refference(
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
                    errors.push(error::Error {
                        scope: "variable_processor".to_string(),
                        debug_message: "a7f3bb2ff5b6347dae9262fb25307692".to_string(),
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
                    debug_message: "replace".to_string(),
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
