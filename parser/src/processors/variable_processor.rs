use crate::parser;
use crate::processors;
use crate::syntax::variable;
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone)]
pub struct CollectorResponse {
    parser: parser::Parser,
    data: crate::parser::Collecting,
}

pub fn collect_variable_value(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::Variable(ref mut variable_data) = parser.current {
        if !variable_data.named {
            if letter_char == ":" {
                if variable_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "6da605ad6ac5218191eb172872a9fb59".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
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
                } else {
                    if variable_data.data.dynamic {
                        //TODO REMOVE THIS
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "5f235d2f3867da7428fd1435dc81d09c".to_string(),
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
                            debug_message: "cc1dc422e11769c99bf778f91b7dde77".to_string(),
                            title: error::errorList::error_s21.title.clone(),
                            code: error::errorList::error_s21.code,
                            message: error::errorList::error_s21.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s21.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: variable_data.data.name.clone(),
                                }],
                            ),
                            pos: variable_data.data.name_pos,
                        });
                    }

                    variable_data.named = true;
                }
            } else if letter_char == ";" && variable_data.data.dynamic {
                if parser_clone
                    .check_keyword(variable_data.data.name.clone())
                    .found
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "0b885c46236f4be9eb0baedf4d935d7c".to_string(),
                        title: error::errorList::error_s24.title.clone(),
                        code: error::errorList::error_s24.code,
                        message: error::errorList::error_s24.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s24.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: variable_data.data.name.clone(),
                            }],
                        ),
                        pos: variable_data.data.name_pos,
                    });
                }
                variable_data.data.pos.range_end = parser.pos.clone().skip_char(1);
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variable_data.data.dynamic {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "50a20c9ab9939b00f97c4b8ef79d1df3".to_string(),
                        title: error::errorList::error_s8.title.clone(),
                        code: error::errorList::error_s8.code,
                        message: error::errorList::error_s8.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s8.message.clone(),
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else if variable_data.data.name.is_empty() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "dca86c97a84b8903180f12f056b1a83a".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
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
                } else {
                    if utils::is_reserved(&variable_data.data.name) {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "365d29d7e2e1e87f1f0ad4a272fe4973".to_string(),
                            title: error::errorList::error_s21.title.clone(),
                            code: error::errorList::error_s21.code,
                            message: error::errorList::error_s21.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s21.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
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
                            debug_message: "c438eb90cd428ded1ea9076f09865ef3".to_string(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
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
                            debug_message: "6aa87705ee3c626261b5e4d03553deaf".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
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
                        debug_message: "3c64e45b71a10cb8a016eea2d5a130d4".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
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
        } else if !variable_data.typed && !variable_data.data.dynamic {
            if letter_char == ";" {
                if !parser_clone.type_exists(variable_data.data.rtype.raw_name())
                    && !parser_clone.generic_type_exists(variable_data.data.rtype.raw_name())
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "782efcb3840a4fc769541f9e984ce4d2".to_string(),
                        title: error::errorList::error_s6.title.clone(),
                        code: error::errorList::error_s6.code,
                        message: error::errorList::error_s6.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s6.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
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
                    .check_keyword(variable_data.data.name.clone())
                    .found
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "2b4b5e6a583b6ec4e72cca390e962131".to_string(),
                        title: error::errorList::error_s24.title.clone(),
                        code: error::errorList::error_s24.code,
                        message: error::errorList::error_s24.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s24.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: variable_data.data.name.clone(),
                            }],
                        ),
                        pos: variable_data.data.name_pos,
                    });
                }

                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variable_data.data.rtype.is_definer_complete() {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "d89c90a2374c140b7d2ba26d3796de30".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
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
                } else {
                    if !parser_clone.type_exists(variable_data.data.rtype.raw_name())
                        && !parser_clone.generic_type_exists(variable_data.data.rtype.raw_name())
                    {
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "157556e136b449300c0143b1335bff54".to_string(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
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

                let resolved_type_name =
                    parser_clone.resolve_variable(variable_data.data.value.clone());

                //nen means cannot resolve type
                if variable_data.data.rtype.raw_name() != resolved_type_name
                    && resolved_type_name != "nen"
                {
                    //We should resolve inner value
                    if variable_data.data.dynamic {
                        #[cfg(feature = "std")]
                        std::println!(
                                "[ParserError]: This is a error please report at: https://github.com/behemehal/Ellie-Language/issues/new?title=ParserError-{}+Dynamic+Variable+Not+Handled+Correctly&labels=bug,parser",
                                variable_data.data.value.get_type(),
                            );

                        #[cfg(feature = "std")]
                        std::process::exit(1);
                    }

                    if variable_data.data.rtype.raw_name() == "nullAble" {
                        if variable_data
                            .data
                            .rtype
                            .as_nullable()
                            .unwrap()
                            .value
                            .raw_name()
                            != resolved_type_name
                        {
                            errors.push(error::Error {
                                path: parser.options.path.clone(),
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "d04f829b3050981b2cdcbe4120cb58a2".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: "_".to_string()
                                                + &(variable_data
                                                    .data
                                                    .rtype
                                                    .as_nullable()
                                                    .unwrap()
                                                    .value
                                                    .raw_name()),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: resolved_type_name,
                                        },
                                    ],
                                ),
                                pos: variable_data.data.value_pos,
                            });
                        }
                    } else {
                        std::println!(
                            "[ParserWarning] Working blind, type checks are not type safe ()"
                        );
                        errors.push(error::Error {
                            path: parser.options.path.clone(),
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "6abea4d71a31249c2169fd2902190f98".to_string(),
                            title: error::errorList::error_s3.title.clone(),
                            code: error::errorList::error_s3.code,
                            message: error::errorList::error_s3.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s3.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "token1".to_string(),
                                        value: variable_data.data.rtype.raw_name(),
                                    },
                                    error::ErrorBuildField {
                                        key: "token2".to_string(),
                                        value: resolved_type_name,
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
                            debug_message: "42170aea01238769338b00899b47194d".to_string(),
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
                if parser_clone
                    .check_keyword(variable_data.data.name.clone())
                    .found
                {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "ea7fa54273176a4079af629bb5df3428".to_string(),
                        title: error::errorList::error_s24.title.clone(),
                        code: error::errorList::error_s24.code,
                        message: error::errorList::error_s24.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s24.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
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
                    debug_message: "7314e4d449e48a998c5667fdf8d48cec".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
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
            parser.collected.push(parser.current.clone());
            parser.current = parser::Collecting::None;
        } else {
            if variable_data.data.value_pos.range_start.0 == 0
                && variable_data.data.value_pos.range_start.1 == 0
                && letter_char != " "
            {
                variable_data.data.value_pos.range_start = parser.pos;
            }
            let mut cd = variable_data.clone();
            let collected = processors::value_processor::collect_value(
                parser_clone,
                &mut cd,
                letter_char,
                next_char,
                last_char,
            );
            for i in collected.errors {
                errors.push(i)
            }
            variable_data.data.value_pos.range_end = parser.pos;
            parser.current = parser::Collecting::Variable(collected.itered_data);
        }
    }
}
