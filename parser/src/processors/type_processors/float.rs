#![allow(clippy::unnecessary_unwrap)]
use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_float<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    _last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    if let types::Types::Float(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                rtype: "float".to_owned(),
            });
        }

        if !data.at_point {
            //[1].111
            if letter_char.parse::<i8>().is_ok() {
                data.base += letter_char;
            } else if letter_char == "." {
                data.at_point = true;
            } else {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "float_processor".to_owned(),
                    debug_message: "747a9b9f514777d38ef50b2d09a579a5".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "val".to_owned(),
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
                        path: parser.options.path.clone(),
                        scope: "float_processor".to_owned(),
                        debug_message: "8f9730d6a579a86ca934a807efad57e7".to_owned(),
                        title: error::errorList::error_s17.title.clone(),
                        code: error::errorList::error_s17.code,
                        message: error::errorList::error_s17.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s17.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "val".to_owned(),
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
                        path: parser.options.path.clone(),
                        scope: "float_processor".to_owned(),
                        debug_message: "6ce3e21cd1ed1e90fbcd5b5a0e6a8e8a".to_owned(),
                        title: error::errorList::error_s17.title.clone(),
                        code: error::errorList::error_s17.code,
                        message: error::errorList::error_s17.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s17.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "val".to_owned(),
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
                    path: parser.options.path.clone(),
                    scope: "float_processor".to_owned(),
                    debug_message: "bcf4c483421865bd51733d2b4926df59".to_owned(),
                    title: error::errorList::error_s17.title.clone(),
                    code: error::errorList::error_s17.code,
                    message: error::errorList::error_s17.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s17.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "val".to_owned(),
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
                types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                    data: types::reference_type::ReferenceType {
                        reference_pos: itered_data.data.value_pos,
                        reference: Box::new(itered_data.data.value.clone()),
                        chain: Vec::new(),
                    },
                    root_available: false,
                    on_dot: false,
                    complete: false,
                    last_entry: itered_data.data.value.clone().to_definer(),
                });
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
                path: parser.options.path.clone(),
                scope: "float_processor".to_owned(),
                debug_message: "e372f176c005c1c9dffc404170d068e4".to_owned(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "val".to_owned(),
                        value: data.data.raw.clone(),
                    }],
                ),
                pos: defs::Cursor {
                    range_start: parser.pos,
                    range_end: parser.pos.clone().skip_char(1),
                },
            });
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
