use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{types, variable, definers};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_integer(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::Integer(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "int".to_string(),
            });
        }

        let is_num = letter_char.parse::<isize>().is_ok();
        if is_num || letter_char == "x" && data.raw.starts_with('0') {

            if data.raw == "0x" {
                panic!("[ParserError]: Hexadecimal are not supported yet")
            }

            if data.complete && last_char.parse::<isize>().is_err() && last_char != "x" {
                errors.push(error::Error {
                    scope: "integer_processor".to_string(),
                    debug_message: "2ffb6e752b390c249fddb3ed67584944".to_string(),
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
                //TODO GROW UP SİZES i8 -> i16 ~ u8 -> u16
                data.raw = data.raw.to_string() + letter_char;

                if let Ok(nm) = data.raw.parse::<i8>() {
                    data.data.value = types::integer_type::IntegerSize::I8(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I8;
                } else if let Ok(nm) = data.raw.parse::<i16>() {
                    data.data.value = types::integer_type::IntegerSize::I16(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I16;
                } else if let Ok(nm) = data.raw.parse::<i32>() {
                    data.data.value = types::integer_type::IntegerSize::I32(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I32;
                } else if let Ok(nm) = data.raw.parse::<i64>() {
                    data.data.value = types::integer_type::IntegerSize::I64(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I64;
                } else if let Ok(nm) = data.raw.parse::<i128>() {
                    data.data.value = types::integer_type::IntegerSize::I128(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::I128;
                } else if let Ok(nm) = data.raw.parse::<isize>() {
                    data.data.value = types::integer_type::IntegerSize::Isize(nm);
                    data.data.rtype = types::integer_type::IntegerTypes::ISize;
                } else {
                    errors.push(error::Error {
                        scope: "integer_processor".to_string(),
                        debug_message: "7d7bdcf4fcda53b49cb3ea564d866efc".to_string(),
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
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
                data.complete = true;
            }
        } else if letter_char == "." {
            data.complete = true;
            if next_char.parse::<i8>().is_ok() {
                //Float
                itered_data.data.value =
                    types::Types::Float(types::float_type::FloatTypeCollector {
                        base: data.raw.clone(),
                        at_point: true,
                        ..Default::default()
                    });
            } else {
                itered_data.data.value =
                    types::Types::Reference(types::reference_type::ReferenceType {
                        reference: Box::new(itered_data.data.value.clone()),
                        on_dot: false,
                        chain: Vec::new(),
                    });
                type_processors::reference::collect_reference(
                    parser,
                    itered_data,
                    errors,
                    letter_char,
                    next_char,
                    last_char,
                )
            }
        } else if types::logical_type::LogicalOperators::is_logical_operator(letter_char)
            || types::logical_type::LogicalOperators::is_logical_operator(
                &(letter_char.to_string() + &next_char),
            )
        {
            data.complete = true;
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
        } else if types::comparison_type::ComparisonOperators::is_comparison_operator(letter_char)
            || types::comparison_type::ComparisonOperators::is_comparison_operator(
                &(letter_char.to_string() + &next_char),
            )
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
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(letter_char)
            || types::arithmetic_type::ArithmeticOperators::is_arithmetic_operator(
                &(letter_char.to_string() + &next_char),
            )
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
        } else if letter_char == " " && !data.raw.is_empty() {
            data.complete = true;
        } else if letter_char != " " {
            errors.push(error::Error {
                scope: "integer_processor".to_string(),
                debug_message: "407e4f0ba35a7ef5f2bec7298e21e078".to_string(),
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
        }
    }
}
