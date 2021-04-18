use crate::error;
use crate::mapper;
use crate::processors;
use crate::syntax::variable;
use crate::utils;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectorResponse {
    mapper: mapper::Mapper,
    data: variable::VariableCollector,
}

pub fn collect(
    mapper: &mut mapper::Mapper,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let mapper::Collecting::Variable(ref mut variabledata) = mapper.current {
        if !variabledata.named {
            if letter_char == ":" {
                if variabledata.data.name == "" {
                    errors.push(error::Error {
                        debug_message: "Redaktik".to_string(),
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
                            range_start: mapper.pos.clone(),
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    if variabledata.data.dynamic {
                        //TODO REMOVE THIS
                        errors.push(error::Error {
                            debug_message: "Ethicl".to_string(),
                            title: error::errorList::error_s11.title.clone(),
                            code: error::errorList::error_s11.code,
                            message: error::errorList::error_s11.message.clone(),
                            builded_message: error::errorList::error_s11.message.clone(),
                            pos: mapper::defs::Cursor {
                                range_start: mapper.pos.clone().skipChar(1),
                                range_end: mapper.pos.clone().skipChar(2),
                            },
                        });
                    }
                    variabledata.named = true;
                    //println!("|DEBUG| VARIABLE NAMED: {}", data.name);
                }
            } else if letter_char == ";" && variabledata.data.dynamic {
                mapper.collected.push(mapper.current.clone());
                mapper.current = mapper::Collecting::None;
            } else if letter_char == "=" {
                if !variabledata.data.dynamic {
                    errors.push(error::Error {
                        debug_message: "Ertsalik".to_string(),
                        title: error::errorList::error_s8.title.clone(),
                        code: error::errorList::error_s8.code,
                        message: error::errorList::error_s8.message.clone(),
                        builded_message: error::errorList::error_s8.message.clone(),
                        pos: mapper::defs::Cursor {
                            range_start: mapper.pos.clone(),
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                } else if variabledata.data.name == "" {
                    errors.push(error::Error {
                        debug_message: "\\src\\mapper\\mod.rs:133:0".to_string(),
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
                            range_start: mapper.pos.clone(),
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    variabledata.named = true;
                    println!("|DEBUG| VARIABLE NAMED: {}", variabledata.data.name);
                }
            } else {
                let current_reliability = crate::utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    if last_char == " " && variabledata.data.name != "" {
                        errors.push(error::Error {
                            debug_message: "\\src\\mapper\\mod.rs:161:0".to_string(),
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
                                range_start: mapper.pos.clone(),
                                range_end: mapper.pos.clone().skipChar(1),
                            },
                        });
                    } else {
                        variabledata.data.name =
                            variabledata.data.name.clone() + &letter_char.clone();
                    }
                } else if letter_char != " "
                    && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                    && (last_char == " " || variabledata.data.name != "")
                {
                    errors.push(error::Error {
                        debug_message: "Ahencam".to_string(),
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
                            range_start: mapper.pos.clone(),
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                }
            }
        } else if !variabledata.typed && !variabledata.data.dynamic {
            if letter_char == ";" {
                if variabledata.data.dynamic {}
                mapper.collected.push(mapper.current.clone());
                mapper.current = mapper::Collecting::None;
            } else if letter_char == "=" {
                if variabledata.r#type == "" {
                    errors.push(error::Error {
                        debug_message: "Odio".to_string(),
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
                            range_start: mapper.pos.clone(),
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                } else {
                    variabledata.typed = true;
                }
            } else {
                let current_reliability = crate::utils::reliable_name_range(
                    crate::utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable && (variabledata.r#type == "" || last_char != " ") {
                    variabledata.r#type = variabledata.r#type.clone() + &letter_char.clone();
                } else if variabledata.r#type != ""
                    && (last_char == " " && (letter_char != ":" && letter_char != " "))
                {
                    errors.push(error::Error {
                        debug_message: "ThnicLimts".to_string(),
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
                            range_start: mapper.pos.clone(),
                            range_end: mapper.pos.clone().skipChar(1),
                        },
                    });
                }
            }
        } else {
            if letter_char == ";" {
                if let mapper::Collecting::Variable(collected) = mapper.current.clone() {
                    println!("{:#?}", collected.data.value);
                    if collected.data.value.is_complete() {
                        mapper.collected.push(mapper.current.clone());
                        mapper.current = mapper::Collecting::None;
                    } else {
                        errors.push(error::Error {
                            debug_message: "Protocol".to_string(),
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
                                range_start: mapper.pos.clone().skipChar(1),
                                range_end: mapper.pos.clone().skipChar(2),
                            },
                        });
                    }
                }
            } else {
                let mut cd = variabledata.clone();
                let collected = processors::value_processor::collect(
                    &mut cd,
                    letter_char,
                    next_char.to_string(),
                    last_char.to_string(),
                    mapper.pos.clone(),
                );
                for i in collected.errors {
                    errors.push(i)
                }
                mapper.current = mapper::Collecting::Variable(collected.itered_data);
            }
        }
    }
}
