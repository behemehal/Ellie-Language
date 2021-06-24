use crate::parser;
use crate::syntax::constructor;
use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_constructor(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Constructor(ref mut constructordata) = parser.current {
        if !constructordata.named {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if current_reliability.reliable {
                if last_char == " " {
                    //class name is now typing
                    constructordata.data.name_pos.range_start = parser.pos;
                }
                constructordata.data.name_pos.range_end = parser.pos;
                constructordata.data.name += letter_char;
            } else if letter_char == " " && !constructordata.data.name.is_empty() {
                constructordata.named = true;
            } else if letter_char == "(" && !constructordata.data.name.is_empty() {
                constructordata.named = true;
                constructordata.parameter_brace_open = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    debug_message: "".to_string(),
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
        } else if !constructordata.parameter_wrote {
            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if constructordata.parameter_brace_open {
                let mut last_entry = constructordata.data.parameters.len();

                if current_reliability.reliable
                    && (last_char != " "
                        || last_entry == 0
                        || constructordata.data.parameters[last_entry - 1]
                            .name
                            .is_empty())
                {
                    if last_entry == 0 {
                        constructordata
                            .data
                            .parameters
                            .push(constructor::ConstructorParameter {
                                pos: defs::Cursor {
                                    range_start: parser.pos.clone().popChar(1),
                                    range_end: parser.pos.clone().skipChar(1),
                                },
                                ..Default::default()
                            });
                        last_entry = 1;
                    }

                    if constructordata.data.parameters[last_entry - 1]
                        .name
                        .is_empty()
                    {
                        constructordata.data.parameters[last_entry - 1]
                            .pos
                            .range_start = parser.pos;
                    };
                    constructordata.data.parameters[last_entry - 1]
                        .pos
                        .range_end = parser.pos.clone().skipChar(1);
                    constructordata.data.parameters[last_entry - 1].name += letter_char
                } else if letter_char == ")" {
                    if last_entry == 0
                        || constructordata.data.parameters[last_entry - 1]
                            .name
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "".to_string(),
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
                        constructordata.parameter_brace_open = false;
                        constructordata.parameter_wrote = true;
                    }
                } else if letter_char == "," {
                    if last_entry == 0
                        || constructordata.data.parameters[last_entry - 1]
                            .name
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "".to_string(),
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
                        constructordata
                            .data
                            .parameters
                            .push(constructor::ConstructorParameter {
                                pos: defs::Cursor {
                                    range_start: parser.pos,
                                    ..Default::default()
                                },
                                ..Default::default()
                            });
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "".to_string(),
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
            } else if letter_char == "(" {
                constructordata.parameter_brace_open = true;
            } else if letter_char == "{" {
                #[cfg(feature = "std")]
                std::println!(
                    "{}[ParserError]{}: Constructor body is not supported yet: {}https://github.com/behemehal/Ellie-Language/issues/{}",
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Red),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Cyan),
                    utils::terminal_colors::get_color(utils::terminal_colors::Colors::Reset),
                );

                #[cfg(feature = "std")]
                std::process::exit(0);
                constructordata.parameter_wrote = true;
            } else {
                errors.push(error::Error {
                    debug_message: "".to_string(),
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
        } else if !constructordata.inside_code_wrote {
            if letter_char == ";"
                && !constructordata.code_brace_open
                && constructordata.brace_count == 0
            {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "}" && constructordata.brace_count == 0 {
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            } else if letter_char == "{" && !constructordata.code_brace_open {
                constructordata.code_brace_open = true;
            } else {
                if letter_char == "{" {
                    if constructordata.collecting_code {
                        constructordata.brace_count += 1;
                    } else {
                        constructordata.collecting_code = true;
                    }
                } else if letter_char == "}" && constructordata.brace_count > 0 {
                    constructordata.brace_count -= 1;
                }
                constructordata.inside_code_string += letter_char;
                let mut child_parser =
                    parser::Parser::new(constructordata.inside_code_string.clone(), options);
                child_parser.pos = parser.pos;
                let mapped = child_parser.map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                constructordata.data.inside_code = mapped.items;
            }
        }
    }
}
