use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::syntax;
use crate::syntax::r#type::Collecting;
use ellie_core::{defs, error, utils};

/*
i8                    //generic type
array(array(i8), 5)   //i8 5 sized array
fn(i16, i32)::i8      //a function that takes i16 and i32 as parameter and returns i8 as result
cloak(i8, i32)        //a cloak that contains i8 as first parameter i32 as second

*/

pub fn collect(
    type_data: &mut Collecting,
    errors: &mut Vec<error::Error>,
    letter_char: String,
    pos: defs::CursorPosition,
    next_char: String,
    last_char: String,
) {
    match type_data {
        Collecting::Array(ref mut data) => {
            if !data.typed {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
                } else if letter_char == "," {
                    data.typed = true;
                } else {
                    collect(
                        &mut data.r#type,
                        errors,
                        letter_char,
                        pos,
                        next_char,
                        last_char,
                    )
                }
            } else {
                let mut emulated_collector_data = syntax::variable::VariableCollector {
                    data: syntax::variable::Variable {
                        value: data.len.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };

                let processed_data = crate::processors::value_processor::collect(
                    &mut emulated_collector_data,
                    &letter_char,
                    next_char,
                    last_char,
                    pos,
                );
                for i in processed_data.errors {
                    errors.push(i)
                }
                
                if emulated_collector_data.data.value.is_complete() {
                    data.complete = true;
                }
                
                data.len = emulated_collector_data.data.value;
            }
        }
        Collecting::Generic(data) => {
            if data.r#type.trim() == "fn" {
                *type_data = Collecting::Function(syntax::r#type::FunctionType {
                    bracket_inserted: letter_char == "(",
                    params: if letter_char == "(" {
                        vec![Collecting::Generic(syntax::r#type::GenericType::default())]
                    } else {
                        vec![]
                    },
                    ..Default::default()
                });
            } else if letter_char == "(" && data.r#type == "array" {
                *type_data = Collecting::Array(syntax::r#type::ArrayType {
                    bracket_inserted: letter_char == "(",
                    ..Default::default()
                });
            } else if letter_char != " " && last_char == " " && data.r#type.trim() != "" {
                errors.push(error::Error {
                    debug_message: "./parser/src/processors/type_check_processor.rs:84".to_string(),
                    title: error::errorList::error_s1.title.clone(),
                    code: error::errorList::error_s1.code,
                    message: error::errorList::error_s1.message.clone(),
                    builded_message: error::Error::build(
                        error::errorList::error_s1.message.clone(),
                        vec![error::ErrorBuildField {
                            key: "token".to_string(),
                            value: letter_char,
                        }],
                    ),
                    pos: defs::Cursor {
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                let current_reliability = utils::reliable_name_range(
                    utils::ReliableNameRanges::VariableName,
                    letter_char.to_string(),
                );
                if current_reliability.reliable {
                    data.r#type += &letter_char;
                    data.r#type = utils::trim_good(data.r#type.trim().to_string());
                } else if letter_char != " " {
                    errors.push(error::Error {
                        debug_message: "./parser/src/processors/type_check_processor.rs:110".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char,
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            }
        }
        Collecting::Function(data) => {
            if !data.parameter_collected {
                if letter_char == "(" && !data.bracket_inserted {
                    data.bracket_inserted = true;
                    data.params
                        .push(Collecting::Generic(syntax::r#type::GenericType::default()));
                } else if letter_char == ")" && data.bracket_inserted {
                    data.parameter_collected = true;
                } else if letter_char == "," && !data.params.is_empty() && !data.at_comma {
                    data.params
                        .push(Collecting::Generic(syntax::r#type::GenericType::default()));
                    data.at_comma = true;
                } else if data.params.is_empty() && data.bracket_inserted {
                    //This should have been filled If everything were right
                    errors.push(error::Error {
                        debug_message: "./parser/src/processors/type_check_processor.rs:144".to_string(),
                        title: error::errorList::error_s1.title.clone(),
                        code: error::errorList::error_s1.code,
                        message: error::errorList::error_s1.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s1.message.clone(),
                            vec![error::ErrorBuildField {
                                key: "token".to_string(),
                                value: letter_char,
                            }],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                } else if data.bracket_inserted {
                    data.at_comma = false;
                    let len = data.params.clone().len();
                    collect(
                        &mut data.params[if len == 0 { 0 } else { len - 1 }],
                        errors,
                        letter_char,
                        pos,
                        next_char,
                        last_char,
                    );

                    if matches!(data.params[if len == 0 { 0 } else { len - 1 }].clone(), crate::syntax::r#type::Collecting::Array(x) if x.complete) || matches!(data.params[if len == 0 { 0 } else { len - 1 }].clone(), crate::syntax::r#type::Collecting::Function(x) if x.complete) || matches!(data.params[if len == 0 { 0 } else { len - 1 }].clone(), crate::syntax::r#type::Collecting::Cloak(x) if x.complete) || matches!(data.params[if len == 0 { 0 } else { len - 1 }].clone(), crate::syntax::r#type::Collecting::Generic(_)) {
                        data.complete = true;
                    }
                }
            } else if !data.return_typed {
                if data.return_keyword != 2 {
                    if letter_char != ":" {
                        errors.push(error::Error {
                            debug_message: "./parser/src/processors/type_check_processor.rs:180".to_string(),
                            title: error::errorList::error_s1.title.clone(),
                            code: error::errorList::error_s1.code,
                            message: error::errorList::error_s1.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s1.message.clone(),
                                vec![error::ErrorBuildField {
                                    key: "token".to_string(),
                                    value: letter_char,
                                }],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                    data.return_keyword += 1;
                } else {
                    data.complete = true;
                    collect(
                        &mut data.returning,
                        errors,
                        letter_char,
                        pos,
                        next_char,
                        last_char,
                    )
                }
            }
        }
        Collecting::Cloak(_) => {}
    }
}

