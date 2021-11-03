#![allow(clippy::unnecessary_unwrap)]
use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::syntax::{types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_bool<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    _last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Copy + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Copy
        + Sized,
{
    if let types::Types::Bool(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "bool".to_owned(),
                    hash: "ellie_bool_hash".to_owned(),
                },
            );
        }

        if data.raw != "true" && data.raw != "false" {
            data.raw += letter_char;
            if data.raw == "true" || data.raw == "false" {
                data.value = data.raw.parse::<bool>().unwrap();
            }
        } else if ellie_core::utils::is_extended(letter_char, next_char).is_some() {
            match ellie_core::utils::is_extended(letter_char, next_char).unwrap() {
                ellie_core::utils::FoundExtended::Reference => {
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
                }
                ellie_core::utils::FoundExtended::BracketReference => {
                    itered_data.data.value = types::Types::BracketReference(
                        types::bracket_reference_type::BracketReferenceCollector {
                            complete: false,
                            data: types::bracket_reference_type::BracketReference {
                                pos: defs::Cursor {
                                    range_start: parser.pos,
                                    ..Default::default()
                                },
                                target: itered_data.data.value.clone().to_definer(),
                            },
                            ..Default::default()
                        },
                    );
                }
                ellie_core::utils::FoundExtended::LogicalOperator => {
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
                }
                ellie_core::utils::FoundExtended::ComparisonOperator => {
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
                }
                ellie_core::utils::FoundExtended::ArithmeticOperator => {
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
                }
                ellie_core::utils::FoundExtended::AssignmentOperator => {
                    itered_data.data.value =
                        types::Types::Operator(types::operator_type::OperatorTypeCollector {
                            data: types::operator_type::OperatorType {
                                first: Box::new(itered_data.data.value.clone()),
                                operator: types::operator_type::Operators::AssignmentType(
                                    types::assignment_type::AssignmentOperators::Null,
                                ),
                                ..Default::default()
                            },
                            operator_collect: letter_char.to_string(),
                            first_filled: true,
                            ..Default::default()
                        });
                }
                ellie_core::utils::FoundExtended::FunctionCall => todo!(),
            }
        } else if letter_char != " " {
            errors.push(error::Error {
                path: parser.options.path.clone(),
                scope: "bool_function".to_owned(),
                debug_message: "47bc6acb79b04da83c5b77d4eb9c64fa".to_owned(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
                    vec![error::ErrorBuildField {
                        key: "token".to_owned(),
                        value: (data.raw).to_string(),
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
