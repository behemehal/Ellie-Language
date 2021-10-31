#[cfg(test)]
mod arithmetic_operator_tests {

    #[test]
    fn addition_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct_operator(
            ellie_engine::test_utils::emulate_value_processor_operator("123 + 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(
                    ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Addition,
                ),
                ..Default::default()
            }
        ));
    }

    #[test]
    fn subtraction_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct_operator(
            ellie_engine::test_utils::emulate_value_processor_operator("123 - 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(
                    ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Subtraction,
                ),
                ..Default::default()
            }
        ));
    }

    #[test]
    fn multiplication_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct_operator(
            ellie_engine::test_utils::emulate_value_processor_operator("123 * 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(
                    ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Multiplication,
                ),
                ..Default::default()
            }
        ));
    }

    #[test]
    fn exponentiation_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct_operator(
            ellie_engine::test_utils::emulate_value_processor_operator("123 ** 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(
                    ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Exponentiation,
                ),
                ..Default::default()
            }
        ));
    }

    #[test]
    fn division_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct_operator(
            ellie_engine::test_utils::emulate_value_processor_operator("123 / 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(
                    ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Division,
                ),
                ..Default::default()
            }
        ));
    }

    #[test]
    fn modulus_collected_with_no_error() {
        assert!(ellie_engine::test_utils::has_no_error_and_correct_operator(
            ellie_engine::test_utils::emulate_value_processor_operator("123 % 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerTypeCollector {
                        data: ellie_parser::syntax::types::integer_type::IntegerType {
                            value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                            rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        },
                        raw: "123".to_owned(),
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(
                    ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Modulus,
                ),
                ..Default::default()
            }
        ));
    }
}
