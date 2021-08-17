use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_operator<F>(
    parser: parser::Parser<F>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) where
    F: FnMut(ellie_core::com::Message) + core::clone::Clone,
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
                            scope: "operator_processor".to_string(),
                            debug_message: "81510a9c486825fb4589d61780c292c1".to_string(),
                            title: error::errorList::error_s13.title.clone(),
                            code: error::errorList::error_s13.code,
                            message: error::errorList::error_s13.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s13.message.clone(),
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
                    let itered_child = value_processor::collect_value(
                        parser.clone(),
                        &mut will_be_itered,
                        letter_char,
                        next_char,
                        last_char,
                    );
                    if itered_child.errors.is_empty() {
                        errors.extend(itered_child.errors);
                    }

                    if let types::Types::Operator(child_operator) =
                        itered_child.itered_data.data.value.clone()
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
                                    data.data.second =
                                        Box::new(itered_child.itered_data.data.value.clone());
                                    data.itered_cache = Box::new(itered_child.itered_data);
                                }
                            }
                        }
                    } else {
                        data.itered_cache = Box::new(itered_child.itered_data.clone());
                        data.data.second = Box::new(itered_child.itered_data.data.value);
                    }
                } else {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "operator_processor".to_string(),
                        debug_message: "88820d686b25a744074196bc99344634".to_string(),
                        title: error::errorList::error_s13.title.clone(),
                        code: error::errorList::error_s13.code,
                        message: error::errorList::error_s13.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s13.message.clone(),
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
        } else {
            //Second
            let mut will_be_itered = data.itered_cache.clone();
            data.second_is_not_null = true;
            let itered_child = value_processor::collect_value(
                parser.clone(),
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
            );
            if itered_child.errors.is_empty() {
                errors.extend(itered_child.errors);
            }

            if let types::Types::Operator(child_operator) =
                itered_child.itered_data.data.value.clone()
            {
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
                            data.data.second =
                                Box::new(itered_child.itered_data.data.value.clone());
                            data.itered_cache = Box::new(itered_child.itered_data);
                        }
                    }
                }
            } else {
                data.itered_cache = Box::new(itered_child.itered_data.clone());
                data.data.second = Box::new(itered_child.itered_data.data.value);
            }
        }
    }
}
