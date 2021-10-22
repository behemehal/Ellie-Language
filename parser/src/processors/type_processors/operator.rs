use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use crate::alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_operator<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + core::clone::Clone,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    //TODO SUPPORT first operator parse
    if let types::Types::Operator(ref mut data) = itered_data.data.value {
        if !data.operator_collected {
            //Operator

            if letter_char == " "
                || types::operator_type::Operators::might_be_operator(
                    data.data.operator.clone(),
                    &data.operator_collect,
                )
            {
                let is_operator = types::operator_type::Operators::resolve_operator(
                    data.data.operator.clone(),
                    &data.operator_collect,
                );

                let is_next_a_operator = types::operator_type::Operators::resolve_operator(
                    data.data.operator.clone(),
                    &(data.operator_collect.to_string() + letter_char),
                );

                if is_operator.is_err() && is_next_a_operator.is_err() {
                    if letter_char == " " {
                        data.operator_collected = true;
                    } else {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: "operator_processor".to_owned(),
                            debug_message: "870322f21e8d8eba42a988af3d95240a".to_owned(),
                            title: error::errorList::error_s13.title.clone(),
                            code: error::errorList::error_s13.code,
                            message: error::errorList::error_s13.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s13.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: letter_char.to_string(),
                                }],
                            ),
                            pos: defs::Cursor {
                                range_start: parser.pos,
                                range_end: parser.pos.clone().skip_char(1),
                            },
                        });
                    }
                } else if let Ok(parsed_operator) = is_operator {
                    if !is_next_a_operator.is_ok() {
                        data.data.operator = parsed_operator;
                        data.operator_collected = true;
                    } else {
                        data.operator_collect += letter_char;
                    }
                } else if is_next_a_operator.is_ok() {
                    data.operator_collect += letter_char;
                }
            } else if types::operator_type::Operators::is_comparison_operator(
                &(data.operator_collect.clone() + letter_char),
            ) {
                data.operator_collect += letter_char;
            } else {
                if let Ok(is_resolved_operator) = types::operator_type::Operators::resolve_operator(
                    data.data.operator.clone(),
                    &data.operator_collect,
                ) {
                    data.data.operator = is_resolved_operator;
                    data.operator_collected = true;
                    let mut will_be_itered = data.itered_cache.clone();
                    data.second_is_not_null = true;
                    value_processor::collect_value(
                        parser.clone(),
                        &mut will_be_itered,
                        errors,
                        letter_char,
                        next_char,
                        last_char,
                    );

                    if let types::Types::Operator(child_operator) =
                        will_be_itered.data.value.clone()
                    {
                        if child_operator.data.operator == data.data.operator {
                            itered_data.data.value = types::Types::Operator(
                                types::operator_type::OperatorTypeCollector {
                                    data: types::operator_type::OperatorType {
                                        cloaked: child_operator.cloaked,
                                        first: Box::new(types::Types::Operator(
                                            types::operator_type::OperatorTypeCollector {
                                                data: types::operator_type::OperatorType {
                                                    first: data.data.first.clone(),
                                                    second: child_operator.data.first,
                                                    operator: data.data.operator.clone(),
                                                    cloaked: data.cloaked,
                                                },
                                                first_filled: true,
                                                operator_collect: data.operator_collect.clone(),
                                                operator_collected: true,
                                                ..Default::default()
                                            },
                                        )),
                                        operator: child_operator.data.operator,
                                        ..Default::default()
                                    },

                                    first_filled: true,

                                    operator_collect: child_operator.operator_collect,
                                    ..Default::default()
                                },
                            )
                        } else {
                            match data.data.operator.clone() {
                                types::operator_type::Operators::ComparisonType(_) => {
                                    if child_operator.data.second == Box::new(types::Types::Null) {}
                                    itered_data.data.value = types::Types::Operator(
                                        types::operator_type::OperatorTypeCollector {
                                            data: types::operator_type::OperatorType {
                                                first: Box::new(types::Types::Operator(
                                                    types::operator_type::OperatorTypeCollector {
                                                        first_filled: true,
                                                        data: types::operator_type::OperatorType {
                                                            cloaked: data.cloaked,
                                                            first: data.data.first.clone(),
                                                            second: child_operator
                                                                .data
                                                                .first
                                                                .clone(),
                                                            operator: data.data.operator.clone(),
                                                        },
                                                        operator_collect: data
                                                            .operator_collect
                                                            .clone(),
                                                        operator_collected: true,
                                                        ..Default::default()
                                                    },
                                                )),
                                                cloaked: data.cloaked,
                                                operator: child_operator.data.operator,
                                                ..Default::default()
                                            },
                                            first_filled: true,
                                            operator_collect: child_operator.operator_collect,
                                            ..Default::default()
                                        },
                                    )
                                }
                                _ => {
                                    data.data.second = Box::new(will_be_itered.data.value.clone());
                                    data.itered_cache = will_be_itered;
                                }
                            }
                        }
                    } else {
                        data.itered_cache = will_be_itered.clone();
                        data.data.second = Box::new(will_be_itered.data.value);
                    }
                } else {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "operator_processor".to_owned(),
                        debug_message: "c793729a8a6efaec08a7bfab080bde3c".to_owned(),
                        title: error::errorList::error_s13.title.clone(),
                        code: error::errorList::error_s13.code,
                        message: error::errorList::error_s13.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s13.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
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
        } else {
            //Second
            let mut will_be_itered = data.itered_cache.clone();
            data.second_is_not_null = true;
            value_processor::collect_value(
                parser.clone(),
                &mut will_be_itered,
                errors,
                letter_char,
                next_char,
                last_char,
            );

            if will_be_itered.data.value.is_type_complete() {
                match data.data.operator.clone() {
                    types::operator_type::Operators::ComparisonType(_) => {
                        itered_data.data.rtype =
                            crate::syntax::definers::DefinerCollecting::Generic(
                                crate::syntax::definers::GenericType {
                                    rtype: "bool".to_owned(),
                                },
                            );
                    }
                    types::operator_type::Operators::LogicalType(op) => {
                        itered_data.data.rtype = match op {
                            types::logical_type::LogicalOperators::And => {
                                data.data.second.clone().to_definer()
                            }
                            types::logical_type::LogicalOperators::Or => {
                                data.data.first.clone().to_definer()
                            }
                            types::logical_type::LogicalOperators::Null => {
                                panic!("Unexpected parser behaviour");
                            }
                        };
                    }
                    types::operator_type::Operators::ArithmeticType(_) => {
                        itered_data.data.rtype = data.data.first.clone().to_definer();
                    }
                    types::operator_type::Operators::Null => panic!("Unexpected parser behaviour"),
                }
            }

            if let types::Types::Operator(child_operator) = will_be_itered.data.value.clone() {
                if child_operator.data.operator == data.data.operator {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorTypeCollector {
                            data: types::operator_type::OperatorType {
                                cloaked: child_operator.cloaked,
                                first: Box::new(types::Types::Operator(
                                    types::operator_type::OperatorTypeCollector {
                                        data: types::operator_type::OperatorType {
                                            first: data.data.first.clone(),
                                            second: child_operator.data.first,
                                            operator: data.data.operator.clone(),
                                            cloaked: data.cloaked,
                                        },
                                        first_filled: true,
                                        operator_collect: data.operator_collect.clone(),
                                        operator_collected: true,
                                        ..Default::default()
                                    },
                                )),
                                operator: child_operator.data.operator,
                                ..Default::default()
                            },
                            first_filled: true,
                            operator_collect: child_operator.operator_collect,
                            ..Default::default()
                        })
                } else {
                    match data.data.operator.clone() {
                        types::operator_type::Operators::ComparisonType(_) => {
                            if child_operator.data.second == Box::new(types::Types::Null) {}
                            itered_data.data.value = types::Types::Operator(
                                types::operator_type::OperatorTypeCollector {
                                    data: types::operator_type::OperatorType {
                                        first: Box::new(types::Types::Operator(
                                            types::operator_type::OperatorTypeCollector {
                                                first_filled: true,
                                                data: types::operator_type::OperatorType {
                                                    cloaked: data.cloaked,
                                                    first: data.data.first.clone(),
                                                    second: child_operator.data.first.clone(),
                                                    operator: data.data.operator.clone(),
                                                },
                                                operator_collect: data.operator_collect.clone(),
                                                operator_collected: true,
                                                ..Default::default()
                                            },
                                        )),
                                        cloaked: data.cloaked,
                                        operator: child_operator.data.operator,
                                        ..Default::default()
                                    },
                                    first_filled: true,
                                    operator_collect: child_operator.operator_collect,
                                    ..Default::default()
                                },
                            )
                        }
                        _ => {
                            data.data.second = Box::new(will_be_itered.data.value.clone());
                            data.itered_cache = will_be_itered;
                        }
                    }
                }
            } else {
                data.itered_cache = will_be_itered.clone();
                data.data.second = Box::new(will_be_itered.data.value);
            }
        }
    } else {
        panic!("Unexpected parser behaviour")
    }
}
