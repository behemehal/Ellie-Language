use crate::parser;
use crate::processors;
use crate::syntax::function;
use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_arrow(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::ArrowFunction(ref mut functiondata) = itered_data.data.value {
        itered_data.data.rtype =
            definers::DefinerCollecting::Function(definers::FunctionType::default());

        if !functiondata.parameter_wrote {
            if letter_char == "(" && !functiondata.param_bracket_opened {
                functiondata.param_bracket_opened = true;
            } else {
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
                        && ((last_char != " " && last_char != "\n")
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
                                scope: "arrow_function".to_string(),
                                debug_message: "2c7dccb9dbff02c405315804796d160f".to_string(),
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
                            scope: "arrow_function".to_string(),
                            debug_message: "7b3c80ddb3d8089d9b111749147c9833".to_string(),
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
                        parser.clone(),
                        &mut functiondata.data.parameters[last_entry - 1].data.rtype,
                        errors,
                        letter_char.to_string(),
                        next_char,
                        last_char,
                    );
                }
            }
        } else if !functiondata.pointer_typed {
            if letter_char == ">" {
                functiondata.pointer_typed = true;
            } else if letter_char == "{" {
                functiondata.return_typed = true;
                functiondata.pointer_typed = true;
            } else if letter_char != " " {
                errors.push(error::Error {
                    scope: "arrow_function".to_string(),
                    debug_message: "2cd9a819587f1984d25bd2a4d452e341".to_string(),
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
        } else if !functiondata.return_typed {
            if letter_char == "{" && functiondata.data.return_type.is_definer_complete() {
                functiondata.return_typed = true;
            } else {
                processors::definer_processor::collect_definer(
                    parser.clone(),
                    &mut functiondata.data.return_type,
                    errors,
                    letter_char.to_string(),
                    next_char,
                    last_char,
                );
            }
        } else if letter_char == "}" && functiondata.brace_count == 0 {
            functiondata.complete = true;
        } else {
            if letter_char == "{" {
                functiondata.brace_count += 1;
            } else if letter_char == "}" && functiondata.brace_count != 0 {
                functiondata.brace_count -= 1;
            }

            let code_letter = if last_char.clone() == "\n" || last_char.clone() == "\r" {
                last_char + letter_char //Make sure we get the lines correctly
            } else {
                letter_char.to_string()
            };
            functiondata.code += &code_letter;
        }
    }
}
