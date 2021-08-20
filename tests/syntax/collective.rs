#[cfg(test)]
mod collective_tests {

    #[test]
    fn empty_collective_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("{}"),
            ellie_parser::syntax::types::Types::Collective(
                ellie_parser::syntax::types::collective_type::CollectiveCollector {
                    complete: true,
                    at_comma: false,
                    data: ellie_parser::syntax::types::collective_type::Collective::default(),
                }
            )
        ),)
    }

    #[test]
    fn duplicate_parameter_handled() {
        assert!(
            ellie_lang::test_utils::emulate_value_processor("{1: 1, 1: 1}")
                .1
                .len()
                == 1
        )
    }

    #[test]
    fn one_dimension_collective_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("{1: 'o', 2: 'k'}"),
            ellie_parser::syntax::types::Types::Collective(
                ellie_parser::syntax::types::collective_type::CollectiveCollector {
                    complete: true,
                    at_comma: false,
                    data: ellie_parser::syntax::types::collective_type::Collective {
                        entries: vec![
                            ellie_parser::syntax::types::collective_type::CollectiveEntryCollector {
                                data: ellie_parser::syntax::types::collective_type::CollectiveEntry {
                                    key: Box::new(ellie_parser::syntax::types::Types::Integer(
                                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                            },
                                            raw: "1".to_string(),
                                            complete: true,
                                        },
                                    ),),
                                    value: Box::new(ellie_parser::syntax::types::Types::Char(
                                        ellie_parser::syntax::types::char_type::CharType {
                                            value: 'o',
                                            complete: true,
                                        },
                                    ),),
                                    key_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            1,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            2,
                                        ),
                                    },
                                    value_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            4,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            7,
                                        ),
                                    },
                                },
                                key_collected: true,
                                value_collected: false,
                            },
                            ellie_parser::syntax::types::collective_type::CollectiveEntryCollector {
                                data: ellie_parser::syntax::types::collective_type::CollectiveEntry {
                                    key: Box::new(ellie_parser::syntax::types::Types::Integer(
                                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                                    2,
                                                ),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                            },
                                            raw: "2".to_string(),
                                            complete: true,
                                        },
                                    ),),
                                    value: Box::new(ellie_parser::syntax::types::Types::Char(
                                        ellie_parser::syntax::types::char_type::CharType {
                                            value: 'k',
                                            complete: true,
                                        },
                                    ),),
                                    key_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            9,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            10,
                                        ),
                                    },
                                    value_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            12,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            15,
                                        ),
                                    },
                                },
                                key_collected: true,
                                value_collected: false,
                            }
                        ],
                    },
                },
            ),
        ),)
    }

    #[test]
    fn two_dimension_collective_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("{1: 'o', 2: {1: 'o', 2: 'k'}}"),
            ellie_parser::syntax::types::Types::Collective(
                ellie_parser::syntax::types::collective_type::CollectiveCollector {
                    complete: true,
                    at_comma: false,
                    data: ellie_parser::syntax::types::collective_type::Collective {
                        entries: vec![
                            ellie_parser::syntax::types::collective_type::CollectiveEntryCollector {
                                data: ellie_parser::syntax::types::collective_type::CollectiveEntry {
                                    key: Box::new(ellie_parser::syntax::types::Types::Integer(
                                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                            },
                                            raw: "1".to_string(),
                                            complete: true,
                                        },
                                    ),),
                                    value: Box::new(ellie_parser::syntax::types::Types::Char(
                                        ellie_parser::syntax::types::char_type::CharType {
                                            value: 'o',
                                            complete: true,
                                        },
                                    ),),
                                    key_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            1,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            2,
                                        ),
                                    },
                                    value_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            4,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            7,
                                        ),
                                    },
                                },
                                key_collected: true,
                                value_collected: false,
                            },
                            ellie_parser::syntax::types::collective_type::CollectiveEntryCollector {
                                data: ellie_parser::syntax::types::collective_type::CollectiveEntry {
                                    key: Box::new(ellie_parser::syntax::types::Types::Integer(
                                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(2),
                                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                            },
                                            raw: "2".to_string(),
                                            complete: true,
                                        },
                                    ),),
                                    value: Box::new(ellie_parser::syntax::types::Types::Collective(
                                        ellie_parser::syntax::types::collective_type::CollectiveCollector {
                                            complete: true,
                                            at_comma: false,
                                            data: ellie_parser::syntax::types::collective_type::Collective {
                                                entries: vec![
                                                    ellie_parser::syntax::types::collective_type::CollectiveEntryCollector {
                                                        data: ellie_parser::syntax::types::collective_type::CollectiveEntry {
                                                            key: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(1),
                                                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    },
                                                                    raw: "1".to_string(),
                                                                    complete: true,
                                                                },
                                                            ),),
                                                            value: Box::new(ellie_parser::syntax::types::Types::Char(
                                                                ellie_parser::syntax::types::char_type::CharType {
                                                                    value: 'o',
                                                                    complete: true,
                                                                },
                                                            ),),
                                                            key_pos: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    13,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    14,
                                                                ),
                                                            },
                                                            value_pos: ellie_core::defs::Cursor {
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
                                                        key_collected: true,
                                                        value_collected: false,
                                                    },
                                                    ellie_parser::syntax::types::collective_type::CollectiveEntryCollector {
                                                        data: ellie_parser::syntax::types::collective_type::CollectiveEntry {
                                                            key: Box::new(ellie_parser::syntax::types::Types::Integer(
                                                                ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                                                                    data: ellie_parser::syntax::types::integer_type::IntegerType {
                                                                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(2),
                                                                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                                                                    },
                                                                    raw: "2".to_string(),
                                                                    complete: true,
                                                                },
                                                            ),),
                                                            value: Box::new(ellie_parser::syntax::types::Types::Char(
                                                                ellie_parser::syntax::types::char_type::CharType {
                                                                    value: 'k',
                                                                    complete: true,
                                                                },
                                                            ),),
                                                            key_pos: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    21,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    22,
                                                                ),
                                                            },
                                                            value_pos: ellie_core::defs::Cursor {
                                                                range_start: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    24,
                                                                ),
                                                                range_end: ellie_core::defs::CursorPosition(
                                                                    0,
                                                                    27,
                                                                ),
                                                            },
                                                        },
                                                        key_collected: true,
                                                        value_collected: false,
                                                    }
                                                ],
                                            },
                                        },
                                    ),),
                                    key_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            9,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            10,
                                        ),
                                    },
                                    value_pos: ellie_core::defs::Cursor {
                                        range_start: ellie_core::defs::CursorPosition(
                                            0,
                                            12,
                                        ),
                                        range_end: ellie_core::defs::CursorPosition(
                                            0,
                                            28,
                                        ),
                                    },
                                },
                                key_collected: true,
                                value_collected: false,
                            }
                        ],
                    },
                },
            ),
        ),)
    }
}
