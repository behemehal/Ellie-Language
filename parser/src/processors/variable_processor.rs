use crate::parser;
use crate::processors;
use crate::syntax::variable;
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectorResponse {
    parser: parser::Parser,
    data: variable::VariableCollector,
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "5cc4b6a6c0388e13f207b604790ab406".to_string(),
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
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "d8250e717c05ebd6408b1578581b5966".to_string(),
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
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "d5a2fb1cae3dc6601642afc5c5b21a7d".to_string(),
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "replace_var_227".to_string(),
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
                if !variable_data.data.dynamic {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "ef9e5227b5d2c8fefd04647d0c39e705".to_string(),
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "ef45542684e0869a3d8dbf6fcddf4985".to_string(),
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
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "4e8c926327e54c7b550d8d47f97e8f6b".to_string(),
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
                        && !parser_clone.generic_type_exists(variable_data.data.rtype.raw_name()) && !variable_data.data.dynamic
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "e4fc3f07eda245ede3d6851a41dfd56b".to_string(),
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
                    if (last_char == " " || last_char == "\n") && !variable_data.data.name.is_empty()
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "1ecb93d7dd927500cd2932a4936fc180".to_string(),
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "c2e721fe71089a3cbb00e8307a1f17b8".to_string(),
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "replace_var_247".to_string(),
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "replace_var_227".to_string(),
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
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "8d04a6065a01abb4da05be86c2ac81e4".to_string(),
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
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "d31848b31da7efb0afd1385706d41fb9".to_string(),
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

                    if parser_clone.generic_type_exists(variable_data.data.rtype.raw_name()) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "f61ac929f9c659d9be2224edf8069fc1".to_string(),
                            title: error::errorList::error_s27.title.clone(),
                            code: error::errorList::error_s27.code,
                            message: error::errorList::error_s27.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s27.message.clone(),
                            ),
                            pos: variable_data.data.value_pos,
                        });
                    }
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "7b506ba769be986dbddbe65c73249b6c".to_string(),
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
                if parser_clone
                    .check_keyword(variable_data.data.name.clone())
                    .found
                {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "replace_var_227".to_string(),
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
                    scope: parser.scope.scope_name.clone(),
                    debug_message: "13d87b32618796312026a80884433f3b".to_string(),
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
