use crate::error;
use crate::mapper;
use crate::syntax::{function, types};
use crate::utils;

pub fn collect(
    mapper: &mut mapper::Mapper,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
) {
    if let mapper::Collecting::Function(ref mut functiondata) = mapper.current {
        if !functiondata.initialized {
            if last_char == " " && letter_char != " " {
                functiondata.initialized = true;
                functiondata.name_pos.range_start.0 = mapper.pos.0; //Function naming started so we set the position
                functiondata.name_pos.range_start.1 = mapper.pos.1; //Function naming started so we set the position
                functiondata.data.name += letter_char;
            }
        } else if !functiondata.named {
            if letter_char == "(" {
                functiondata.name_pos.range_end.0 = mapper.pos.0; // function naming ended
                functiondata.name_pos.range_end.1 = mapper.pos.1; // function naming ended
                functiondata.parameter_bracket_start_pos.range_start.0 = mapper.pos.0; //parameter start
                functiondata.parameter_bracket_start_pos.range_start.1 = mapper.pos.1; //parameter start
                functiondata.parameter_bracket_start_pos.range_end.0 = mapper.pos.skipChar(1).0; //parameter start
                functiondata.parameter_bracket_start_pos.range_end.1 = mapper.pos.skipChar(1).1; //parameter start
                functiondata.named = true;
            } else if last_char == " " && letter_char != " " && !functiondata.data.name.is_empty() {
                errors.push(error::Error {
                    debug_message: "Entropy".to_string(),
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
                    pos: mapper::defs::Cursor {
                        range_start: mapper.pos,
                        range_end: mapper.pos.clone().skipChar(1),
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
                            debug_message: "Physicist".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos,
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        functiondata.data.name += letter_char;
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "AntiLine".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: mapper.pos,
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                }
                //user naming the function
            }
        } else if !functiondata.parameter_wrote {
            let last_entry = functiondata.data.parameters.len();
            let typing_name = if last_entry == 0 {
                true
            } else {
                !functiondata.data.parameters[last_entry - 1].named
            };

            let colon_expected = if last_entry == 0 {
                false
            } else {
                functiondata.data.parameters[last_entry - 1].colon_expected
            };

            let typing = if last_entry == 0 {
                false
            } else {
                functiondata.data.parameters[last_entry - 1].named
                    && !functiondata.data.parameters[last_entry - 1].colon_expected
            };

            let current_reliability = utils::reliable_name_range(
                utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );

            if last_entry == 0 && letter_char == ")" {
                functiondata.parameter_wrote = true;
            } else if typing_name {
                if current_reliability.reliable {
                    if last_entry == 0 {
                        functiondata
                            .data
                            .parameters
                            .push(function::FunctionParameterCollector::default())
                    } else if last_char == " "
                        && !functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "Irenar".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos,
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        if functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty()
                        {
                            functiondata.data.parameters[last_entry - 1]
                                .name_pos
                                .range_start = mapper.pos;
                        }
                        functiondata.data.parameters[last_entry - 1].data.name += letter_char;
                        functiondata.data.parameters[last_entry - 1]
                            .name_pos
                            .range_end = mapper.pos;
                    }
                } else if letter_char == ":" {
                    functiondata.data.parameters[last_entry - 1].named = true;
                } else if letter_char == " " {
                    if last_entry != 0
                        && !functiondata.data.parameters[last_entry - 1]
                            .data
                            .name
                            .is_empty()
                    {
                        functiondata.data.parameters[last_entry - 1].named = true;
                        functiondata.data.parameters[last_entry - 1].colon_expected = true;
                    }
                } else {
                    errors.push(error::Error {
                        debug_message: "Quadro".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: mapper.pos,
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                }
            } else if colon_expected {
                if letter_char == ":" {
                    functiondata.data.parameters[last_entry - 1].colon_expected = false;
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "Nucleic".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: mapper.pos,
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                }
            } else if typing {
                if current_reliability.reliable {
                    if last_char == " "
                        && !functiondata.data.parameters[last_entry - 1]
                            .type_text
                            .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "Estetik".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos,
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    } else if functiondata.data.parameters[last_entry - 1]
                        .type_text
                        .is_empty()
                    {
                        functiondata.data.parameters[last_entry - 1]
                            .type_pos
                            .range_start = mapper.pos;
                    }
                    functiondata.data.parameters[last_entry - 1]
                        .type_pos
                        .range_end = mapper.pos;
                    functiondata.data.parameters[last_entry - 1].type_text += letter_char;
                } else if letter_char == "," {
                    let parameter_names = functiondata
                        .data
                        .parameters
                        .iter()
                        .map(|x| x.data.name.clone());
                    let mut parameter_names_deduped = functiondata
                        .data
                        .parameters
                        .iter()
                        .map(|x| x.data.name.clone())
                        .collect::<Vec<String>>();
                    parameter_names_deduped.dedup();

                    if parameter_names.len() != parameter_names_deduped.len() {
                        errors.push(error::Error {
                            debug_message: "Artiyik".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::errorList::error_s10.message.clone(),
                            pos: mapper::defs::Cursor {
                                range_start: functiondata.data.parameters
                                    [functiondata.data.parameters.len() - 1]
                                    .name_pos
                                    .range_start,
                                range_end: functiondata.data.parameters
                                    [functiondata.data.parameters.len() - 1]
                                    .type_pos
                                    .range_end,
                            },
                        });
                    }
                    functiondata
                        .data
                        .parameters
                        .push(function::FunctionParameterCollector::default());
                } else if letter_char == ")" {
                    let parameter_names = functiondata
                        .data
                        .parameters
                        .iter()
                        .map(|x| x.data.name.clone());
                    let mut parameter_names_deduped = functiondata
                        .data
                        .parameters
                        .iter()
                        .map(|x| x.data.name.clone())
                        .collect::<Vec<String>>();
                    parameter_names_deduped.dedup();

                    if parameter_names.len() != parameter_names_deduped.len() {
                        errors.push(error::Error {
                            debug_message: "Selranda".to_string(),
                            title: error::errorList::error_s10.title.clone(),
                            code: error::errorList::error_s10.code,
                            message: error::errorList::error_s10.message.clone(),
                            builded_message: error::errorList::error_s10.message.clone(),
                            pos: mapper::defs::Cursor {
                                range_start: functiondata.data.parameters
                                    [functiondata.data.parameters.len() - 1]
                                    .name_pos
                                    .range_start
                                    .clone()
                                    .popChar(1),
                                range_end: functiondata.data.parameters
                                    [functiondata.data.parameters.len() - 1]
                                    .type_pos
                                    .range_end
                                    .clone()
                                    .skipChar(1),
                            },
                        });
                    } else if functiondata.data.parameters[functiondata.data.parameters.len() - 1]
                        .type_text
                        .is_empty()
                    {
                        errors.push(error::Error {
                            debug_message: "Elliead".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos,
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    }
                    functiondata.parameter_wrote = true;
                }
            }
        } else if !functiondata.return_typed {
            if letter_char == "{" {
                //Skipped return type it's void
                functiondata.return_typed = true;
                functiondata.data.return_type = types::Types::Void;
                functiondata.inside_code_wrote = true;
                functiondata.code_bracket_start.range_start.0 = mapper.pos.0; //Bracket start
                functiondata.code_bracket_start.range_start.1 = mapper.pos.1;
            //Bracket start
            } else if !functiondata.pointer_typed {
                if letter_char == ">" {
                    functiondata.pointer_typed = true
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "Elase".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: mapper.pos,
                            range_end: mapper.pos.clone().skipChar(1),
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
                            debug_message: "Ellie".to_string(),
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
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos,
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        functiondata.return_type_text += letter_char;
                    }
                } else if letter_char == "{" {
                    if functiondata.return_type_text.is_empty() {
                        errors.push(error::Error {
                            debug_message: "Aesthetics".to_string(),
                            title: error::errorList::error_s8.title.clone(),
                            code: error::errorList::error_s8.code,
                            message: error::errorList::error_s8.message.clone(),
                            builded_message: error::errorList::error_s8.message.clone(),
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos,
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        functiondata.return_typed = true;
                        functiondata.data.return_type = types::Types::Void;
                        functiondata.inside_code_wrote = true;
                        functiondata.code_bracket_start.range_start.0 = mapper.pos.0; //Bracket start
                        functiondata.code_bracket_start.range_start.1 = mapper.pos.1;
                    }
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "Elsa".to_string(),
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
                        pos: mapper::defs::Cursor {
                            range_start: mapper.pos,
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                }
            } else if letter_char != " " {
                errors.push(error::Error {
                    debug_message: "Aghtcnm".to_string(),
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
                    pos: mapper::defs::Cursor {
                        range_start: mapper.pos,
                        range_end: mapper.pos.clone().skipChar(1),
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
                let child_mapper = mapper::Mapper::new(
                    functiondata.inside_code_string.clone(),
                    mapper::defs::MapperOptions {
                        functions: true,
                        break_on_error: false,
                        loops: true,
                        global_variables: true,
                        collectives: true,
                        variables: true,
                    },
                );
                mapper.pos = child_mapper.pos;
                let mapped = child_mapper.Map();
                for i in mapped.syntax_errors {
                    errors.push(i)
                }
                functiondata.data.inside_code = mapped.items;
                mapper.collected.push(mapper.current.clone());
                mapper.current = mapper::Collecting::None;
            }
        } else {
            functiondata.inside_code_string += letter_char;
        }
    }
}
