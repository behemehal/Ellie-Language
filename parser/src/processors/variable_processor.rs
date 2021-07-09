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
    _options: defs::ParserOptions,
) {
    let parser_clone = parser.clone();
    if let parser::Collecting::Variable(ref mut variabledata) = parser.current {
        if !variabledata.named {
            if letter_char == ":" {
                if variabledata.data.name.is_empty() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "80c2917a69685ab5b6be2f0a5f460ced".to_string(),
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
                    if variabledata.data.dynamic {
                        //TODO REMOVE THIS
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "eb5dce4787d55ba612e289fb306055a8".to_string(),
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

                    if utils::is_reserved(&variabledata.data.name) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "e6809287b67c6b9627b27c39b3069528".to_string(),
                            title: error::errorList::error_s21.title.clone(),
                            code: error::errorList::error_s21.code,
                            message: error::errorList::error_s21.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s21.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: variabledata.data.name.clone(),
                                }],
                            ),
                            pos: variabledata.data.name_pos,
                        });
                    }

                    variabledata.named = true;
                }
            } else if letter_char == ";" && variabledata.data.dynamic {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variabledata.data.dynamic {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "77378a00ed50e9b8966c181d4e9cafbd".to_string(),
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
                } else if variabledata.data.name.is_empty() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "58603f5f5bb4797221374d807949841b".to_string(),
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
                    if utils::is_reserved(&variabledata.data.name) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "1752f30580f8d0c34ec6aaa8a99d7047".to_string(),
                            title: error::errorList::error_s21.title.clone(),
                            code: error::errorList::error_s21.code,
                            message: error::errorList::error_s21.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s21.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: variabledata.data.name.clone(),
                                }],
                            ),
                            pos: variabledata.data.name_pos,
                        });
                    }
                    if !parser_clone.type_exists(variabledata.data.rtype.raw_name())
                        && !parser_clone.generic_type_exists(variabledata.data.rtype.raw_name())
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "2e2994cb362ebd325cbcaf1f41c1afff".to_string(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: variabledata.data.rtype.raw_name(),
                                }],
                            ),
                            pos: variabledata.data.type_pos,
                        });
                    }
                    variabledata.named = true;
                }
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if (last_char == " " || last_char == "\n") && !variabledata.data.name.is_empty()
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "995ae3ed3c0e7d7ab685925fa801f466".to_string(),
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
                        if variabledata.data.name.is_empty() {
                            variabledata.data.name_pos.range_start = parser.pos;
                        }
                        variabledata.data.name_pos.range_end = parser.pos.clone().skip_char(1);
                        variabledata.data.name = variabledata.data.name.clone() + letter_char;
                    }
                } else if letter_char != " "
                    && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                    && ((last_char == " " || last_char == "\n")
                        || !variabledata.data.name.is_empty())
                {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "dbb1f6ad2eee6f1ee9f56f8dca3027c1".to_string(),
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
        } else if !variabledata.typed && !variabledata.data.dynamic {
            if letter_char == ";" {
                if !parser_clone.type_exists(variabledata.data.rtype.raw_name())
                    && !parser_clone.generic_type_exists(variabledata.data.rtype.raw_name())
                {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "a352d9598a738f1723257e2140cf0235".to_string(),
                        title: error::errorList::error_s6.title.clone(),
                        code: error::errorList::error_s6.code,
                        message: error::errorList::error_s6.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s6.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: variabledata.data.rtype.raw_name(),
                            }],
                        ),
                        pos: variabledata.data.type_pos,
                    });
                }
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variabledata.data.rtype.is_definer_complete() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "eb76108ab63ca1f07a18709662e7d085".to_string(),
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
                    if !parser_clone.type_exists(variabledata.data.rtype.raw_name())
                        && !parser_clone.generic_type_exists(variabledata.data.rtype.raw_name())
                    {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "88450981f97e7e1169953aaa2d5887aa".to_string(),
                            title: error::errorList::error_s6.title.clone(),
                            code: error::errorList::error_s6.code,
                            message: error::errorList::error_s6.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s6.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: variabledata.data.rtype.raw_name(),
                                }],
                            ),
                            pos: variabledata.data.type_pos,
                        });
                    }
                    variabledata.typed = true;
                }
            } else {
                if variabledata.data.type_pos.range_start.0 == 0
                    && variabledata.data.type_pos.range_start.1 == 0
                    && letter_char != " "
                {
                    variabledata.data.type_pos.range_start = parser.pos;
                }
                processors::definer_processor::collect_definer(
                    parser_clone,
                    &mut variabledata.data.rtype,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
                variabledata.data.type_pos.range_end = parser.pos;
            }
        } else if letter_char == ";" && variabledata.data.value.is_type_complete() {
            variabledata.data.value_pos.range_end = parser.pos;
            if let parser::Collecting::Variable(ref mut collected) = parser.current {
                if collected.data.value.is_type_complete() {
                    collected.data.pos.range_end = parser.pos;
                    collected.data.value_pos.range_end = parser.pos;

                    let resolved_type_name =
                        parser_clone.resolve_variable(collected.data.value.clone());

                    if collected.data.rtype.raw_name() != resolved_type_name {
                        //We should resolve inner value
                        if collected.data.dynamic {
                            #[cfg(feature = "std")]
                            std::println!(
                                "[ParserError]: This is a error please report at: https://github.com/behemehal/Ellie-Language/issues/new?title=ParserError-{}+Dynamic+Variable+Not+Handled+Correctly&labels=bug,parser",
                                collected.data.value.get_type(),
                            );

                            #[cfg(feature = "std")]
                            std::process::exit(1);
                        }

                        if parser_clone.generic_type_exists(collected.data.rtype.raw_name()) {
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "daed6063cee09a592e975531db4c4fa9".to_string(),
                                title: error::errorList::error_s27.title.clone(),
                                code: error::errorList::error_s27.code,
                                message: error::errorList::error_s27.message.clone(),
                                builded_message: error::BuildedError::build_from_string(
                                    error::errorList::error_s27.message.clone(),
                                ),
                                pos: collected.data.value_pos,
                            });
                        }
                            errors.push(error::Error {
                                scope: parser.scope.scope_name.clone(),
                                debug_message: "b51085f82309a0a4207c3600e2d6a7c4".to_string(),
                                title: error::errorList::error_s3.title.clone(),
                                code: error::errorList::error_s3.code,
                                message: error::errorList::error_s3.message.clone(),
                                builded_message: error::Error::build(
                                    error::errorList::error_s3.message.clone(),
                                    vec![
                                        error::ErrorBuildField {
                                            key: "token1".to_string(),
                                            value: collected.data.rtype.raw_name(),
                                        },
                                        error::ErrorBuildField {
                                            key: "token2".to_string(),
                                            value: resolved_type_name,
                                        },
                                    ],
                                ),
                                pos: collected.data.value_pos,
                            });
                        
                    }
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                } else {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "96265ee297e9b058fc0fd3e2ca36eb6c".to_string(),
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
        } else {
            if variabledata.data.value_pos.range_start.0 == 0
                && variabledata.data.value_pos.range_start.1 == 0
                && letter_char != " "
            {
                variabledata.data.value_pos.range_start = parser.pos;
            }
            let mut cd = variabledata.clone();
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
            variabledata.data.value_pos.range_end = parser.pos;
            parser.current = parser::Collecting::Variable(collected.itered_data);
        }
    }
}
