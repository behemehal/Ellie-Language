use crate::syntax::{definers, types, variable};
use ellie_core::{defs, error, utils};

use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_refference(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    last_char: String,
    pos: defs::CursorPosition,
    _options: defs::ParserOptions,
) {
    if let types::Types::Refference(ref mut data) = itered_data.data.value {
        if letter_char == "." {
            if data.on_dot {
                errors.push(error::Error {
                    debug_message: "a81199d6a09bdd5851f8d6d5db3bf4c6"
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
                        debug_message: "799df70f6c5747599900205fefdec46d"
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

                            if itered_data.data.dynamic {
                                itered_data.rtype =
                                    definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                //  itered_data.rtype.raw_name()
                            }

                            if let types::number_type::NumberSize::F32(_) = refference_value.value {
                                errors.push(error::Error {
                                    debug_message:
                                        "./parser/src/processors/type_processors/refference.rs:89"
                                            .to_string(),
                                    title: error::errorList::error_s18.title.clone(),
                                    code: error::errorList::error_s18.code,
                                    message: error::errorList::error_s18.message.clone(),
                                    builded_message: error::BuildedError::build_from_string(error::errorList::error_s18.message.clone()),
                                    pos: defs::Cursor {
                                        range_start: pos,
                                        range_end: pos.clone().skipChar(1),
                                    },
                                });
                            } else if let types::number_type::NumberSize::F64(_) =
                                refference_value.value
                            {
                                errors.push(error::Error {
                                    debug_message:
                                        "./parser/src/processors/type_processors/refference.rs:101"
                                            .to_string(),
                                    title: error::errorList::error_s18.title.clone(),
                                    code: error::errorList::error_s18.code,
                                    message: error::errorList::error_s18.message.clone(),
                                    builded_message: error::BuildedError::build_from_string(error::errorList::error_s18.message.clone()),
                                    pos: defs::Cursor {
                                        range_start: pos,
                                        range_end: pos.clone().skipChar(1),
                                    },
                                });
                            } else if itered_data.rtype.raw_name() == "f32" {
                                let double_parse =
                                    (refference_value.raw.clone() + "." + letter_char)
                                        .parse::<f32>();
                                if let Ok(parsed_double) = double_parse {
                                    if parsed_double.is_infinite() {
                                        errors.push(error::Error {
                                            debug_message: "ae4b3dbbbaed470a884cc9bca505d614".to_string(),
                                            title: error::errorList::error_s17.title.clone(),
                                            code: error::errorList::error_s17.code,
                                            message: error::errorList::error_s17.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s17.message.clone(),
                                                vec![error::ErrorBuildField {
                                                    key: "val".to_string(),
                                                    value: (refference_value.raw.clone()
                                                        + letter_char),
                                                }],
                                            ),
                                            pos: defs::Cursor {
                                                range_start: pos.clone().popChar(
                                                    (refference_value.raw.clone()
                                                        + "."
                                                        + letter_char)
                                                        .len()
                                                        as i64,
                                                ),
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
                                                rtype: types::number_type::NumberTypes::F32,
                                                complete: false,
                                            })
                                    }
                                } else {
                                    errors.push(error::Error {
                                        debug_message: "a8930ce99eb974aa2f6b625e11cc672b".to_string(),
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
                            } else if itered_data.rtype.raw_name() == "f64" {
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
                                            rtype: types::number_type::NumberTypes::F32,
                                            complete: false,
                                        })
                                } else {
                                    errors.push(error::Error {
                                        debug_message: "88aab3a5e23244ee0ebcd07887b05bbb".to_string(),
                                        title: error::errorList::error_s16.title.clone(),
                                        code: error::errorList::error_s16.code,
                                        message: error::errorList::error_s16.message.clone(),
                                        builded_message: error::Error::build(
                                            error::errorList::error_s16.message.clone(),
                                            vec![
                                                error::ErrorBuildField {
                                                    key: "val".to_string(),
                                                    value: (refference_value.raw.clone()
                                                        + "."
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
                                    debug_message:
                                        "./parser/src/processors/type_processors/refference.rs:223"
                                            .to_string(),
                                    title: error::errorList::error_s3.title.clone(),
                                    code: error::errorList::error_s3.code,
                                    message: error::errorList::error_s3.message.clone(),
                                    builded_message: error::Error::build(
                                        error::errorList::error_s3.message.clone(),
                                        vec![
                                            error::ErrorBuildField {
                                                key: "token1".to_string(),
                                                value: itered_data.rtype.raw_name(),
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
                            errors.push(error::Error {
                                debug_message:
                                    "./parser/src/processors/type_processors/refference.rs:248"
                                        .to_string(),
                                title: error::errorList::error_s18.title.clone(),
                                code: error::errorList::error_s18.code,
                                message: error::errorList::error_s18.message.clone(),
                                builded_message: error::BuildedError::build_from_string(error::errorList::error_s18.message.clone()),
                                pos: defs::Cursor {
                                    range_start: pos,
                                    range_end: pos.clone().skipChar(1),
                                },
                            });
                        }
                    } else {
                        errors.push(error::Error {
                            debug_message:
                                "./parser/src/processors/type_processors/refference.rs:261"
                                    .to_string(),
                            title: error::errorList::error_s18.title.clone(),
                            code: error::errorList::error_s18.code,
                            message: error::errorList::error_s18.message.clone(),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                            ..Default::default()
                        });
                    }
                } else {
                    let current_reliability = utils::reliable_name_range(
                        utils::ReliableNameRanges::VariableName,
                        letter_char.to_string(),
                    );
                    if current_reliability.reliable {
                        if data.chain.is_empty() {
                            data.chain.push(letter_char.to_string());
                        } else {
                            let chain_last_element = data.chain.len() - 1;
                            data.chain[chain_last_element] =
                                data.chain[chain_last_element].clone() + letter_char;
                        }
                    } else {
                        errors.push(error::Error {
                            debug_message:
                                "./parser/src/processors/type_processors/refference.rs:287"
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
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                }
            }
        }
    }
}





