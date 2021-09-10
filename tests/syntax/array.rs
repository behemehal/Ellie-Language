#[cfg(test)]
mod array_tests {
    #[test]
    fn one_dimension_array_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("[1, 2, 3]"),
            ellie_parser::syntax::types::Types::Array(
                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                    complete: true,
                    comma: false,
                    child_start: false,
                    data: ellie_parser::syntax::types::array_type::ArrayType {
                        layer_size: 3,
                        collective: vec![
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                    value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                    },
                                    raw: "1".to_owned(),
                                    complete: true,
                                },
                            ),),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(0, 1,),
                                range_end: ellie_core::defs::CursorPosition(0, 2,),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(2),
                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                    },
                                    raw: "2".to_owned(),
                                    complete: true,
                                },
                            ),),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(0, 4,),
                                range_end: ellie_core::defs::CursorPosition(0, 5,),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(3),
                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                    },
                                    raw: "3".to_owned(),
                                    complete: true,
                                },
                            ),),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(0, 7,),
                                range_end: ellie_core::defs::CursorPosition(0, 8,),
                            },
                        },
                    ],
                    },
                },
            ),
        ),)
    }

    #[test]
    fn two_dimension_array_collected_with_no_error() {
        assert!(
            ellie_engine::test_utils::has_no_error_and_correct(
                ellie_engine::test_utils::emulate_value_processor("[[1], [2], [3]]"),
                ellie_parser::syntax::types::Types::Array(
                    ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                        complete: true,
                        comma: false,
                        child_start: false,
                        data: ellie_parser::syntax::types::array_type::ArrayType {
                            layer_size: 3,
                        collective: vec![
                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(ellie_parser::syntax::types::Types::Array(
                                    ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                        complete: true,
                                        comma: false,
                                        child_start: false,
                                        data: ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                        collective: vec![
                                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                                value_complete: true,
                                                value: Box::new(
                                                    ellie_parser::syntax::types::Types::Integer(
                                                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                            },
                                                            raw: "1".to_owned(),
                                                            complete: true,
                                                        },
                                                    ),
                                                ),
                                                location: ellie_core::defs::Cursor {
                                                    range_start: ellie_core::defs::CursorPosition(
                                                        0,
                                                        2,
                                                    ),
                                                    range_end: ellie_core::defs::CursorPosition(
                                                        0,
                                                        3,
                                                    ),
                                                },
                                            },
                                        ],
                                        },
                                    },
                                ),),
                                location: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(
                                        0,
                                        1,
                                    ),
                                    range_end: ellie_core::defs::CursorPosition(
                                        0,
                                        4,
                                    ),
                                },
                            },
                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(ellie_parser::syntax::types::Types::Array(
                                    ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                        complete: true,
                                        comma: false,
                                        child_start: false,
                                        data: ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                        collective: vec![
                                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                                value_complete: true,
                                                value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(2),
                                                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                        },
                                                        raw: "2".to_owned(),
                                                        complete: true,
                                                    },
                                                ),),
                                                location: ellie_core::defs::Cursor {
                                                    range_start: ellie_core::defs::CursorPosition(
                                                        0,
                                                        7,
                                                    ),
                                                    range_end: ellie_core::defs::CursorPosition(
                                                        0,
                                                        8,
                                                    ),
                                                },
                                            },
                                        ],
                                        }
                                    },
                                ),),
                                location: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(
                                        0,
                                        6,
                                    ),
                                    range_end: ellie_core::defs::CursorPosition(
                                        0,
                                        9,
                                    ),
                                },
                            },
                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(ellie_parser::syntax::types::Types::Array(
                                    ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                        complete: true,
                                        comma: false,
                                        child_start: false,
                                        data: ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                        collective: vec![
                                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                                value_complete: true,
                                                value: Box::new(
                                                    ellie_parser::syntax::types::Types::Integer(
                                                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(3),
                                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                            },
                                                            raw: "3".to_owned(),
                                                            complete: true,
                                                        },
                                                    ),
                                                ),
                                                location: ellie_core::defs::Cursor {
                                                    range_start: ellie_core::defs::CursorPosition(
                                                        0,
                                                        12,
                                                    ),
                                                    range_end: ellie_core::defs::CursorPosition(
                                                        0,
                                                        13,
                                                    ),
                                                },
                                            },
                                        ],}
                                    },
                                ),),
                                location: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(
                                        0,
                                        11,
                                    ),
                                    range_end: ellie_core::defs::CursorPosition(
                                        0,
                                        14,
                                    ),
                                },
                            },
                        ],},
                    },
                ),
            ),
        );
    }

    #[test]
    fn three_dimension_array_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("[[[1]], [[2]], [[3]]]"),
            ellie_parser::syntax::types::Types::Array(
                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                    complete: true,
                    comma: false,
                    child_start: false,
                    data: ellie_parser::syntax::types::array_type::ArrayType {
                        layer_size: 3,
                    collective: vec![
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                    complete: true,
                                    comma: false,
                                    child_start: false,
                                    data: ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                    collective: vec![
                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                            value_complete: true,
                                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                                    complete: true,
                                                    comma: false,
                                                    child_start: false,
                                                    data: ellie_parser::syntax::types::array_type::ArrayType {
                                                        layer_size: 1,
                                                    collective: vec![
                                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                                            value_complete: true,
                                                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    },
                                                                    raw: "1".to_owned(),
                                                                    complete: true,
                                                                },
                                                            ),),
                                                            location: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    3,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    4,
                                                                ),
                                                            },
                                                        },
                                                    ], },
                                                },
                                            )),
                                            location: ellie_core::defs::Cursor {
                                                range_start: ellie_core::defs::CursorPosition(
                                                    0,
                                                    2,
                                                ),
                                                range_end: ellie_core::defs::CursorPosition(
                                                    0,
                                                    5,
                                                ),
                                            },
                                        },
                                    ],}
                                },
                            )),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(
                                    0,
                                    1,
                                ),
                                range_end: ellie_core::defs::CursorPosition(
                                    0,
                                    6,
                                ),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                    complete: true,
                                    comma: false,
                                    child_start: false,
                                    data: ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                    collective: vec![
                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                            value_complete: true,
                                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                                    complete: true,
                                                    comma: false,
                                                    child_start: false,
                                                    data: ellie_parser::syntax::types::array_type::ArrayType {
                                                        layer_size: 1,
                                                    collective: vec![
                                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                                            value_complete: true,
                                                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                                                            2,
                                                                        ),
                                                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    },
                                                                    raw: "2".to_owned(),
                                                                    complete: true,
                                                                },
                                                            )),
                                                            location: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    10,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    11,
                                                                ),
                                                            },
                                                        },
                                                    ],}
                                                },
                                            )),
                                            location: ellie_core::defs::Cursor {
                                                range_start: ellie_core::defs::CursorPosition(
                                                    0,
                                                    9,
                                                ),
                                                range_end: ellie_core::defs::CursorPosition(
                                                    0,
                                                    12,
                                                ),
                                            },
                                        },
                                    ],}
                                },
                            )),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(
                                    0,
                                    8,
                                ),
                                range_end: ellie_core::defs::CursorPosition(
                                    0,
                                    13,
                                ),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                    complete: true,
                                    comma: false,
                                    child_start: false,
                                    data: ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                    collective: vec![
                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                            value_complete: true,
                                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                                ellie_parser::syntax::types::array_type::ArrayTypeCollector {
                                                    complete: true,
                                                    comma: false,
                                                    child_start: false,
                                                    data: ellie_parser::syntax::types::array_type::ArrayType {
                                                        layer_size: 1,
                                                    collective: vec![
                                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                                            value_complete: true,
                                                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(3),
                                                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    },
                                                                    raw: "3".to_owned(),
                                                                    complete: true,
                                                                },
                                                            )),
                                                            location: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    17,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    18,
                                                                ),
                                                            },
                                                       },
                                                    ],}
                                                },
                                            )),
                                            location: ellie_core::defs::Cursor {
                                                range_start: ellie_core::defs::CursorPosition(
                                                    0,
                                                    16,
                                                ),
                                                range_end: ellie_core::defs::CursorPosition(
                                                    0,
                                                    19,
                                                ),
                                            },
                                        },
                                    ],},
                                },
                            )),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(
                                    0,
                                    15,
                                ),
                                range_end: ellie_core::defs::CursorPosition(
                                    0,
                                    20,
                                ),
                            },
                        },
                    ],
                },
            },
        ),
    ),
);
    }
}
