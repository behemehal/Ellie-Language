#[cfg(test)]
mod array_tests {

    #[test]
    fn one_dimension_array_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("[1, 2, 3]"),
            ellie_parser::syntax::types::Types::Array(
                ellie_parser::syntax::types::array_type::ArrayType {
                    layer_size: 3,
                    complete: true,
                    comma: false,
                    child_start: false,
                    collective: vec![
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                ellie_parser::syntax::types::integer_type::IntegerType {
                                    value:
                                        ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                            1
                                        ),
                                    raw: "1".to_string(),
                                    rtype:
                                        ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                    complete: true,
                                },
                            ),),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(0, 0,),
                                range_end: ellie_core::defs::CursorPosition(0, 1,),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                ellie_parser::syntax::types::integer_type::IntegerType {
                                    value:
                                        ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                            2
                                        ),
                                    raw: "2".to_string(),
                                    rtype:
                                        ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                    complete: true,
                                },
                            ),),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(0, 1,),
                                range_end: ellie_core::defs::CursorPosition(0, 1,),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                ellie_parser::syntax::types::integer_type::IntegerType {
                                    value:
                                        ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                            3
                                        ),
                                    raw: "3".to_string(),
                                    rtype:
                                        ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                    complete: true,
                                },
                            ),),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(0, 1,),
                                range_end: ellie_core::defs::CursorPosition(0, 1,),
                            },
                        },
                    ],
                },
            ),
        ),)
    }

    #[test]
    fn two_dimension_array_collected_with_no_error() {
        assert!(
            ellie_lang::test_utils::has_no_error_and_correct(
                ellie_lang::test_utils::emulate_value_processor("[[1], [2], [3]]"),
                ellie_parser::syntax::types::Types::Array(
                    ellie_parser::syntax::types::array_type::ArrayType {
                        layer_size: 3,
                        complete: true,
                        comma: false,
                        child_start: false,
                        collective: vec![
                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(ellie_parser::syntax::types::Types::Array(
                                    ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                        complete: true,
                                        comma: false,
                                        child_start: false,
                                        collective: vec![
                                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                                value_complete: true,
                                                value: Box::new(ellie_parser::syntax::types::Types::Integer(ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                                raw: "1".to_string(),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                complete: true,
                                            },),),
                                                location: ellie_core::defs::Cursor {
                                                    range_start: ellie_core::defs::CursorPosition(
                                                        0,
                                                        0,
                                                    ),
                                                    range_end: ellie_core::defs::CursorPosition(
                                                        0,
                                                        1,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                ),),
                                location: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(
                                        0,
                                        0,
                                    ),
                                    range_end: ellie_core::defs::CursorPosition(
                                        0,
                                        1,
                                    ),
                                },
                            },
                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(ellie_parser::syntax::types::Types::Array(
                                    ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                        complete: true,
                                        comma: false,
                                        child_start: false,
                                        collective: vec![
                                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                                value_complete: true,
                                                value: Box::new(ellie_parser::syntax::types::Types::Integer(ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(2),
                                                raw: "2".to_string(),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                complete: true,
                                            },),),
                                                location: ellie_core::defs::Cursor {
                                                    range_start: ellie_core::defs::CursorPosition(
                                                        0,
                                                        0,
                                                    ),
                                                    range_end: ellie_core::defs::CursorPosition(
                                                        0,
                                                        1,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                ),),
                                location: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(
                                        0,
                                        0,
                                    ),
                                    range_end: ellie_core::defs::CursorPosition(
                                        0,
                                        1,
                                    ),
                                },
                            },
                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(ellie_parser::syntax::types::Types::Array(
                                    ellie_parser::syntax::types::array_type::ArrayType {
                                        layer_size: 1,
                                        complete: true,
                                        comma: false,
                                        child_start: false,
                                        collective: vec![
                                            ellie_parser::syntax::types::array_type::ArrayEntry {
                                                value_complete: true,
                                                value: Box::new(ellie_parser::syntax::types::Types::Integer(ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(3),
                                                raw: "3".to_string(),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                complete: true,
                                            },),),
                                                location: ellie_core::defs::Cursor {
                                                    range_start: ellie_core::defs::CursorPosition(
                                                        0,
                                                        0,
                                                    ),
                                                    range_end: ellie_core::defs::CursorPosition(
                                                        0,
                                                        1,
                                                    ),
                                                },
                                            },
                                        ],
                                    },
                                ),),
                                location: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(
                                        0,
                                        0,
                                    ),
                                    range_end: ellie_core::defs::CursorPosition(
                                        0,
                                        1,
                                    ),
                                },
                            },
                        ],
                    },
                ),
            ),
        );
    }

    #[test]
    fn three_dimension_array_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("[[[1]], [[2]], [[3]]]"),
            ellie_parser::syntax::types::Types::Array(
                ellie_parser::syntax::types::array_type::ArrayType {
                    layer_size: 3,
                    complete: true,
                    comma: false,
                    child_start: false,
                    collective: vec![
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                ellie_parser::syntax::types::array_type::ArrayType {
                                    layer_size: 1,
                                    complete: true,
                                    comma: false,
                                    child_start: false,
                                    collective: vec![
                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                            value_complete: true,
                                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                                ellie_parser::syntax::types::array_type::ArrayType {
                                                    layer_size: 1,
                                                    complete: true,
                                                    comma: false,
                                                    child_start: false,
                                                    collective: vec![
                                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                                            value_complete: true,
                                                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerType {
                                                                    value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                                                        1,
                                                                    ),
                                                                    raw: "1".to_string(),
                                                                    rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    complete: true,
                                                                },
                                                            ),),
                                                            location: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    0,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                            )),
                                            location: ellie_core::defs::Cursor {
                                                range_start: ellie_core::defs::CursorPosition(
                                                    0,
                                                    0,
                                                ),
                                                range_end: ellie_core::defs::CursorPosition(
                                                    0,
                                                    1,
                                                ),
                                            },
                                        },
                                    ],
                                },
                            )),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(
                                    0,
                                    0,
                                ),
                                range_end: ellie_core::defs::CursorPosition(
                                    0,
                                    1,
                                ),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                ellie_parser::syntax::types::array_type::ArrayType {
                                    layer_size: 1,
                                    complete: true,
                                    comma: false,
                                    child_start: false,
                                    collective: vec![
                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                            value_complete: true,
                                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                                ellie_parser::syntax::types::array_type::ArrayType {
                                                    layer_size: 1,
                                                    complete: true,
                                                    comma: false,
                                                    child_start: false,
                                                    collective: vec![
                                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                                            value_complete: true,
                                                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerType {
                                                                    value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                                                        2,
                                                                    ),
                                                                    raw: "2".to_string(),
                                                                    rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    complete: true,
                                                                },
                                                            )),
                                                            location: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    0,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                            )),
                                            location: ellie_core::defs::Cursor {
                                                range_start: ellie_core::defs::CursorPosition(
                                                    0,
                                                    0,
                                                ),
                                                range_end: ellie_core::defs::CursorPosition(
                                                    0,
                                                    1,
                                                ),
                                            },
                                        },
                                    ],
                                },
                            )),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(
                                    0,
                                    0,
                                ),
                                range_end: ellie_core::defs::CursorPosition(
                                    0,
                                    1,
                                ),
                            },
                        },
                        ellie_parser::syntax::types::array_type::ArrayEntry {
                            value_complete: true,
                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                ellie_parser::syntax::types::array_type::ArrayType {
                                    layer_size: 1,
                                    complete: true,
                                    comma: false,
                                    child_start: false,
                                    collective: vec![
                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                            value_complete: true,
                                            value: Box::new(ellie_parser::syntax::types::Types::Array(
                                                ellie_parser::syntax::types::array_type::ArrayType {
                                                    layer_size: 1,
                                                    complete: true,
                                                    comma: false,
                                                    child_start: false,
                                                    collective: vec![
                                                        ellie_parser::syntax::types::array_type::ArrayEntry {
                                                            value_complete: true,
                                                            value: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerType {
                                                                    value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                                                        3,
                                                                    ),
                                                                    raw: "3".to_string(),
                                                                    rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    complete: true,
                                                                },
                                                            )),
                                                            location: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    0,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    1,
                                                                ),
                                                            },
                                                        },
                                                    ],
                                                },
                                            )),
                                            location: ellie_core::defs::Cursor {
                                                range_start: ellie_core::defs::CursorPosition(
                                                    0,
                                                    0,
                                                ),
                                                range_end: ellie_core::defs::CursorPosition(
                                                    0,
                                                    1,
                                                ),
                                            },
                                        },
                                    ],
                                },
                            )),
                            location: ellie_core::defs::Cursor {
                                range_start: ellie_core::defs::CursorPosition(
                                    0,
                                    0,
                                ),
                                range_end: ellie_core::defs::CursorPosition(
                                    0,
                                    1,
                                ),
                            },
                        },
                    ],
                },
            ),
        ),);
    }
}
