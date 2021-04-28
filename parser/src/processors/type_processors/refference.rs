use crate::syntax::{types, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Refference(ref mut data) = itered_data.data.value {
        if letter_char == "." {
            if data.on_dot {
                errors.push(error::Error {
                    debug_message: "Yugirmnoa".to_string(),
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
                        range_start: pos,
                        range_end: pos.clone().skipChar(1),
                    },
                });
            } else {
                data.on_dot = true;
            }
        } else if letter_char != " " {
            if data.on_dot {
                data.on_dot = false;
            }
            if last_char == " "
                && !data.chain.is_empty()
                && !data.chain[data.chain.len() - 1].is_empty()
            {
                if utils::is_opearators(letter_char) {
                    //itered_data.data.value = types::Types::Operators(types::OperatorType {
                    //    first: Box::new(itered_data.data.value.clone()),
                    //    operator_collect: letter_char.to_string(),
                    //    collecting_operator: true,
                    //    ..Default::default()
                    //});
                } else {
                    errors.push(error::Error {
                        debug_message: "Fsteasthialvi".to_string(),
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
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }
            } else {
                let is_num = letter_char.parse::<i8>().is_ok();

                if is_num {
                    if data.chain.is_empty() {
                        if let types::Types::Number(refference_value) = &*data.refference {
                            //It's a f32 or f64

                            if itered_data.r#type.data.name == "f32" {
                                let double_parse =
                                    (refference_value.raw.clone() + "." + letter_char)
                                        .parse::<f32>();
                                if let Ok(parsed_double) = double_parse {
                                    if parsed_double.is_infinite() {
                                        errors.push(error::Error {
                                            debug_message: "InfinityAndBeyond".to_string(),
                                            title: error::errorList::error_s17.title.clone(),
                                            code: error::errorList::error_s17.code,
                                            message: error::errorList::error_s17.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s17.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    },
                                                ],
                                            ),
                                            pos: defs::Cursor {
                                                range_start: pos.clone().popChar((refference_value.raw.clone() + "." + letter_char).len() as i64),
                                                range_end: pos.clone().skipChar(1),
                                            },
                                        });
                                    } else {
                                        itered_data.data.value =
                                            types::Types::Number(types::number_type::NumberType {
                                                value: types::number_type::NumberSize::F32(
                                                    parsed_double,
                                                ),
                                                raw: (refference_value.raw.clone()
                                                    + "."
                                                    + letter_char),
                                                r#type: types::number_type::NumberTypes::F32,
                                                complete: false,
                                            })
                                    }
                                } else {
                                    errors.push(error::Error {
                                        debug_message: "Onion".to_string(),
                                        title: error::errorList::error_s16.title.clone(),
                                        code: error::errorList::error_s16.code,
                                        message: error::errorList::error_s16.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s16.message.clone(),
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "val".to_string(),
                                                    value: (refference_value.raw.clone()
                                                        + letter_char),
                                                },
                                                error::ErrorBuildField {
                                                    key: "type".to_string(),
                                                    value: "f32".to_string(),
                                                },
                                            ],
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                }
                            } else if itered_data.r#type.data.name == "f64" {
                                let double_parse =
                                    (refference_value.raw.clone() + "." + letter_char)
                                        .parse::<f64>();
                                if let Ok(parsed_double) = double_parse {
                                    itered_data.data.value =
                                        types::Types::Number(types::number_type::NumberType {
                                            value: types::number_type::NumberSize::F64(
                                                parsed_double,
                                            ),
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            r#type: types::number_type::NumberTypes::F32,
                                            complete: false,
                                        })
                                } else {
                                    errors.push(error::Error {
                                        debug_message: "Onion".to_string(),
                                        title: error::errorList::error_s16.title.clone(),
                                        code: error::errorList::error_s16.code,
                                        message: error::errorList::error_s16.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s16.message.clone(),
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "val".to_string(),
                                                    value: (refference_value.raw.clone()
                                                        + letter_char),
                                                },
                                                error::ErrorBuildField {
                                                    key: "type".to_string(),
                                                    value: "f64".to_string(),
                                                },
                                            ],
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                }
                            } else {
                                errors.push(error::Error {
                                    debug_message: "Heliport".to_string(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_string(),
                                                value: itered_data.r#type.data.name.clone(),
                                            },
                                            error::ErrorBuildField {
                                                key: "token2".to_string(),
                                                value: "f32".to_string(),
                                            },
                                        ],
                                    ),
                                    pos: defs::Cursor {
                                        range_start: pos,
                                        range_end: pos.clone().skipChar(1),
                                    },
                                });
                            }
                        } else {
                            panic!("Unexpected token number");
                        }
                    } else {
                        panic!("Unexpected token number: {:#?}", data);
                        //Properties cannot be a raw number
                    }
                } else {
                    let chain_last_element = data.chain.len() - 1;
                    data.chain[chain_last_element] =
                        data.chain[chain_last_element].clone() + letter_char;
                }
            }
        }
    }
}
