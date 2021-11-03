use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors::type_processors;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_variable<F, E>(
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
    let itered_data_clone = itered_data.clone();
    if let types::Types::VariableType(ref mut variable_data) = itered_data.data.value {
        let current_reliability = utils::reliable_name_range(
            utils::ReliableNameRanges::VariableName,
            letter_char.to_owned(),
        );

        if !variable_data.value_complete {
            if current_reliability.reliable {
                if last_char == " " && !variable_data.data.value.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "variable_processor".to_owned(),
                        debug_message: "ef08004e72a7b7cd858a67ecd382dbf5".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: current_reliability.found.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    if variable_data.data.value.is_empty() {
                        variable_data.data.pos.range_start = parser.pos;
                    }
                    variable_data.data.value += letter_char;
                    variable_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                    if (next_char == ";"
                        || next_char == ","
                        || next_char == " "
                        || next_char == "."
                        || next_char == "("
                        || next_char == ")"
                        || next_char == "["
                        || next_char == "]"
                        || next_char == "{"
                        || next_char == "}")
                        && !itered_data_clone.ignore_existence
                        && (variable_data.data.value != "new"
                            && variable_data.data.value != "true"
                            && variable_data.data.value != "false")
                    {
                        let found_target =
                            parser.check_keyword(variable_data.data.value.clone(), false, false);
                        if !found_target.found && !itered_data.ignore_existence {
                            variable_data.value_exists = false;
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: "variable_processor".to_owned(),
                                debug_message: "d9806c303f29c79dbdf615a588476589".to_owned(),
                                title: error::errorList::error_s6.title.clone(),
                                code: error::errorList::error_s6.code,
                                message: error::errorList::error_s6.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s6.message.clone(),
                                    vec![error::ErrorBuildField {
                                        key: "token".to_owned(),
                                        value: variable_data.data.value.clone(),
                                    }],
                                ),
                                pos: variable_data.data.pos,
                            });
                        } else {
                            variable_data.value_exists = true;
                            match found_target.found_type {
                                parser::NameCheckResponseType::Variable(variable_type) => {
                                    if !itered_data_clone
                                        .clone()
                                        .data
                                        .rtype
                                        .same_as(variable_type.data.rtype.clone())
                                        && itered_data.data.rtype.raw_name_with_extensions() != ""
                                    {
                                        errors.push(error::Error {
                                            path: parser.options.path.clone(),
                                            scope: parser.scope.scope_name.clone(),
                                            debug_message: "01375137fbb1465d1491046e9cb64d89"
                                                .to_string(),
                                            title: error::errorList::error_s3.title.clone(),
                                            code: error::errorList::error_s3.code,
                                            message: error::errorList::error_s3.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s3.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "token1".to_owned(),
                                                        value: itered_data
                                                            .data
                                                            .rtype
                                                            .raw_name_with_extensions(),
                                                    },
                                                    error::ErrorBuildField {
                                                        key: "token2".to_owned(),
                                                        value: variable_type
                                                            .data
                                                            .rtype
                                                            .raw_name_with_extensions(),
                                                    },
                                                ],
                                            ),
                                            pos: variable_data.data.pos,
                                        });
                                    }
                                    if itered_data.data.dynamic {
                                        itered_data.data.rtype = variable_type.data.rtype.clone();
                                    }
                                }
                                parser::NameCheckResponseType::Class(class_type) => {
                                    if itered_data.data.dynamic {
                                        itered_data.data.rtype =
                                            definers::DefinerCollecting::Generic(
                                                definers::GenericType {
                                                    rtype: class_type.data.name,
                                                    hash: class_type.data.hash,
                                                },
                                            );
                                    }
                                }
                                parser::NameCheckResponseType::Getter(getter_type) => {
                                    if itered_data.data.dynamic {
                                        itered_data.data.rtype = getter_type.data.rtype;
                                    }
                                }
                                parser::NameCheckResponseType::Setter(_) => {
                                    if itered_data.data.dynamic {
                                        itered_data.data.rtype =
                                            definers::DefinerCollecting::Generic(
                                                definers::GenericType {
                                                    rtype: "void".to_owned(),
                                                    hash: "ellie_null_hash".to_owned(),
                                                },
                                            );
                                    }
                                }
                                parser::NameCheckResponseType::Function(_) => {
                                    if itered_data.data.dynamic {
                                        itered_data.data.rtype =
                                            definers::DefinerCollecting::Generic(
                                                definers::GenericType {
                                                    rtype: "function".to_string(),
                                                    hash: "ellie_function_hash".to_owned(),
                                                },
                                            );
                                    }
                                }
                                parser::NameCheckResponseType::NativeFunction(_) => {
                                    if itered_data.data.dynamic {
                                        itered_data.data.rtype =
                                            definers::DefinerCollecting::Generic(
                                                definers::GenericType {
                                                    rtype: "function".to_string(),
                                                    hash: "ellie_function_hash".to_owned(),
                                                },
                                            );
                                    }
                                }
                                _ => {
                                    errors.push(error::Error {
                                        path: parser.options.path.clone(),
                                        scope: parser.scope.scope_name.clone(),
                                        debug_message: "change_variable"
                                            .to_string(),
                                        title: error::errorList::error_s41.title.clone(),
                                        code: error::errorList::error_s41.code,
                                        message: error::errorList::error_s41.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s41.message.clone(),
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "token".to_owned(),
                                                    value: "Inferring language items to variables are not yet supported".to_owned(),
                                                },
                                            ],
                                        ),
                                        pos: variable_data.data.pos,
                                    });
                                }
                            }
                        }
                    }
                }

                if variable_data.data.value == "true" || variable_data.data.value == "false" {
                    if itered_data.data.dynamic {
                        itered_data.data.rtype =
                            crate::syntax::definers::DefinerCollecting::Generic(
                                crate::syntax::definers::GenericType {
                                    rtype: "bool".to_owned(),
                                    hash: "ellie_bool_hash".to_owned(),
                                },
                            );
                    }
                    itered_data.data.value = types::Types::Bool(types::bool_type::BoolType {
                        value: variable_data.data.value == "true",
                        raw: variable_data.data.value.clone(),
                    });
                } else if variable_data.data.value == "new" && next_char == " " {
                    itered_data.data.value = types::Types::ConstructedClass(
                        types::constructed_class::ConstructedClassCollector {
                            keyword_index: 3,
                            data: types::constructed_class::ConstructedClass {
                                value: Box::new(types::Types::Null),
                                keyword_pos: defs::Cursor {
                                    range_start: parser.pos.clone().pop_char(3),
                                    range_end: parser.pos.clone(),
                                },
                                ..Default::default()
                            },
                            ..Default::default()
                        },
                    );
                }
            } else if !variable_data.data.value.is_empty() {
                if letter_char == ";" {
                    variable_data.value_complete = true;
                } else if ellie_core::utils::is_extended(letter_char, next_char).is_some() {
                    match ellie_core::utils::is_extended(letter_char, next_char).unwrap() {
                        ellie_core::utils::FoundExtended::Reference => {
                            itered_data.data.value = types::Types::Reference(
                                types::reference_type::ReferenceTypeCollector {
                                    data: types::reference_type::ReferenceType {
                                        reference_pos: itered_data.data.value_pos,
                                        reference: Box::new(itered_data.data.value.clone()),
                                        chain: Vec::new(),
                                    },
                                    root_available: true,
                                    on_dot: false,
                                    complete: false,
                                    last_entry: itered_data.data.value.clone().to_definer(),
                                },
                            );
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
                            itered_data.data.value = types::Types::Operator(
                                types::operator_type::OperatorTypeCollector {
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
                                },
                            );
                        }
                        ellie_core::utils::FoundExtended::ComparisonOperator => {
                            itered_data.data.value = types::Types::Operator(
                                types::operator_type::OperatorTypeCollector {
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
                                },
                            );
                        }
                        ellie_core::utils::FoundExtended::ArithmeticOperator => {
                            itered_data.data.value = types::Types::Operator(
                                types::operator_type::OperatorTypeCollector {
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
                                },
                            );
                        }
                        ellie_core::utils::FoundExtended::AssignmentOperator => {
                            itered_data.data.value = types::Types::Operator(
                                types::operator_type::OperatorTypeCollector {
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
                                },
                            );
                        }
                        ellie_core::utils::FoundExtended::FunctionCall => (),
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "variable_processor".to_owned(),
                        debug_message: "7f84c4afca6fcfe07c09db9bd47bc285".to_owned(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: current_reliability.found.to_string(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "variable_processor".to_owned(),
                    debug_message: "b4e6b2a6c435cc10a7b4f131b9767b4d".to_owned(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_owned(),
                            value: current_reliability.found.to_string(),
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
        panic!("Unexpected parser behaviour")
    }
}
