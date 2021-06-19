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
        } else if current_reliability.reliable {
            data.value += letter_char;
        } else {
            errors.push(error::Error {
                debug_message: "2b753c2fc59bbdc3d0e84f8f08a23d65".to_string(),
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
                        (pos.1 - itered_data.raw_value.len() as i64)
                            + current_reliability.at as i64,
                    ),
                    range_end: defs::CursorPosition(
                        pos.0,
                        ((pos.1 - itered_data.raw_value.len() as i64)
                            + current_reliability.at as i64)
                            + 1,
                    ),
                },
            });
        }
    }
}
