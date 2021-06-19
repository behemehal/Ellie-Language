use crate::syntax::{definers, types, variable};
use crate::processors::type_processors;
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_string(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::String(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "string".to_string(),
            });
        }

        if letter_char == "\"" && last_char != "\\" {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "208a32d29523f06a5f07edeed73b6cbf".to_string(),
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
        } else if !data.complete {
            data.value += letter_char;
        } else if letter_char == "." {
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
        } else if types::logical_type::LogicalOpearators::is_logical_opearator(letter_char) {
            data.complete = true;
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
            data.complete = true;
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
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(letter_char)
        {
            data.complete = true;
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
        
        } else if letter_char != " " {
            errors.push(error::Error {
                debug_message: "355b9eb2b4c7f94e04334bf097806f7a".to_string(),
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
