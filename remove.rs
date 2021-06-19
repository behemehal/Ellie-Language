match &*data.refference {
                            types::Types::U8(refference_value) => {

                            }
                            types::Types::U16(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "7e8313ae49df43aa261be511abca0644".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "e1ba853d4ecb8003caa699585d95fc03".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "fd651ed43d904cef5ee1bb19c158a34a".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "e76b1ea0cd456ed26f28ea33993a49c5".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "7bb67f4afe7eb691040c6f48d281738e".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "3723bc4572797c0d78276ae2379c3416".to_string(),
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
                            }
                            types::Types::U32(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "08facb7bfcb6d2b671c4a941ba36cfd9".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "e7e5d56b0aa724cd6c1f11834e82b315".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "1a72057a90d0f29224eac346cc2a4a4d".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "656489f016c946b7e5d3a0e5f275d9cd".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "63081087d92c6088ee6a5449adda88ea".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "0bd1de0e8b6207b97d3a68250cda00c0".to_string(),
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
                            }
                            types::Types::U128(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "dc782625007e6832e98fb21e2cca5b25".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "b396cc2308bc3ef0c6433e46e42288fe".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "d0a58c060db7b69bba7c4edfbdcf1746".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "cbdbb594124fe50d5bf6edaae56846be".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "734577ed4963a0c54c13ddc56913eb6b".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "f66326d67b8f1e2448802da9b4cbb5a8".to_string(),
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
                            }
                            types::Types::Usize(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "ed6c0a3e481aed5f7ff7d036d4cacc27".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "a957619957d706fe2ee6f06728ac9abe".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "a0cf4d076bfbf7fdc603fdd7aa23b0c4".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "a4c5e136269f7964b021faaa82587656".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "c602233b79514bfce20f74120f765c4d".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "8573289361e034b3655d72b36ce9e3eb".to_string(),
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
                            }
                            types::Types::I8(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "19ed9d06c42671ddc78277d6b7308b04".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "dcd4fbf9e2e988ca1da0bf5918665f8a".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "809270a5a2e1bd536056d07e07ad8a55".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "55632377f4a3b098dddfa3f16413d396".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "f9062df2cf069641e33cb3cfa47fcfa3".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "482bb5a4469b85feacd57b4c4303cbff".to_string(),
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
                            }
                            types::Types::I16(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "a01c7b121c43305f6a051eefa70e19ea".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "31cb90285ab837396df57623479e7685".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "23339afe7841babe7c4d09fa38b04bbc".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "4a4334b364abc74cfc08232b796e9135".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "f0ff62bd3867352d5376fca2413b9fd9".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "2ed110c6bd432253bdfade1968c25446".to_string(),
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
                            }
                            types::Types::I32(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "b9bacad49cdc0db432c0f85f1b19985a".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "7490b067d8e159347c19f9a9e4363693".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "3996648eb1cf9e33b22381adfeea6c26".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "43bdcfb2a525e8d5219f44efbd1de816".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "5837ac6bea2d8be7b4db04789eca3468".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "b48c81a26e481d9e017d7f7f85d6c6e6".to_string(),
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
                            }
                            types::Types::I64(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "16727ca69cf144c0ffbf938c73cee0cc".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "08e4b9e735f77eb833b87c7133a88548".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "118b13eb6596837af71c12b2381f18a5".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "0d5dfb6f5469eee85d1ba3a3e1d0f98f".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "7197cfac83a49535447c3fb9227cbb6c".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "9492d8263379076d70d2ba217ea3cc9f".to_string(),
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
                            }
                            types::Types::I128(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "7c9b54714cd691abc8082d0386b7a5c7".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "049c8078f3a885183bded6166d60667a".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "e85024abbcb7ef6c782dd249ee35cc96".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "7d25b419433eb74edd9343545b4db2ec".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "38ce470f308ed35340b63b7f30ed48bf".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "72685730fb355a83544f64a41fe035a4".to_string(),
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
                            }
                            types::Types::Isize(refference_value) => {
                                if itered_data.data.dynamic {
                                    itered_data.rtype = definers::DefinerCollecting::Generic(definers::GenericType {
                                        rtype: "f32".to_string(),
                                    });
                                    //  itered_data.rtype.raw_name()
                                }
                            
                                if let types::integer_type::IntegerSize::F32(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "f06a68224c93ea410c01ab252c95494c".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if let types::integer_type::IntegerSize::F64(_) = refference_value.value {
                                    errors.push(error::Error {
                                        debug_message: "e7e2a22bf176f4af8ff319b298d76455".to_string(),
                                        title: error::errorList::error_s18.title.clone(),
                                        code: error::errorList::error_s18.code,
                                        message: error::errorList::error_s18.message.clone(),
                                        builded_message: error::BuildedError::build_from_string(
                                            error::errorList::error_s18.message.clone(),
                                        ),
                                        pos: defs::Cursor {
                                            range_start: pos,
                                            range_end: pos.clone().skipChar(1),
                                        },
                                    });
                                } else if itered_data.rtype.raw_name() == "f32" {
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f32>();
                                    if let Ok(parsed_double) = double_parse {
                                        if parsed_double.is_infinite() {
                                            errors.push(error::Error {
                                                debug_message: "8754ced22ef43f34a70bedf07594673e".to_string(),
                                                title: error::errorList::error_s17.title.clone(),
                                                code: error::errorList::error_s17.code,
                                                message: error::errorList::error_s17.message.clone(),
                                                builded_message: error::Error::build(
                                                    error::errorList::error_s17.message.clone(),
                                                    vec![error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
                                                    }],
                                                ),
                                                pos: defs::Cursor {
                                                    range_start: pos.clone().popChar(
                                                        (refference_value.raw.clone() + "." + letter_char).len() as i64,
                                                    ),
                                                    range_end: pos.clone().skipChar(1),
                                                },
                                            });
                                        } else {
                                            itered_data.data.value = types::Types::F32(types::numbers::F32::F32Type {
                                                value: parsed_double,
                                                raw: (refference_value.raw.clone() + "." + letter_char),
                                                complete: false,
                                            })
                                        }
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "bfde2fdc307191b92e5f26168f95a04d".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + letter_char),
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
                                    let double_parse = (refference_value.raw.clone() + "." + letter_char).parse::<f64>();
                                    if let Ok(parsed_double) = double_parse {
                                        itered_data.data.value = types::Types::F64(types::numbers::F64::F64Type {
                                            value: parsed_double,
                                            raw: (refference_value.raw.clone() + "." + letter_char),
                                            complete: false,
                                        })
                                    } else {
                                        errors.push(error::Error {
                                            debug_message: "7d20210bb17826a0914b6b4015015f33".to_string(),
                                            title: error::errorList::error_s16.title.clone(),
                                            code: error::errorList::error_s16.code,
                                            message: error::errorList::error_s16.message.clone(),
                                            builded_message: error::Error::build(
                                                error::errorList::error_s16.message.clone(),
                                                vec![
                                                    error::ErrorBuildField {
                                                        key: "val".to_string(),
                                                        value: (refference_value.raw.clone() + "." + letter_char),
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
                                        debug_message: "03752fa2e7637de60b846a2378636a6e".to_string(),
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
                            }
                            (_) => {
                                errors.push(error::Error {
                                    debug_message:
                                        "./parser/src/processors/type_processors/refference.rs:248"
                                            .to_string(),
                                    title: error::errorList::error_s18.title.clone(),
                                    code: error::errorList::error_s18.code,
                                    message: error::errorList::error_s18.message.clone(),
                                    builded_message: error::BuildedError::build_from_string(
                                        error::errorList::error_s18.message.clone(),
                                    ),
                                    pos: defs::Cursor {
                                        range_start: pos,
                                        range_end: pos.clone().skipChar(1),
                                    },
                                });
                            }
                        }
