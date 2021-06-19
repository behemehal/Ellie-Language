use crate::processors::value_processor;
use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_operator(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    options: defs::ParserOptions,
) {
    //TODO SUPPORT first operator parse
    if let types::Types::Operator(ref mut data) = itered_data.data.value {
        if !data.operator_collected {
            //Operator

            if letter_char == " "
                || (types::operator_type::Operators::might_be_operator(
                    data.data.operator.clone(),
                    &data.operator_collect,
                ) && !types::operator_type::Operators::might_be_operator(
                    data.data.operator.clone(),
                    &(data.operator_collect.clone() + letter_char),
                ))
            {
                let is_opearator = types::operator_type::Operators::resolve_operator(
                    data.data.operator.clone(),
                    &data.operator_collect,
                );
                if is_opearator.is_err() {
                    if letter_char == " " {
                        data.operator_collected = true;
                    } else {
                        errors.push(error::Error {
                            debug_message: "1314fbc5e77d6d17b3ce04d224c38636".to_string(),
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
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if let Ok(parsed_operator) = is_opearator {
                    data.data.operator = parsed_operator;
                    data.operator_collected = true;
                }
            } else if types::operator_type::Operators::might_be_operator(
                data.data.operator.clone(),
                &(data.operator_collect.clone() + letter_char),
            ) {
                data.operator_collect += letter_char;
            } else {
                errors.push(error::Error {
                    debug_message: "3c8f0c6ac57f269d86c9ce6db370ab56".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            }
        } else {
            //Second
            let mut will_be_itered = data.itered_cache.clone();
            data.second_is_not_null = true;
            let itered_child = value_processor::collect_value(
                &mut will_be_itered,
                letter_char,
                next_char,
                last_char,
                defs::CursorPosition(0, 0),
                options,
            );
            if itered_child.errors.is_empty() {
                for returned_error in itered_child.errors {
                    let mut edited = returned_error;
                    edited.pos.range_start.0 += pos.0;
                    edited.pos.range_start.1 += pos.1;
                    edited.pos.range_end.0 += pos.0;
                    edited.pos.range_end.1 += pos.1;
                    errors.push(edited);
                }
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
                            itered_data.data.value =
                                types::Types::Operator(types::operator_type::OperatorTypeCollector {
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
                                })
                        }
                        _ => {
                            data.data.second = Box::new(itered_child.itered_data.data.value.clone());
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


