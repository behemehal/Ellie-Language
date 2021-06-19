use crate::processors::type_processors;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_integer(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::Integer(ref mut data) = itered_data.data.value {
        let is_num = letter_char.parse::<isize>().is_ok();

        if is_num || letter_char == "x" && data.raw.starts_with('0') {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "100c2874158fa27d7dae463b22e54706".to_string(),
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
                /*
                if matches!(&itered_data.rtype, crate::syntax::definers::DefinerCollecting::Generic(x) if x.rtype.is_empty()) && itered_data.data.dynamic {
                    //Make type default to u16
                    itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                        crate::syntax::definers::GenericType {
                            rtype: "u16".to_string(),
                        },
                    );
                }
                */

                //TODO GROW UP SİZES i8 -> i16
                data.raw = data.raw.to_string() + letter_char;

                if let Ok(nm) = data.raw.parse::<i8>() {
                    data.value = types::integer_type::IntegerSize::I8(nm);
                    data.rtype = types::integer_type::IntegerTypes::I8;
                } else if let Ok(nm) = data.raw.parse::<i16>() {
                    data.value = types::integer_type::IntegerSize::I16(nm);
                    data.rtype = types::integer_type::IntegerTypes::I16;
                } else if let Ok(nm) = data.raw.parse::<i32>() {
                    data.value = types::integer_type::IntegerSize::I32(nm);
                    data.rtype = types::integer_type::IntegerTypes::I32;
                } else if let Ok(nm) = data.raw.parse::<i64>() {
                    data.value = types::integer_type::IntegerSize::I64(nm);
                    data.rtype = types::integer_type::IntegerTypes::I64;
                } else if let Ok(nm) = data.raw.parse::<i128>() {
                    data.value = types::integer_type::IntegerSize::I128(nm);
                    data.rtype = types::integer_type::IntegerTypes::I128;
                } else if let Ok(nm) = data.raw.parse::<isize>() {
                    data.value = types::integer_type::IntegerSize::Isize(nm);
                    data.rtype = types::integer_type::IntegerTypes::ISize;
                } else {
                    errors.push(error::Error {
                        debug_message: "ssssssssssssssssssssssssserrrq".to_string(),
                        title: error::errorList::error_s16.title.clone(),
                        code: error::errorList::error_s16.code,
                        message: error::errorList::error_s16.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s16.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "val".to_string(),
                                value: data.raw.clone(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }

                //data.complete = true;
            }
        } else if letter_char == "." {
            data.complete = true;
            if next_char.parse::<i8>().is_ok() {
                //Float
                itered_data.data.value = types::Types::Float(types::float_type::FloatTypeCollector {
                    base: data.raw.clone(),
                    at_point: true,
                    ..Default::default()
                });
            } else {
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
            }
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
        } else if letter_char == " " {
            data.complete = true;
        } else {
            errors.push(error::Error {
                debug_message: "2175c3c11a42f5add25bed8ee9bcdf4f".to_string(),
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
