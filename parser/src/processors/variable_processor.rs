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
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Variable(ref mut variabledata) = parser.current {
        if !variabledata.named {
            if letter_char == ":" {
                if variabledata.data.name.is_empty() {
                    errors.push(error::Error {
                        debug_message: "230fc6a68358620eb096f0e0cc69036c"
                            .to_string(),
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
                            debug_message: "be0d4986844c78190f84b23f893bcb8a"
                                .to_string(),
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
                    variabledata.named = true;
                }
            } else if letter_char == ";" && variabledata.data.dynamic {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                #[cfg(feature = "std")]
                std::println!(
                    "{}[ParserWarning]{}: WORKING BLIND, Read more: {}https://github.com/behemehal/Ellie-Language/issues/2{}",
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Yellow),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                );

                if !variabledata.data.dynamic {
                    errors.push(error::Error {
                        debug_message: "570cb25f49da7a8bdb28b331ce78b9c3"
                            .to_string(),
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
                        debug_message: "d367c41b1d0367d981c89170b05d1ec1"
                            .to_string(),
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
                    variabledata.named = true;
                }
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if last_char == " " && !variabledata.data.name.is_empty() {
                        errors.push(error::Error {
                            debug_message: "6ae4df29c1e921f452838f4ceac73698"
                                .to_string(),
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
                        variabledata.data.name = variabledata.data.name.clone() + letter_char;
                    }
                } else if letter_char != " "
                    && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                    && (last_char == " " || !variabledata.data.name.is_empty())
                {
                    errors.push(error::Error {
                        debug_message: "172e939d7299dfb37adc7149cfde348d"
                            .to_string(),
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
                    "{}[ParserWarning]{}: WORKING BLIND, Read more: {}https://github.com/behemehal/Ellie-Language/issues/2{}",
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Yellow),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                );
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "=" {
                if !variabledata.data.rtype.is_definer_complete() {
                    errors.push(error::Error {
                        debug_message: "4efcd23df31918facb67a137fe83af8a"
                            .to_string(),
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
                        "{}[ParserWarning]{}: WORKING BLIND, Read more: {}https://github.com/behemehal/Ellie-Language/issues/2{}",
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Yellow),
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                        utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    );
                    variabledata.typed = true;
                }
            } else {
                if variabledata.data.type_pos.range_start.0 == 0 && variabledata.data.type_pos.range_start.1 == 0 && letter_char != " " {
                    variabledata.data.type_pos.range_start = parser.pos;
                }
                processors::definer_processor::collect_definer(
                    &mut variabledata.data.rtype,
                    errors,
                    letter_char.to_string(),
                    parser.pos,
                    next_char,
                    last_char,
                    options,
                );
                variabledata.data.type_pos.range_end = parser.pos;
            }
        } else if letter_char == ";" {
            variabledata.data.value_pos.range_end = parser.pos;
            if let parser::Collecting::Variable(ref mut collected) = parser.current {
                if collected.data.value.is_type_complete() {
                    collected.data.pos.range_end = parser.pos;
                    collected.data.value_pos.range_end = parser.pos;

                    if collected.data.rtype.raw_name() != collected.data.value.get_type() {
                        //We should resolve inner value

                        if collected.data.dynamic {
                            #[cfg(feature = "std")]
                            std::println!(
                                "{}[ParserError]{}: This is a error please report at: {}https://github.com/behemehal/Ellie-Language/issues/new?title=ParserError-{}+Dynamic+Variable+Not+Handled+Correctly&labels=bug,parser{}",
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Red),
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                                collected.data.value.get_type(),
                                utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                            );
                            
                            #[cfg(feature = "std")]
                            std::process::exit(0);
                        }

                        errors.push(error::Error {
                            debug_message: "e406a57e295fad7845e369ab1a618f79"
                                .to_string(),
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
                                        value: collected.data.value.get_type()
                                    },
                                ],
                            ),
                            pos: collected.data.value_pos
                        });
                    }
                    parser.collected.push(parser.current.clone());
                    parser.current = parser::Collecting::None;
                } else {
                    std::println!("{:#?}", collected);
                    errors.push(error::Error {
                        debug_message: "4550f37874175424049c26203f492eb0"
                            .to_string(),
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
            if variabledata.data.value_pos.range_start.0 == 0 && variabledata.data.value_pos.range_start.1 == 0 && letter_char != " " {
                variabledata.data.value_pos.range_start = parser.pos;
            }
            let mut cd = variabledata.clone();
            let collected = processors::value_processor::collect_value(
                &mut cd,
                letter_char,
                next_char,
                last_char,
                parser.pos,
                options,
            );
            for i in collected.errors {
                errors.push(i)
            }
            variabledata.data.value_pos.range_end = parser.pos;
            parser.current = parser::Collecting::Variable(collected.itered_data);
        }
    }
}





