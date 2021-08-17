#[cfg(test)]
mod negative_tests {

    #[test]
    fn int_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("!123"),
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
                            raw: "123".to_string(),
                            complete: true
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn float_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("!0.1"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::Float(
                        ellie_parser::syntax::types::float_type::FloatTypeCollector {
                            data: ellie_parser::syntax::types::float_type::FloatType {
                                value: ellie_parser::syntax::types::float_type::FloatSize::F32(0.1,),
                                rtype: ellie_parser::syntax::types::float_type::FloatTypes::F32,
                                raw: "".to_string(),
                            },
                            base: "0".to_string(),
                            point: "1".to_string(),
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
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("!true"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::Bool(
                        ellie_parser::syntax::types::bool_type::BoolType {
                            value: true,
                            raw: "true".to_string(),
                        },
                    )),
                },
            ),
        ));
    }

    #[test]
    fn string_collected_with_no_error() {
        println!(
            "{:#?}",
            ellie_lang::test_utils::emulate_value_processor("!\"ok\"")
        );
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("!\"ok\""),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::String(
                        ellie_parser::syntax::types::string_type::StringTypeCollector {
                            data: ellie_parser::syntax::types::string_type::StringType {
                                value: "ok".to_string(),
                                ..Default::default()
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
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("!\'o'"),
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
        assert!(ellie_lang::test_utils::has_no_error_and_correct(
            ellie_lang::test_utils::emulate_value_processor("!test"),
            ellie_parser::syntax::types::Types::Negative(
                ellie_parser::syntax::types::negative_type::Negative {
                    value: Box::new(ellie_parser::syntax::types::Types::VariableType(
                        ellie_parser::syntax::types::variable_type::VariableTypeCollector {
                            data: ellie_parser::syntax::types::variable_type::VariableType {
                                value: "test".to_string(),
                                pos: ellie_core::defs::Cursor {
                                    range_start: ellie_core::defs::CursorPosition::default(),
                                    range_end: ellie_core::defs::CursorPosition::default()
                                }
                            },
                            value_complete: false,
                        },
                    )),
                },
            ),
        ));
    }
}
