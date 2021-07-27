#[cfg(test)]
mod logical_operator_tests {

    #[test]
    fn and_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct_operator(
            ellie_lang::test_utils::emulate_value_processor_operator("123 && 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerType {
                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                        raw: "123".to_string(),
                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerType {
                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                        raw: "123".to_string(),
                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::LogicalType(
                    ellie_parser::syntax::types::logical_type::LogicalOperators::And,
                ),
            }
        ));
    }
    #[test]
    fn or_collected_with_no_error() {
        assert!(ellie_lang::test_utils::has_no_error_and_correct_operator(
            ellie_lang::test_utils::emulate_value_processor_operator("123 || 123"),
            ellie_parser::syntax::types::operator_type::OperatorType {
                cloaked: false,
                first: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerType {
                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                        raw: "123".to_string(),
                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        complete: true,
                    },
                )),
                second: Box::new(ellie_parser::syntax::types::Types::Integer(
                    ellie_parser::syntax::types::integer_type::IntegerType {
                        value: ellie_parser::syntax::types::integer_type::IntegerSize::I8(123),
                        raw: "123".to_string(),
                        rtype: ellie_parser::syntax::types::integer_type::IntegerTypes::I8,
                        complete: true,
                    },
                )),
                operator: ellie_parser::syntax::types::operator_type::Operators::LogicalType(
                    ellie_parser::syntax::types::logical_type::LogicalOperators::Or,
                ),
            }
        ));
    }
}
