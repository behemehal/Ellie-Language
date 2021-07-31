#![allow(clippy::unnecessary_unwrap)]
use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_float(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
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
                    debug_message: "b4a01b14d864fb650e66d08c334158e1".to_string(),
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
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
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
                        debug_message: "4264b1f390cb9eeb55256b01e0b85737".to_string(),
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
                            range_start: parser
                                .pos
                                .clone()
                                .pop_char((data.point.clone() + "." + letter_char).len()),
                            range_end: parser.pos.clone().skip_char(1),
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
                        debug_message: "4a7a776ec72db48207fa0996b88865d4".to_string(),
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
                            range_start: parser
                                .pos
                                .clone()
                                .pop_char((data.point.clone() + "." + letter_char).len()),
                            range_end: parser.pos.clone().skip_char(1),
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
                    debug_message: "a52e908a24366b12b28becca35b51e29".to_string(),
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
                        range_start: parser.pos,
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if letter_char == "." {
            itered_data.data.value =
                types::Types::Reference(types::reference_type::ReferenceType {
                    reference: Box::new(itered_data.data.value.clone()),
                    chain: Vec::new(),
                    on_dot: false,
                });
            type_processors::reference::collect_reference(
                parser,
                itered_data,
                errors,
                letter_char,
                next_char,
                last_char,
            )
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
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
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
                    operator_collect: letter_char.to_string(),
                    first_filled: true,
                    ..Default::default()
                });
        } else if letter_char == " " {
            data.complete = true;
        } else {
            errors.push(error::Error {
                scope: "float_processor".to_string(),
                debug_message: "ceeda817ef074f0dcc9657b64f69a0e7".to_string(),
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
                    range_start: parser.pos,
                    range_end: parser.pos.clone().skip_char(1),
                },
            });
        }
    }
}
