use crate::parser;
use crate::processors::value_processor;
use crate::syntax::{definers, types, variable};
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::defs;
use ellie_core::error;

pub fn collect_new_call(
    parser: parser::Parser,
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    next_char: String,
    last_char: String,
) {
    if let types::Types::ConstructedClass(ref mut new_call_data) = itered_data.data.value {
        if !new_call_data.keyword_collected {
            if new_call_data.keyword_index == 0 && letter_char == "n" {
                new_call_data.data.keyword_pos.range_start = parser.pos.clone();
                new_call_data.keyword_index = 1;
            } else if new_call_data.keyword_index == 1 && letter_char == "e" {
                new_call_data.keyword_index = 2
            } else if new_call_data.keyword_index == 2 && letter_char == "w" {
                new_call_data.keyword_index = 3;
                new_call_data.data.keyword_pos.range_end = parser.pos.clone();
            } else if new_call_data.keyword_index == 3 && letter_char == " " {
                new_call_data.keyword_collected = true;
            } else if (letter_char == " " && new_call_data.keyword_index == 0) || letter_char != " "
            {
                errors.push(error::Error {
                    path: parser.options.path.clone(),
                    scope: "function_call_processor".to_string(),
                    debug_message: "replace_35".to_string(),
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
                        range_end: parser.pos.clone().skip_char(1),
                    },
                });
            }
        } else if !new_call_data.value_collected {
            if new_call_data.data.value.is_type_complete() && letter_char == "(" {
                new_call_data.value_collected = true;
            } else {
                if new_call_data.data.value_pos.is_zero() {
                    new_call_data.data.value_pos.range_start = parser.pos.clone();
                }

                let mut will_be_itered = variable::VariableCollector {
                    data: variable::Variable {
                        value: *new_call_data.data.value.clone(),
                        ..Default::default()
                    },
                    ..Default::default()
                };
                let itered_ncall_vector = Box::new(value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    letter_char,
                    next_char.clone(),
                    last_char,
                ));

                if !itered_ncall_vector.errors.is_empty() {
                    errors.extend(itered_ncall_vector.errors);
                }

                new_call_data.raw_value += letter_char;
                new_call_data.data.value = Box::new(itered_ncall_vector.itered_data.data.value);
                new_call_data.data.value_pos.range_end = parser.pos.clone().skip_char(1);
            }
        } else if !new_call_data.complete {
            let last_entry = new_call_data.data.params.clone().len();
            let is_s_n = last_entry == 0
                || new_call_data.data.params[last_entry - 1]
                    .value
                    .is_type_complete();

            if letter_char == "," && is_s_n && last_entry != 0 {
                if new_call_data.complete {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_string(),
                        debug_message: "ddaa252e0882a65b5843d855c5e48ecc".to_string(),
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
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else if new_call_data.comma {
                    errors.push(error::Error {
                        path: parser.options.path.clone(),
                        scope: "function_call_processor".to_string(),
                        debug_message: "e2162b6b2db3bdfc450b242f00dedb4f".to_string(),
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
                            range_end: parser.pos.clone().skip_char(1),
                        },
                    });
                } else {
                    if last_entry != 0 {
                        new_call_data.data.params[last_entry - 1]
                            .value
                            .make_complete();
                    }
                    new_call_data.comma = true;
                    new_call_data
                        .data
                        .params
                        .push(types::constructed_class::ConstructedClassParameter::default());
                }
            } else if letter_char == ")" && is_s_n {
                if last_entry != 0 {
                    new_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }

                let resolved_new_call = parser.resolve_new_call(new_call_data.clone());
                if let Err(e) = resolved_new_call {
                    errors.extend(e);
                } else if let Ok(e) = resolved_new_call {
                    std::println!("![ParserError] Working blind: {:#?}", e);
                }

                /*
                let fn_exists = parser.resolve_function_call(new_call_data.clone());
                if let Some(type_errors) = fn_exists {
                    for error in type_errors {
                        errors.push(error);
                    }
                }
                */
                new_call_data.complete = true;
            } else {
                if letter_char != " " {
                    //TODO IS THIS SAFE ?
                    new_call_data.comma = false;
                }

                //TODO FIX THIS with function after resolving complete
                let mut will_be_itered: variable::VariableCollector;
                if let definers::DefinerCollecting::Cloak(cloak_data) =
                    itered_data.data.rtype.clone()
                {
                    will_be_itered = if new_call_data.data.params.is_empty() {
                        variable::VariableCollector {
                            data: variable::Variable {
                                rtype: cloak_data.rtype[0].clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: new_call_data.data.params
                                    [new_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                rtype: cloak_data.rtype[new_call_data.data.params.len() - 1]
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                } else {
                    will_be_itered = if new_call_data.data.params.is_empty() {
                        variable::VariableCollector::default()
                    } else {
                        variable::VariableCollector {
                            data: variable::Variable {
                                value: new_call_data.data.params
                                    [new_call_data.data.params.len() - 1]
                                    .value
                                    .clone(),
                                ..Default::default()
                            },
                            ..variable::VariableCollector::default()
                        }
                    };
                }

                let itered_fcall_vector = Box::new(value_processor::collect_value(
                    parser.clone(),
                    &mut will_be_itered,
                    letter_char,
                    next_char,
                    last_char,
                ));

                let itered_entry = match itered_fcall_vector.itered_data.data.value {
                    types::Types::Integer(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Integer(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Float(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Float(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Operator(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Operator(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Bool(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Bool(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::String(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::String(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Char(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Char(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Collective(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Collective(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Reference(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Reference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::BraceReference(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::BraceReference(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Negative(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Negative(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Array(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Array(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Cloak(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::Cloak(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ArrowFunction(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::ArrowFunction(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::FunctionCall(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::FunctionCall(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::ConstructedClass(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::ConstructedClass(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Void => types::constructed_class::ConstructedClassParameter {
                        value: types::Types::Void,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            new_call_data.data.params[last_entry - 1].pos
                        },
                    },
                    types::Types::VariableType(match_data) => {
                        types::constructed_class::ConstructedClassParameter {
                            value: types::Types::VariableType(match_data),
                            pos: if last_entry == 0 {
                                defs::Cursor::default()
                            } else {
                                new_call_data.data.params[last_entry - 1].pos
                            },
                        }
                    }
                    types::Types::Null => types::constructed_class::ConstructedClassParameter {
                        value: types::Types::Null,
                        pos: if last_entry == 0 {
                            defs::Cursor::default()
                        } else {
                            new_call_data.data.params[last_entry - 1].pos
                        },
                    },
                };

                if !itered_fcall_vector.errors.is_empty() {
                    errors.extend(itered_fcall_vector.errors);
                }
                if new_call_data.data.params.is_empty() {
                    new_call_data.data.params.push(itered_entry);

                    if new_call_data.data.params[0].pos.is_zero() {
                        new_call_data.data.params[0].pos.range_start = parser.pos;
                    }
                    new_call_data.data.params[0].pos.range_end = parser.pos;
                } else {
                    new_call_data.data.params[last_entry - 1] = itered_entry;
                    if new_call_data.data.params[last_entry - 1].pos.is_zero() {
                        new_call_data.data.params[last_entry - 1].pos.range_start = parser.pos;
                    }
                    new_call_data.data.params[last_entry - 1].pos.range_end = parser.pos;
                }
            }
        }
    }
}
