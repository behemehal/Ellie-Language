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
                        debug_message: "d8f6d56e50c48758e2067473d5b044e9".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    if variabledata.data.dynamic {
                        //TODO REMOVE THIS
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "ec07ec2e0a5c98fd6f0154c053b223da".to_string(),
                            title: error::errorList::error_s11.title.clone(),
                            code: error::errorList::error_s11.code,
                            message: error::errorList::error_s11.message.clone(),
                            builded_message: error::BuildedError::build_from_string(
                                error::errorList::error_s11.message.clone(),
                            ),
                            pos: defs::Cursor {
                                range_start: parser.pos.clone().skipChar(1),
                                range_end: parser.pos.clone().skipChar(2),
                            },
                        });
                    }

                    if utils::is_reserved(&variabledata.data.name) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "8192a6244de9aa4ffd4aa4405e1e696e".to_string(),
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
                #[cfg(feature = "std")]
                std::println!(
                    "[ParserWarning]: WORKING BLIND, Read more: https://github.com/behemehal/Ellie-Language/issues/2",
                );

                if !variabledata.data.dynamic {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "b0bb5d383a6cfa14202444bb6efdcbb0".to_string(),
                        title: error::errorList::error_s8.title.clone(),
                        code: error::errorList::error_s8.code,
                        message: error::errorList::error_s8.message.clone(),
                        builded_message: error::BuildedError::build_from_string(
                            error::errorList::error_s8.message.clone(),
                        ),
                        pos: defs::Cursor {
                            range_start: parser.pos,
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else if variabledata.data.name.is_empty() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "9eb56ec5b364817c2ff4c39e9a0a80ee".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    if utils::is_reserved(&variabledata.data.name) {
                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "8192a6244de9aa4ffd4aa4405e1e696e".to_string(),
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
                            debug_message: "dec49838632cf5e0fc6477e251656003".to_string(),
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
                                range_end: parser.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        if variabledata.data.name.is_empty() {
                            variabledata.data.name_pos.range_start = parser.pos;
                        }
                        variabledata.data.name_pos.range_end = parser.pos.clone().skipChar(1);
                        variabledata.data.name = variabledata.data.name.clone() + letter_char;
                    }
                } else if letter_char != " "
                    && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                    && ((last_char == " " || last_char == "\n")
                        || !variabledata.data.name.is_empty())
                {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "054b0818b7f836c05d2647f89e76899f".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                }
            }
        } else if !variabledata.typed && !variabledata.data.dynamic {
            if letter_char == ";" {
                #[cfg(feature = "std")]
                std::println!(
                    "[ParserWarning]: WORKING BLIND, Read more: https://github.com/behemehal/Ellie-Language/issues/2",
                );
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variabledata.data.rtype.is_definer_complete() {
                    errors.push(error::Error {
                        scope: parser.scope.scope_name.clone(),
                        debug_message: "acad86a361c6da68b60d67a8fcd3947e".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    #[cfg(feature = "std")]
                    std::println!(
                        "[ParserWarning]: WORKING BLIND, Read more: https://github.com/behemehal/Ellie-Language/issues/2"
                    );
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
                        //&& errors.is_empty()
                        //We should resolve inner value
                        if collected.data.dynamic {
                            #[cfg(feature = "std")]
                            std::println!(
                                "[ParserError]: This is a error please report at: https://github.com/behemehal/Ellie-Language/issues/new?title=ParserError-{}+Dynamic+Variable+Not+Handled+Correctly&labels=bug,parser",
                                collected.data.value.get_type(),
                            );

                            #[cfg(feature = "std")]
                            std::process::exit(0);
                        }

                        errors.push(error::Error {
                            scope: parser.scope.scope_name.clone(),
                            debug_message: "5ebdb98933991ba25b33f369986807a2".to_string(),
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
                        debug_message: "f2276f727552adff776092e1f8220a59".to_string(),
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
                            range_end: parser.pos.clone().skipChar(1),
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
