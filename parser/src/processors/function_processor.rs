use crate::parser;
use crate::processors;
use crate::syntax::{function, types};
use ellie_core::{defs, error, utils};

use crate::alloc::string::{String, ToString};
use crate::alloc::vec;
use crate::alloc::vec::Vec;

pub fn collect_function(
    parser: &mut parser::Parser,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
    options: defs::ParserOptions,
) {
    if let parser::Collecting::Function(ref mut functiondata) = parser.current {
        if !functiondata.initialized {
            if last_char == " " && letter_char != " " {
                functiondata.initialized = true;
                functiondata.name_pos.range_start.0 = parser.pos.0; //Function naming started so we set the position
                functiondata.name_pos.range_start.1 = parser.pos.1; //Function naming started so we set the position
                functiondata.data.name += letter_char;
            }
        } else if !functiondata.named {
            if letter_char == "(" {
                functiondata.name_pos.range_end.0 = parser.pos.0; // function naming ended
                functiondata.name_pos.range_end.1 = parser.pos.1; // function naming ended
                functiondata.parameter_bracket_start_pos.range_start.0 = parser.pos.0; //parameter start
                functiondata.parameter_bracket_start_pos.range_start.1 = parser.pos.1; //parameter start
                functiondata.parameter_bracket_start_pos.range_end.0 = parser.pos.skipChar(1).0; //parameter start
                functiondata.parameter_bracket_start_pos.range_end.1 = parser.pos.skipChar(1).1; //parameter start
                functiondata.named = true;
            } else if last_char == " " && letter_char != " " && !functiondata.data.name.is_empty() {
                errors.push(error::Error {
                    debug_message: "523964763834146856134fb129b14410".to_string(),
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
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if last_char == " " && !functiondata.data.name.is_empty() {
                        errors.push(error::Error {
                            debug_message: "132ead0d605afdd8046af83b838aa8ec"
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
                        functiondata.data.name += letter_char;
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "fe9e93b7fb468c82eefcea4468bf2185"
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
                //user naming the function
            }
        } else if !functiondata.parameter_wrote {
            let mut last_entry = functiondata.data.parameters.len();
            let typing_name = if last_entry == 0 {
                true
            } else {
                !functiondata.data.parameters[last_entry - 1].named
            };

            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if typing_name {
                if current_reliability.reliable
                    && (last_char != " "
                        || last_entry == 0
                        || functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty())
                {
                    if last_entry == 0 {
                        functiondata
                            .data
                            .parameters
                            .push(function::FunctionParameterCollector::default());
                        last_entry = 1;
                    }
                    functiondata.data.parameters[last_entry - 1].data.name += letter_char
                } else if letter_char == ":" {
                    if last_entry == 0
                        || functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message:
                                "./parser/src/processors/type_processors/arrow_function.rs:59"
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
                        functiondata.data.parameters[last_entry - 1].named = true;
                    }
                } else if letter_char == ")" && last_entry == 0 {
                    functiondata.parameter_wrote = true;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message:
                            "./parser/src/processors/type_processors/arrow_function.rs:82"
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
            } else if letter_char == ")"
                && (last_entry == 0
                    || functiondata.data.parameters[last_entry - 1].child_brace == 0)
            {
                functiondata.parameter_wrote = true;
            } else if letter_char == ","
                && functiondata.data.parameters[last_entry - 1]
                    .data
                    .rtype
                    .is_definer_complete()
            {
                //If its type's comma dont stop collecting it
                functiondata
                    .data
                    .parameters
                    .push(function::FunctionParameterCollector::default());
            } else {
                if letter_char == ")" {
                    functiondata.data.parameters[last_entry - 1].child_brace -= 1;
                } else if letter_char == "(" {
                    functiondata.data.parameters[last_entry - 1].child_brace += 1;
                }
                processors::definer_processor::collect_definer(
                    &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                    errors,
                    letter_char.to_string(),
                    parser.pos,
                    next_char,
                    last_char,
                    options,
                );
            }
        } else if !functiondata.return_typed {
            if letter_char == "{" {
                //Skipped return type it's void
                functiondata.return_typed = true;
                functiondata.data.return_type = types::Types::Void;
                functiondata.inside_code_wrote = true;
                functiondata.code_bracket_start.range_start.0 = parser.pos.0; //Bracket start
                functiondata.code_bracket_start.range_start.1 = parser.pos.1;
            //Bracket start
            } else if !functiondata.pointer_typed {
                if letter_char == ">" {
                    functiondata.pointer_typed = true
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "7f69ab1b2f744e9235a2c14021e31cef"
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
            } else if functiondata.pointer_typed && !functiondata.return_typed {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );

                if current_reliability.reliable {
                    if last_char == " " && !functiondata.return_type_text.is_empty() {
                        errors.push(error::Error {
                            debug_message: "faa50e4b797dd0791a9a8a9413f8f815"
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
                        functiondata.return_type_text += letter_char;
                    }
                } else if letter_char == "{" {
                    if functiondata.return_type_text.is_empty() {
                        errors.push(error::Error {
                            debug_message: "0017aef4c43e39f2094735016dd39134"
                                .to_string(),
                            title: error::errorList::error_s8.title.clone(),
                            code: error::errorList::error_s8.code,
                            message: error::errorList::error_s8.message.clone(),
                            builded_message: error::BuildedError::default(),
                            pos: defs::Cursor {
                                range_start: parser.pos,
                                range_end: parser.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        functiondata.return_typed = true;
                        functiondata.data.return_type = types::Types::Void;
                        functiondata.inside_code_wrote = true;
                        functiondata.code_bracket_start.range_start.0 = parser.pos.0; //Bracket start
                        functiondata.code_bracket_start.range_start.1 = parser.pos.1;
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "129918cad52d9d423e115551b4624931"
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
            } else if letter_char != " " {
                errors.push(error::Error {
                    debug_message: "161d86e9e29a8cc8d16f97732b59d3f4".to_string(),
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
        } else if letter_char == "{" {
            functiondata.inside_object_start = true;
            functiondata.inside_object_count += 1;
        } else if letter_char == "}" {
            if functiondata.inside_object_start {
                if functiondata.inside_object_count == 0 {
                    functiondata.inside_object_start = true;
                } else {
                    functiondata.inside_object_count -= 1;
                }
            } else {
                let child_parser =
                    parser::Parser::new(functiondata.inside_code_string.clone(), options);
                parser.pos = child_parser.pos;
                let mapped = child_parser.map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                functiondata.data.inside_code = mapped.items;
                parser.collected.push(parser.current.clone());
                parser.current = parser::Collecting::None;
            }
        } else {
            functiondata.inside_code_string += letter_char;
        }
    }
}





