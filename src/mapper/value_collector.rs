use crate::error::errorList;

#[derive(Debug, Clone, PartialEq)]
pub struct CollectorResponse {
    pub itered_data: crate::syntax::variable::VariableCollector,
    pub errors: Vec<crate::error::Error>,
}

pub fn collect(
    itered_data: &mut crate::syntax::variable::VariableCollector,
    letter_char: String,
    next_char: String,
    last_char: String,
    pos: crate::mapper::defs::CursorPosition
) -> CollectorResponse {
    //TODO: Provide Error
    let mut errors: Vec<crate::error::Error> = Vec::new();
    match &mut itered_data.value {
        crate::syntax::types::Types::Number(data) => {
            let isn = letter_char.parse::<usize>().is_ok();
            if (isn) {
                if (data.complete) {
                    errors.push(crate::error::Error {
                        debug_message: "\\src\\mapper\\value_collector.rs:24:0".to_string(),
                        title: crate::error::errorList::error_s1.title.clone(),
                        code: crate::error::errorList::error_s1.code,
                        message: crate::error::errorList::error_s1.message.clone(),
                        builded_message: crate::error::Error::build(
                            crate::error::errorList::error_s1.message.clone(),
                            vec![crate::error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }]
                        ),
                        pos: crate::mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.value = (data.value.to_string() + &letter_char)
                        .parse::<usize>()
                        .unwrap();
                }
            } else {
                if letter_char == "." {
                    // String prototype
                    itered_data.value = crate::syntax::types::Types::Refference(
                        crate::syntax::types::RefferenceType {
                            refference: Box::new(itered_data.value.clone()),
                            on_dot: true,
                            chain: Vec::new(),
                        },
                    );
                } else if letter_char == " " {
                    data.complete = true;
                } else {
                    errors.push(crate::error::Error {
                        debug_message: "\\src\\mapper\\value_collector.rs:59:0".to_string(),
                        title: crate::error::errorList::error_s1.title.clone(),
                        code: crate::error::errorList::error_s1.code,
                        message: crate::error::errorList::error_s1.message.clone(),
                        builded_message: crate::error::Error::build(
                            crate::error::errorList::error_s1.message.clone(),
                            vec![crate::error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }]
                        ),
                        pos: crate::mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            }
            CollectorResponse {itered_data: itered_data.clone(), errors}
        }
        crate::syntax::types::Types::Double(data) => CollectorResponse {itered_data: itered_data.clone(), errors},
        crate::syntax::types::Types::Bool(data) => CollectorResponse {itered_data: itered_data.clone(), errors},
        crate::syntax::types::Types::Dynamic => CollectorResponse {itered_data: itered_data.clone(), errors},
        crate::syntax::types::Types::String(data) => {
            if letter_char == data.quote_type && last_char != "\\" {
                if data.complete {
                    errors.push(crate::error::Error {
                        debug_message: "\\src\\mapper\\value_collector.rs:86:0".to_string(),
                        title: crate::error::errorList::error_s1.title.clone(),
                        code: crate::error::errorList::error_s1.code,
                        message: crate::error::errorList::error_s1.message.clone(),
                        builded_message: crate::error::Error::build(
                            crate::error::errorList::error_s1.message.clone(),
                            vec![crate::error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }]
                        ),
                        pos: crate::mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.complete = true;
                }
                CollectorResponse {itered_data: itered_data.clone(), errors}
            } else if letter_char == "." {
                // String prototype
                itered_data.value =
                    crate::syntax::types::Types::Refference(crate::syntax::types::RefferenceType {
                        refference: Box::new(itered_data.value.clone()),
                        on_dot: true,
                        chain: Vec::new(),
                    });
                    CollectorResponse {itered_data: itered_data.clone(), errors}
            } else if letter_char != "\\" {
                data.value = data.value.clone() + &letter_char;
                CollectorResponse {itered_data: itered_data.clone(), errors}
            } else {
                CollectorResponse {itered_data: itered_data.clone(), errors}
            }
        }
        crate::syntax::types::Types::Collective(data) => CollectorResponse {itered_data: itered_data.clone(), errors},
        crate::syntax::types::Types::Refference(data) => {
            if letter_char == "." {
                if data.on_dot {
                    errors.push(crate::error::Error {
                        debug_message: "\\src\\mapper\\value_collector.rs:127:0".to_string(),
                        title: crate::error::errorList::error_s1.title.clone(),
                        code: crate::error::errorList::error_s1.code,
                        message: crate::error::errorList::error_s1.message.clone(),
                        builded_message: crate::error::Error::build(
                            crate::error::errorList::error_s1.message.clone(),
                            vec![crate::error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char.to_string(),
                            }]
                        ),
                        pos: crate::mapper::defs::Cursor {
                            range_start: pos.clone(),
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else {
                    data.on_dot = true;
                }
            } else if letter_char != " " {
                if (data.on_dot) {
                    data.on_dot = false;
                    data.chain.push(letter_char);
                } else {
                    let chainLastElement = data.chain.len() - 1;
                    data.chain[chainLastElement] = data.chain[chainLastElement].clone() + &letter_char;
                }
            }
            CollectorResponse {itered_data: itered_data.clone(), errors}
        }
        crate::syntax::types::Types::Array => CollectorResponse {itered_data: itered_data.clone(), errors},
        crate::syntax::types::Types::Function => CollectorResponse {itered_data: itered_data.clone(), errors},
        crate::syntax::types::Types::Null => {
            let isn = itered_data.raw_value.parse::<usize>().is_ok();
            if itered_data.raw_value == "" {
                if letter_char == "\"" || letter_char == "'" {
                    itered_data.value =
                        crate::syntax::types::Types::String(crate::syntax::types::StringType {
                            quote_type: letter_char,
                            ..Default::default()
                        })
                } else if (itered_data.raw_value.clone() + &letter_char)
                    .to_string()
                    .parse::<i32>()
                    .is_ok()
                {
                    itered_data.value =
                        crate::syntax::types::Types::Number(crate::syntax::types::NumberType {
                            value: (itered_data.raw_value.clone() + &letter_char)
                                .parse::<usize>()
                                .unwrap(),
                            complete: false,
                        })
                } else if letter_char != " " {
                    itered_data.raw_value += &letter_char;
                }
            } else if letter_char == "[" {
            } else if letter_char == "{" {
            } else if letter_char != " " {
                itered_data.raw_value += &letter_char;
                if next_char == ";" || next_char == " " {
                    if itered_data.raw_value == "false" || itered_data.raw_value == "true" {
                        itered_data.value =
                            crate::syntax::types::Types::Bool(crate::syntax::types::BoolType {
                                value: if itered_data.raw_value == "true" {
                                    true
                                } else {
                                    false
                                },
                            })
                    } else if itered_data.raw_value.parse::<i32>().is_ok() {
                        itered_data.value =
                            crate::syntax::types::Types::Number(crate::syntax::types::NumberType {
                                value: (itered_data.raw_value.clone() + &letter_char)
                                    .parse::<usize>()
                                    .unwrap(),
                                complete: false,
                            })
                    }
                }
            }
            CollectorResponse {itered_data: itered_data.clone(), errors}
        }
    }
}
