use crate::parser;
use crate::processors;
use ellie_core::{defs, error, utils};

use crate::alloc::borrow::ToOwned;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_variable_value<F>(
    parser: &mut parser::Parser<F>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Variable(ref mut variable_data) = parser.current {
        if !variable_data.named {
            if letter_char == ":" {
                if variable_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "12d9e6d35de90e859c49a8cfd1a54426".to_owned(),
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
                } else {
                    if variable_data.data.dynamic {
                        //TODO REMOVE THIS
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "0aa67f3519d68dfa5a103accc9f60c72".to_owned(),
                            title: error::errorList::error_s11.title.clone(),
                            code: error::errorList::error_s11.code,
                            message: error::errorList::error_s11.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s11.message.clone(),
                            ),
                            pos: defs::Cursor {
                                range_start: parser.pos.clone().skip_char(1),
                                range_end: parser.pos.clone().skip_char(2),
                            },
                        });
                    }

                    if utils::is_reserved(&variable_data.data.name) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "fcdd1bb5114d68a3a0d929333c8e557d".to_owned(),
                            title: error::errorList::error_s21.title.clone(),
                            code: error::errorList::error_s21.code,
                            message: error::errorList::error_s21.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s21.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: variable_data.data.name.clone(),
                                }],
                            ),
                            pos: variable_data.data.name_pos,
                        });
                    }

                    variable_data.named = true;
                }
            } else if letter_char == ";" {
                if parser_clone
                    .check_keyword(variable_data.data.name.clone(), false)
                    .found
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "78aa88c271978386b56caecc50da4f56".to_owned(),
                        title: error::errorList::error_s24.title.clone(),
                        code: error::errorList::error_s24.code,
                        message: error::errorList::error_s24.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s24.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: variable_data.data.name.clone(),
                            }],
                        ),
                        pos: variable_data.data.name_pos,
                    });
                }

                if !variable_data.data.dynamic {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "381b5cb08b2a3e9f84a0326dc9894ff0".to_owned(),
                        title: error::errorList::error_s8.title.clone(),
                        code: error::errorList::error_s8.code,
                        message: error::errorList::error_s8.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s8.message.clone(),
                        ),
                        pos: variable_data.data.name_pos,
                    });
                }

                variable_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variable_data.data.dynamic {
                    #[cfg(feature = "std")]
                    std::println!("\u{001b}[33m[Experimental]\u{001b}[0m: Casting as dynamic");
                    variable_data.data.dynamic = true;
                    variable_data.named = true;
                } else if variable_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "6801f15330985014e3ec4825fee1200f".to_owned(),
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
                } else {
                    if utils::is_reserved(&variable_data.data.name) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "7a75e541ddb1d56267433aab0bf6631c".to_owned(),
                            title: error::errorList::error_s21.title.clone(),
                            code: error::errorList::error_s21.code,
                            message: error::errorList::error_s21.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s21.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: variable_data.data.name.clone(),
                                }],
                            ),
                            pos: variable_data.data.name_pos,
                        });
                    }
                    if !parser_clone.type_exists(variable_data.data.rtype.raw_name())
                        && !parser_clone.generic_type_exists(variable_data.data.rtype.raw_name())
                        && !variable_data.data.dynamic
                    {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "aa58d0a249b80c38da16b1ae1b731b16".to_owned(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: variable_data.data.rtype.raw_name(),
                                }],
                            ),
                            pos: variable_data.data.type_pos,
                        });
                    }
                    variable_data.named = true;
                }
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if (last_char == " " || last_char == "\n")
                        && !variable_data.data.name.is_empty()
                    {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "02f91cd62e6cc4f67cdfbb609d92f8a8".to_owned(),
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
                    } else {
                        if variable_data.data.name.is_empty() {
                            variable_data.data.name_pos.range_start = parser.pos;
                        }
                        variable_data.data.name_pos.range_end = parser.pos.clone().skip_char(1);
                        variable_data.data.name = variable_data.data.name.clone() + letter_char;
                    }
                } else if letter_char != " "
                    && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                    && ((last_char == " " || last_char == "\n")
                        || !variable_data.data.name.is_empty())
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "5bb837cd31f5dc828abde77518527973".to_owned(),
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
            }
        } else if !variable_data.typed && !variable_data.data.dynamic {
            if variable_data.data.rtype.is_definer_complete() && letter_char == ";" {
                if !parser_clone.type_exists(variable_data.data.rtype.raw_name())
                    && !parser_clone.generic_type_exists(variable_data.data.rtype.raw_name())
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "97bd92f8aa278250963446f941fe3fc7".to_owned(),
                        title: error::errorList::error_s6.title.clone(),
                        code: error::errorList::error_s6.code,
                        message: error::errorList::error_s6.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s6.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: variable_data.data.rtype.raw_name(),
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: variable_data.data.type_pos.range_start,
                            range_end: variable_data.data.type_pos.range_end.clone().skip_char(1),
                        },
                    });
                }
                if parser_clone
                    .check_keyword(variable_data.data.name.clone(), false)
                    .found
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "711bb7fd50ff06cf58733769708c64a8".to_owned(),
                        title: error::errorList::error_s24.title.clone(),
                        code: error::errorList::error_s24.code,
                        message: error::errorList::error_s24.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s24.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: variable_data.data.name.clone(),
                            }],
                        ),
                        pos: variable_data.data.name_pos,
                    });
                }

                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if variable_data.data.rtype.is_definer_complete() && letter_char == "=" {
                if !variable_data.data.rtype.is_definer_complete() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "4447d6fe44d3142264fbbb0f5a6326b8".to_owned(),
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
                } else {
                    if !parser_clone.type_exists(variable_data.data.rtype.raw_name())
                        && !parser_clone.generic_type_exists(variable_data.data.rtype.raw_name())
                    {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "06d089dc2240b4e98da7000523ae245a".to_owned(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_owned(),
                                    value: variable_data.data.rtype.raw_name(),
                                }],
                            ),
                            pos: variable_data.data.type_pos,
                        });
                    }
                    variable_data.typed = true;
                }
            } else {
                if variable_data.data.type_pos.range_start.0 == 0
                    && variable_data.data.type_pos.range_start.1 == 0
                    && letter_char != " "
                {
                    variable_data.data.type_pos.range_start = parser.pos;
                }
                processors::definer_processor::collect_definer(
                    parser_clone,
                    &mut variable_data.data.rtype,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
                variable_data.data.type_pos.range_end = parser.pos;
            }
        } else if letter_char == ";" && variable_data.data.value.is_type_complete() {
            variable_data.data.value_pos.range_end = parser.pos;
            if variable_data.data.value.is_type_complete() {
                variable_data.data.pos.range_end = parser.pos;
                variable_data.data.value_pos.range_end = parser.pos;

                let resolved_type_name_option =
                    parser_clone.resolve_variable(variable_data.data.value.clone());

                if let Ok(resolved_type_name) = resolved_type_name_option {
                    //nen means cannot resolve type
                    if variable_data.data.rtype != resolved_type_name
                        && (resolved_type_name.raw_name() != "array"
                            || (resolved_type_name.raw_name() == "array"
                                && variable_data.data.rtype.raw_name() != "growableArray"))
                        && (resolved_type_name.raw_name() != "growableArray"
                            || (resolved_type_name.raw_name() == "growableArray"
                                && variable_data.data.rtype.raw_name() != "array"))
                    {
                        if variable_data.data.dynamic {
                            #[cfg(feature = "std")]
                            std::println!(
                                "\u{001b}[31m[ParserError]\u{001b}[0m: This is a error please report at: https://github.com/behemehal/Ellie-Language/issues/new?title=ParserError-{}+Dynamic+Variable+Not+Handled+Correctly&labels=bug,parser&template=bug_report.md",
                                variable_data.data.value.get_type(),
                            );
                        }

                        if variable_data.data.rtype.raw_name() == "nullAble" {
                            if *variable_data.data.rtype.as_nullable().unwrap().value
                                != resolved_type_name
                            {
                                errors.push(error::Error {
                                    path: parser.options.path.clone(),
                                    scope: parser.scope.scope_name.clone(),
                                    debug_message: "0f2473d59ade4e9c976fc4bd6d0e90e1".to_owned(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_owned(),
                                                value: "_".to_owned()
                                                    + &(variable_data
                                                        .data
                                                        .rtype
                                                        .as_nullable()
                                                        .unwrap()
                                                        .value
                                                        .raw_name()),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_owned(),
                                                value: resolved_type_name
                                                    .raw_name_with_extensions(),
                                            },
                                        ],
                                    ),
                                    pos: variable_data.data.value_pos,
                                });
                            }
                        } else {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "ab7130a2cff08658bfd326d80040e0bf".to_owned(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_owned(),
                                            value: variable_data
                                                .data
                                                .rtype
                                                .raw_name_with_extensions(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_owned(),
                                            value: resolved_type_name.raw_name_with_extensions(),
                                        },
                                    ],
                                ),
                                pos: variable_data.data.value_pos,
                            });
                        }

                        if parser_clone.generic_type_exists(variable_data.data.rtype.raw_name()) {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "7dcc87b479f901f6487968246f82e4fd".to_owned(),
                                title: error::errorList::error_s27.title.clone(),
                                code: error::errorList::error_s27.code,
                                message: error::errorList::error_s27.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s27.message.clone(),
                                ),
                                pos: variable_data.data.value_pos,
                            });
                        }
                    }
                } else if let Err(found_errors) = resolved_type_name_option {
                    errors.extend(found_errors)
                }

                if parser_clone
                    .check_keyword(variable_data.data.name.clone(), false)
                    .found
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "17cabcda06bde97d11a6414834b57942".to_owned(),
                        title: error::errorList::error_s24.title.clone(),
                        code: error::errorList::error_s24.code,
                        message: error::errorList::error_s24.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s24.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_owned(),
                                value: variable_data.data.name.clone(),
                            }],
                        ),
                        pos: variable_data.data.name_pos,
                    });
                }
            } else {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "eb6ffe0005876a95111fd577e86d3ad4".to_owned(),
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
            variable_data.data.value_pos.range_end = parser.pos;
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if variable_data.data.value_pos.range_start.0 == 0
                && variable_data.data.value_pos.range_start.1 == 0
                && letter_char != " "
            {
                variable_data.data.value_pos.range_start = parser.pos;
            }

            if (variable_data.collected_value == "" && letter_char != " ")
                || variable_data.collected_value != "" && variable_data.collected_value.len() == 5
            {
                variable_data.collected_value += letter_char;
                if variable_data.collected_value.len() == 5 {
                    variable_data.collected_value += "..."
                }
            }

            let mut cd = variable_data.clone();
            processors::value_processor::collect_value(
                parser_clone,
                &mut cd,
                errors,
                letter_char.clone(),
                next_char,
                last_char,
            );
            parser.current = parser::Collecting::Variable(cd);
        }
    }
}
