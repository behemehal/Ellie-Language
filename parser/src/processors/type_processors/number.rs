use crate::syntax::{types, variable};
use ellie_core::{defs, error};

use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;

pub fn collect_number(
    itered_data: &mut variable::VariableCollector,
    errors: &mut Vec<error::Error>,
    letter_char: &str,
    _next_char: String,
    _last_char: String,
    pos: defs::CursorPosition,
) {
    if let types::Types::Number(ref mut data) = itered_data.data.value {
        let is_num = letter_char.parse::<isize>().is_ok();

        if is_num || letter_char == "x" && data.raw.starts_with('0') {
            if data.complete {
                errors.push(error::Error {
                    debug_message: "11351109feef10b08d15759f0ad27a88"
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
                if matches!(&itered_data.rtype, crate::syntax::definers::DefinerCollecting::Generic(x) if x.rtype.is_empty())
                    && itered_data.data.dynamic
                {
                    //Make type default to u16
                    itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
                        crate::syntax::definers::GenericType {
                            rtype: "u16".to_string(),
                        },
                    );
                }

                if itered_data.rtype.raw_name() == "u8" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u8>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U8(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "202ef8966c9528c20eb67bed63000205"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "u16" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u16>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U16(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "d6be5ca94689ba205219a0e900f9b0b0"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "u32" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u32>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U32(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "06204ca602c2fa33cb5f7baacb8be59b"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "u64" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u64>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U64(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "fb56238c11a4b84c587adc232178fe34"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "u128" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<u128>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::U128(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "a64fa0b21f9f8d32dbe2e0e70b58faf4"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "usize" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<usize>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::Usize(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "d0d987971f459aea6d0f2376e8594918"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "i8" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i8>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I8(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "1ab372711844e64b66b9625c31c97cc5"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "i16" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i16>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I16(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "4de9f83c400c3bd400c2f238e2cb3600"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "i32" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i32>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I32(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "c2769b16cb2de507293c9f9f162f183f"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "i64" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i64>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I64(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "13649bec3a2d18bffedde294c60865a9"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "i128" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<i128>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::I128(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "8d64659491b12f9d17f69d50c6ebaa47"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "isize" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<isize>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::Isize(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "27545f8bacdc70818ce033c982d9d2de"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "f32" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<f32>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::F32(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "30f8f6e480dd3c47b6277c4976330a87"
                                .to_string(),
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
                } else if itered_data.rtype.raw_name() == "f64" {
                    data.raw = data.raw.to_string() + letter_char;
                    let try_parse = data.raw.parse::<f64>();
                    if let Ok(nm) = try_parse {
                        data.value = types::number_type::NumberSize::F64(nm);
                    } else {
                        errors.push(error::Error {
                            debug_message: "c97ef8a82fc31f19b88ffbebe4c7e22c"
                                .to_string(),
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
                } else if !itered_data.rtype.raw_name().is_empty() {
                    //UNSAFE
                    errors.push(error::Error {
                        debug_message: "da7fd8902eb4e1f0054b59aba9d4564e"
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
                                    value: data.value.get_type().to_lowercase(),
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
        } else if types::logical_type::LogicalOpearators::is_logical_opearator(letter_char) {
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
        } else if types::comparison_type::ComparisonOperators::is_comparison_opearator(letter_char)
        {
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
        } else if types::arithmetic_type::ArithmeticOperators::is_arithmetic_opearator(letter_char)
        {
            data.complete = true;
            itered_data.data.value = types::Types::Operator(types::operator_type::OperatorType {
                first: Box::new(itered_data.data.value.clone()),
                first_filled: true,
                operator_collect: letter_char.to_string(),
                operator: types::operator_type::Operators::ArithmeticType(
                    types::arithmetic_type::ArithmeticOperators::Null,
                ),
                ..Default::default()
            });
        } else if letter_char == " " || letter_char == ")" {
            data.complete = true;
        } else {
            errors.push(error::Error {
                debug_message: "7c3cf2fae40c83f78d76a2b941855ced".to_string(),
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





