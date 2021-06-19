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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:89".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:101".to_string(),
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
                                                debug_message: "c1d6f2151d0c557e94a975b806204b74".to_string(),
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
                                            debug_message: "0cc147769a4fb2b9390e8b739ecb44fa".to_string(),
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
                                            debug_message: "85373b694c83394a8c8ce937cd3feffb".to_string(),
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
                                        debug_message: "./parser/src/processors/type_processors/refference.rs:223".to_string(),
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