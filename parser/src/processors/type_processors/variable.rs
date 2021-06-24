use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use crate::processors::type_processors;

use alloc::boxed::Box;
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

        if letter_char == "." {
            data.value_complete = true;
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    on_dot: false,
                    chain: Vec::new(),
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
        } else if types::logical_type::LogicalOpearators::is_logical_opearator(letter_char) {
            data.value_complete = true;
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
        } else if types::comparison_type::ComparisonOperators::is_comparison_opearator(letter_char)
        {
            data.value_complete = true;
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
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(letter_char)
        {
            data.value_complete = true;
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
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
                pos,
                options,
            )
        } else if !utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            next_char.to_string(),
        )
        .reliable
            && ((data.value.clone() + &letter_char.to_string()) == "false"
                || (data.value.clone() + &letter_char.to_string()) == "true")
        {
            itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                raw: data.value.clone(),
                ..Default::default()
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
        } else if letter_char == "(" {
            itered_data.data.value =
                types::Types::FunctionCall(types::function_call::FunctionCallCollector {
                    data: types::function_call::FunctionCall {
                        name: data.value.clone(),
                        name_pos: ellie_core::defs::Cursor {
                            range_start: pos.clone().popChar(data.value.clone().len()),
                            range_end: pos,
                        },
                        ..Default::default()
                    },
                    name_collected: true,
                    ..Default::default()
                });
        } else if current_reliability.reliable {
            data.value += letter_char;
        } else {
            errors.push(error::Error {
                debug_message: "810442f9fcea51975d4e23cc870acc26".to_string(),
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
                        (pos.1 - itered_data.raw_value.len()) + current_reliability.at,
                    ),
                    range_end: defs::CursorPosition(
                        pos.0,
                        ((pos.1 - itered_data.raw_value.len()) + current_reliability.at) + 1,
                    ),
                },
            });
        }
    }
}
