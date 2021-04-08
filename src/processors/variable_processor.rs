use crate::error;
use crate::mapper;
use crate::processors;
use crate::syntax::variable;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectorResponse {
    mapper: mapper::Mapper,
    data: variable::VariableCollector,
}

pub fn collect(
    mapper: &mut mapper::Mapper,
    data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String
) -> CollectorResponse {
    //let mut errors : Vec<error::Error> = Vec::new();

    if !data.named {
        if letter_char == ":" {
            if data.name == "" {
                errors.push(error::Error {
                    debug_message: "Ann".to_string(),
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
                data.named = true;
                //println!("|DEBUG| VARIABLE NAMED: {}", data.name);
            }
        } else if letter_char == ";" && data.dynamic {
            //println!("|DEBUG| DYNAMIC VARIABLE COMPLETE: {}", data.r#type);
            mapper.collected.push(mapper.current.clone());
            mapper.current = mapper::Collecting::None;
        } else if letter_char == "=" {
            if !data.dynamic {
                errors.push(error::Error {
                    debug_message: "Project:Ann".to_string(),
                    title: error::errorList::error_s8.title.clone(),
                    code: error::errorList::error_s8.code,
                    message: error::errorList::error_s8.message.clone(),
                    builded_message: error::errorList::error_s8.message.clone(),
                    pos: mapper::defs::Cursor {
                        range_start: mapper.pos.clone(),
                        range_end: mapper.pos.clone().skipChar(1),
                    },
                });
            } else if data.name == "" {
                errors.push(error::Error {
                    debug_message: "bhemehal".to_string(),
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
                data.named = true;
                println!("|DEBUG| VARIABLE NAMED: {}", data.name);
            }
        } else {
            let current_reliability = crate::utils::reliable_name_range(
                crate::utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );
            if current_reliability.reliable {
                if last_char == " " && data.name != "" {
                    errors.push(error::Error {
                        debug_message: "Ataturk".to_string(),
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
                    data.name = data.name.clone() + &letter_char.clone();
                }
            } else if letter_char != " "
                && (letter_char != ":" || letter_char != "=" || letter_char != ";")
                && (last_char == " " || data.name != "")
            {
                errors.push(error::Error {
                    debug_message: "\\src\\processors\\variable_processors.rs:182:0".to_string(),
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
    } else if !data.typed && !data.dynamic {
        if letter_char == ";" {
            //if (data.dynamic) {}
            mapper.collected.push(mapper.current.clone());
            mapper.current = mapper::Collecting::None;
        } else if letter_char == "=" {
            if data.r#type == "" {
                errors.push(error::Error {
                    debug_message: "\\src\\processors\\variable_processors.rs:211:0".to_string(),
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
                data.typed = true;
            }
        } else {
            let current_reliability = crate::utils::reliable_name_range(
                crate::utils::ReliableNameRanges::VariableName,
                letter_char.to_string(),
            );
            if current_reliability.reliable && (data.r#type == "" || (last_char != " ")) {
                data.r#type = data.r#type.clone() + &letter_char.clone();
            } else if data.r#type != ""
                && (last_char == " " && (letter_char != ":" && letter_char != " "))
            {
                errors.push(error::Error {
                    debug_message: "\\src\\processors\\variable_processors.rs:239:0".to_string(),
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
            mapper.collected.push(mapper.current.clone());
            mapper.current = mapper::Collecting::None;
        } else {
            let mut cd = data.clone();
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

    CollectorResponse {
        mapper: mapper.clone(),
        data: data.clone()
    }
}
