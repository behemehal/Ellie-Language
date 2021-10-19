use crate::alloc::borrow::ToOwned;
use crate::parser;
use crate::processors;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::{defs, error, utils};

pub fn collect_variable_value<F, E>(
    parser: &mut parser::Parser<F, E>,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: &str,
    last_char: &str,
) where
    F: FnMut(ellie_core::com::Message) + Clone + Sized,
    E: FnMut(ellie_core::defs::ParserOptions, String, bool) -> parser::ResolvedImport
        + Clone
        + Sized,
{
    let parser_clone = parser.clone();
    if let parser::Collecting::Variable(ref mut variable_data) = parser.current {
        if !variable_data.named {
            if letter_char == ":" {
                if variable_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "26c9bfc5cff316baa167bba3e0a38233".to_owned(),
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
                            debug_message: "32b08969623e5ec99a608e7d4c65b934".to_owned(),
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
                            debug_message: "b91c8269b725dab565088ba99393f60f".to_owned(),
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
                        debug_message: "dd23ffb5da449c8a36698866f0bf0f1a".to_owned(),
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
                        debug_message: "f68052bb94e47168b8613d2982998450".to_owned(),
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
                    variable_data.data.dynamic = true;
                    variable_data.named = true;
                } else if variable_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "84bed3f7a8b19ee1aa6d44203194cba4".to_owned(),
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
                            debug_message: "0327233f7cf26ee253224f6bbb3a8843".to_owned(),
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
                            debug_message: "3fce6d6030996712755df2b7eb135f1d".to_owned(),
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
                            debug_message: "cd0eb51ad0e722e1bf832180290e74b8".to_owned(),
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
                        debug_message: "ab52464793fcba8d13011030e591eefd".to_owned(),
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
                        debug_message: "58a59961ace04af64b21a31e979350ca".to_owned(),
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
                        debug_message: "2b5477e829bdf1fa2bc2067a90728a42".to_owned(),
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
                        debug_message: "235f716563b655f6042822f4a33adfb3".to_owned(),
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
                            debug_message: "dc8428ea8a1fe99155851622c2565ab7".to_owned(),
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
                    parser_clone.resolve_variable(variable_data.data.value.clone(), false);

                if let Ok(resolved_type_name) = resolved_type_name_option {
                    //nen means cannot resolve type
                    if !variable_data
                        .data
                        .rtype
                        .clone()
                        .same_as(resolved_type_name.clone())
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
                                    debug_message: "812959000fe24d74e6c3ec8fd101944b".to_owned(),
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
                                debug_message: "869dfd0a8d67603e3718a3ba4723d097".to_owned(),
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
                                debug_message: "d2c5b012acc683f086a187287f54560d".to_owned(),
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
                        debug_message: "b3e9fae4951345678a3995b8640c8c11".to_owned(),
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
                    debug_message: "8f3d2befeee0721318c6020b898821d8".to_owned(),
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
    } else {
        panic!("Unexpected parser behaviour")
    }
}
