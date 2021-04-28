use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Number(ref mut data) = itered_data.data.value {
        let is_num = letter_char.parse::<isize>().is_ok();

        if is_num {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "Caria".to_string(),
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
                /*
                let dynamic_calculated_type =
                if nm >= 0 && nm <= 255
                if nm <= 127 && nm >= -128 {
                    types::number_type::NumberTypes::I8
                } else if nm < 32767 && nm > -32768 {
                    types::number_type::NumberTypes::I16
                } else if nm < 2147483647 && nm > -2147483648 {
                    types::number_type::NumberTypes::I32
                } else if nm < 9223372036854775807 && nm > -9223372036854775808 {
                    types::number_type::NumberTypes::I64
                } else if nm < 170141183460469231731687303715884105727 && nm > -170141183460469231731687303715884105728 {
                    types::number_type::NumberTypes::I128
                } else {
                    types::number_type::NumberTypes::ISize
                };
                */

                if itered_data.r#type.data.name == "u8" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u8>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U8(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "u8".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "u16" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u16>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U16(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "u16".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "u32" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u32>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U32(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "u32".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "u64" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u64>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U64(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "u64".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "u128" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u128>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U128(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "u128".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "usize" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<usize>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::Usize(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "usize".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "i8" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i8>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I8(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "i8".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "i16" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i16>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I16(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "i16".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "i32" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i32>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I32(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "i32".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "i64" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i64>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I64(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "i64".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "i128" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i128>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I128(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "i128".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "isize" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<isize>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::Isize(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
                                    },
                                    error::ErrorBuildField {
                                        key: "type".to_string(),
                                        value: "isize".to_string(),
                                    },
                                ],
                            ),
                            pos: defs::Cursor {
                                range_start: pos,
                                range_end: pos.clone().skipChar(1),
                            },
                        });
                    }
                } else if itered_data.r#type.data.name == "f32" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<f32>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::F32(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
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
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<f64>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::F64(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "hurt".to_string(),
                            title: error::errorList::error_s16.title.clone(),
                            code: error::errorList::error_s16.code,
                            message: error::errorList::error_s16.message.clone(),
                            builded_message: error::Error::build(
                                error::errorList::error_s16.message.clone(),
                                vec![
                                    error::ErrorBuildField {
                                        key: "val".to_string(),
                                        value: data.raw.clone(),
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
                    //#[cfg(feature = "std")]
                    println!("{:#?}", itered_data.r#type);

                    errors.push(error::Error {
                        debug_message: "hurt".to_string(),
                        title: error::errorList::error_s3.title.clone(),
                        code: error::errorList::error_s3.code,
                        message: error::errorList::error_s3.message.clone(),
                        builded_message: error::Error::build(
                            error::errorList::error_s3.message.clone(),
                            vec![
                                error::ErrorBuildField {
                                    key: "token1".to_string(),
                                    value: itered_data.r#type.collecting.to_string(), 
                                },
                                error::ErrorBuildField {
                                    key: "token2".to_string(),
                                    value: data.value.to_string().to_lowercase(),
                                },
                            ],
                        ),
                        pos: defs::Cursor {
                            range_start: pos,
                            range_end: pos.clone().skipChar(1),
                        },
                    });
                }

                //data.complete = true;
            }
        } else if letter_char == "." {
            // String prototype
            data.complete = true;
            itered_data.data.value =
                types::Types::Refference(types::refference_type::RefferenceType {
                    refference: Box::new(itered_data.data.value.clone()),
                    on_dot: true,
                    chain: Vec::new(),
                });
        } else if types::logical_type::LogicalOpearators::is_opearator(letter_char) {
            data.complete = true;
            itered_data.data.value = types::Types::Operator(types::operator_type::OperatorType {
                first: Box::new(itered_data.data.value.clone()),
                first_filled: true,
                operator: types::operator_type::Operators::LogicalType(
                    types::logical_type::LogicalOpearators::Null,
                ),
                operator_collect: letter_char.to_string(),
                ..Default::default()
            });
        } else if types::comparison_type::ComparisonOperators::is_opearator(letter_char) {
            data.complete = true;
            itered_data.data.value = types::Types::Operator(types::operator_type::OperatorType {
                first: Box::new(itered_data.data.value.clone()),
                first_filled: true,
                operator_collect: letter_char.to_string(),
                operator: types::operator_type::Operators::ComparisonType(
                    types::comparison_type::ComparisonOperators::Null,
                ),
                ..Default::default()
            });
        } else if letter_char == " " || letter_char == ")" {
            data.complete = true;
        } else {
            errors.push(error::Error {
                debug_message: "mRNA".to_string(),
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
