#![allow(clippy::unnecessary_unwrap)]
use crate::processors::type_processors;
use crate::syntax::{types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_float(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    if let types::Types::Float(ref mut data) = itered_data.data.value {
        if !data.at_point {
            //[1].111
            if letter_char.parse::<i8>().is_ok() {
                data.base += letter_char;
            } else if letter_char == "." {
                data.at_point = true;
            } else {
                errors.push(error::Error {
                    scope: "float_processor".to_string(),
                    debug_message: "df5f89475d63468a6739bd9bec8df6fc".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "val".to_string(),
                            value: data.data.raw.clone(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
        } else if letter_char.parse::<i8>().is_ok() {
            data.point += letter_char;
            let f32_parse = data.collect().parse::<f32>();

            if f32_parse.is_ok() && data.collect().len() < 9 {
                if f32_parse.clone().unwrap().is_infinite() {
                    errors.push(error::Error {
                        scope: "float_processor".to_string(),
                        debug_message: "f3bd376bc526b3d81a35eedacc6cfd14".to_string(),
                        title: error::errorList::error_s17.title.clone(),
                        code: error::errorList::error_s17.code,
                        message: error::errorList::error_s17.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s17.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "val".to_string(),
                                value: (data.point.clone() + letter_char),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: pos
                                .clone()
                                .popChar((data.point.clone() + "." + letter_char).len()),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.data.value = types::float_type::FloatSize::F32(f32_parse.unwrap());
                    data.data.rtype = types::float_type::FloatTypes::F32;
                    data.complete = true;
                }
            } else if let Ok(flt) = data.collect().parse::<f64>() {
                if flt.is_infinite() {
                    errors.push(error::Error {
                        scope: "float_processor".to_string(),
                        debug_message: "0ae5939d1bda4a8e8416c5268c24a9aa".to_string(),
                        title: error::errorList::error_s17.title.clone(),
                        code: error::errorList::error_s17.code,
                        message: error::errorList::error_s17.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s17.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "val".to_string(),
                                value: (data.point.clone() + letter_char),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: pos
                                .clone()
                                .popChar((data.point.clone() + "." + letter_char).len()),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.data.value = types::float_type::FloatSize::F64(flt);
                    data.data.rtype = types::float_type::FloatTypes::F64;
                    data.complete = true;
                }
            } else {
                errors.push(error::Error {
                    scope: "float_processor".to_string(),
                    debug_message: "8b604926c7dfdafddbddd1a11663d399".to_string(),
                    title: error::errorList::error_s17.title.clone(),
                    code: error::errorList::error_s17.code,
                    message: error::errorList::error_s17.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s17.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "val".to_string(),
                            value: data.data.raw.clone(),
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
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
        } else if letter_char == " " {
            data.complete = true;
        } else {
            errors.push(error::Error {
                scope: "float_processor".to_string(),
                debug_message: "e2d6422198e8abbe19189417de23c99b".to_string(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "val".to_string(),
                        value: data.data.raw.clone(),
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
