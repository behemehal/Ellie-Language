use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::syntax::{types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error};

pub fn collect_char<F, E>(
    parser: parser::Parser<F, E>,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Copy + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Copy
        + Sized,
{
    if let types::Types::Char(ref mut data) = itered_data.data.value {
        if itered_data.data.dynamic {
            itered_data.data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                crate::syntax::definers::GenericType {
                    rtype: "char".to_owned(),
                    hash: "ellie_char_hash".to_owned(),
                },
            );
        }

        if letter_char == "'" && last_char != "\\" {
            if data.value == '\0' {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "char_function".to_owned(),
                    debug_message: "67705f3931ffba4c5c94411c805d55a5".to_owned(),
                    title: error::errorList::error_s14.title.clone(),
                    code: error::errorList::error_s14.code,
                    message: error::errorList::error_s14.message.clone(),
                    builded_message: error::BuildedError::build_from_string(
                        error::errorList::error_s14.message.clone(),
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos.clone().pop_char(1),
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
            data.complete = true;
        } else if !data.complete {
            if data.value != '\0' {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "char_function".to_owned(),
                    debug_message: "2b196de5549d524bf89af7248cb4878f".to_owned(),
                    title: error::errorList::error_s15.title.clone(),
                    code: error::errorList::error_s15.code,
                    message: error::errorList::error_s15.message.clone(),
                    builded_message: error::BuildedError::build_from_string(
                        error::errorList::error_s15.message.clone(),
                    ),
                    pos: defs::Cursor {
                        range_start: parser.pos.clone().pop_char(1),
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            } else {
                data.value = letter_char.chars().next().unwrap();
            }
        } else if ellie_core::utils::is_extended(letter_char, next_char).is_some() {
            data.complete = true;
            match ellie_core::utils::is_extended(letter_char, next_char).unwrap() {
                ellie_core::utils::FoundExtended::Reference => {
                    itered_data.data.value =
                        types::Types::Reference(types::reference_type::ReferenceTypeCollector {
                            data: types::reference_type::ReferenceType {
                                reference_pos: itered_data.data.value_pos,
                                reference: Box::new(itered_data.data.value.clone()),
                                chain: Vec::new(),
                            },
                            root_available: true,
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
                scope: "char_function".to_owned(),
                debug_message: "57dd97e5f19893130998c827031fe207".to_owned(),
                title: error::errorList::error_s1.title.clone(),
                code: error::errorList::error_s1.code,
                message: error::errorList::error_s1.message.clone(),
                builded_message: error::Error::build(
                    error::errorList::error_s1.message.clone(),
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
    } else {
        panic!("Unexpected parser behaviour")
    }
}
