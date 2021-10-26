#[cfg(test)]
mod negative_tests {

    #[test]
    fn int_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("!123"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::Integer(
                        ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                            data: ellie_parser::syntax::types::integer_type::IntegerType {
                                value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(
                                    123
                                ),
                                rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                            },
                            raw: "123".to_owned(),
                            complete: true
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn float_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("!0.1"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::Float(
                        ellie_parser::syntax::types::float_type::FloatTypeCollector {
                            data: ellie_parser::syntax::types::float_type::FloatType {
                                value: ellie_parser::syntax::types::float_type::FloatSize::F32(0.1,),
                                rtype: ellie_parser::syntax::types::float_type::FloatTypes::F32,
                                raw: "".to_owned(),
                            },
                            base: "0".to_owned(),
                            point: "1".to_owned(),
                            at_point: true,
                            complete: true,
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn bool_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("!true"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::Bool(
                        ellie_parser::syntax::types::bool_type::BoolType {
                            value: true,
                            raw: "true".to_owned(),
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn string_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("!\"ok\""),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::String(
                        ellie_parser::syntax::types::string_type::StringTypeCollector {
                            data: ellie_parser::syntax::types::string_type::StringType {
                                value: "ok".to_owned(),
                                comma_start_pos: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(0, 0),
                                    range_end: ellie_core::defs::CursorPosition(0, 1)
                                },
                                comma_end_pos: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(0, 3),
                                    range_end: ellie_core::defs::CursorPosition(0, 4)
                                },
                                value_pos: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(0, 2),
                                    range_end: ellie_core::defs::CursorPosition(0, 3)
                                },
                            },
                            complete: true,

                            ..Default::default()
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn char_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("!\'o'"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::Char(
                        ellie_parser::syntax::types::char_type::CharType {
                            value: 'o',
                            complete: true,
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn variable_value_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct(
            ellie_engine::test_utils::emulate_value_processor("!test"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::VariableType(
                        ellie_parser::syntax::types::variable_type::VariableTypeCollector {
                            data: ellie_parser::syntax::types::variable_type::VariableType {
                                value: "test".to_owned(),
                                pos: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition(0, 1),
                                    range_end: ellie_core::defs::CursorPosition(0, 5)
                                }
                            },
                            value_exists: false,
                            value_complete: false,
                        },
                    )),
                },
            ),
        ));
    }
}
