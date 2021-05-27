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
                    debug_message: "./parser/src/processors/type_processors/number.rs:22" .to_string(),
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
<<<<<<< HEAD
                if matches!(&itered_data.rtype, crate::syntax::definers::DefinerCollecting::Generic(x) if x.rtype.is_empty())
                    && itered_data.data.dynamic
                {
                    //Make type default to u16
                    itered_data.rtype = crate::syntax::definers::DefinerCollecting::Generic(
=======
                if matches!(&itered_data.r#type, crate::syntax::definers::DefinerCollecting::Generic(x) if x.r#type.is_empty())
                    && itered_data.data.dynamic
                {
                    //Make type default to u16
                    itered_data.r#type = crate::syntax::definers::DefinerCollecting::Generic(
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:58"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:58" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:89"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:89" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:120"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:120" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:151"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:151" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:182"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:182" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:213"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:213" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:244"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:244" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:275"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:275" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:306"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:306" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:337"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:337" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:368"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:368" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:399"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:399" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:430"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:430" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
<<<<<<< HEAD
                            debug_message: "./parser/src/processors/type_processors/number.rs:461"
                                .to_string(),
=======
                            debug_message: "./parser/src/processors/type_processors/number.rs:461" .to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
                    errors.push(error::Error {
<<<<<<< HEAD
                        debug_message: "@./parser/src/processors/type_processors/number.rs:487"
                            .to_string(),
=======
                        debug_message: "@./parser/src/processors/type_processors/number.rs:487".to_string(),
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
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
                debug_message: "./parser/src/processors/type_processors/number.rs:560".to_string(),
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

