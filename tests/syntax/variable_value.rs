#[cfg(test)]
mod variable_value_tests {

    #[test]
    fn variable_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector {
            ignore_existence: true,
            ..Default::default()
        };
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            test

        ";

        let content = code.chars().collect::<Vec<_>>();
        for i in 0..content.len() {
            let char = content[i];
            let letter_char = &char.to_string();
            let last_char = if i == 0 {
                "".to_string()
            } else {
                content[i - 1].to_string()
            };
            let next_char = if i + 1 > content.len() - 1 {
                "".to_string()
            } else {
                content[i + 1].to_string()
            };
            if char == '\n' || char == '\r' {
                continue;
            }
            let itered = ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
            );

            for error in itered.errors {
                syntax_errors.push(error);
            }
            emulated_collector_data = itered.itered_data;
        }

        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::VariableType(x) if x.data.value == "test")
        );
    }

    #[test]
    fn variable_comparison_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector {
            ignore_existence: true,
            ..Default::default()
        };
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            test == test_second

        ";

        let content = code.chars().collect::<Vec<_>>();
        for i in 0..content.len() {
            let char = content[i];
            let letter_char = &char.to_string();
            let last_char = if i == 0 {
                "".to_string()
            } else {
                content[i - 1].to_string()
            };
            let next_char = if i + 1 > content.len() - 1 {
                "".to_string()
            } else {
                content[i + 1].to_string()
            };
            if char == '\n' || char == '\r' {
                continue;
            }
            let itered = ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
            );

            for error in itered.errors {
                syntax_errors.push(error);
            }
            emulated_collector_data = itered.itered_data;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Operator(x) if x.data.first.as_variable_type().unwrap().data.value == "test" && x.data.second.as_variable_type().unwrap().data.value == "test_second" && x.operator_collect == "==" && x.data.operator == ellie_parser::syntax::types::operator_type::Operators::ComparisonType(ellie_parser::syntax::types::comparison_type::ComparisonOperators::Equal))
        );
    }

    #[test]
    fn logical_equal_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector {
            ignore_existence: true,
            ..Default::default()
        };
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            test && test_second

        ";

        let content = code.chars().collect::<Vec<_>>();
        for i in 0..content.len() {
            let char = content[i];
            let letter_char = &char.to_string();
            let last_char = if i == 0 {
                "".to_string()
            } else {
                content[i - 1].to_string()
            };
            let next_char = if i + 1 > content.len() - 1 {
                "".to_string()
            } else {
                content[i + 1].to_string()
            };
            if char == '\n' || char == '\r' {
                continue;
            }
            let itered = ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
            );

            for error in itered.errors {
                syntax_errors.push(error);
            }
            emulated_collector_data = itered.itered_data;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Operator(x) if x.data.first.as_variable_type().unwrap().data.value == "test" && x.data.second.as_variable_type().unwrap().data.value == "test_second" && x.operator_collect == "&&" && x.data.operator == ellie_parser::syntax::types::operator_type::Operators::LogicalType(ellie_parser::syntax::types::logical_type::LogicalOperators::And))
        );
    }

    #[test]
    fn arithmetic_equal_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector {
            ignore_existence: true,
            ..Default::default()
        };
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "
            test + test_second
        ";

        let content = code.chars().collect::<Vec<_>>();
        for i in 0..content.len() {
            let char = content[i];
            let letter_char = &char.to_string();
            let last_char = if i == 0 {
                "".to_string()
            } else {
                content[i - 1].to_string()
            };
            let next_char = if i + 1 > content.len() - 1 {
                "".to_string()
            } else {
                content[i + 1].to_string()
            };
            if char == '\n' || char == '\r' {
                continue;
            }

            let itered = ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
            );

            for error in itered.errors {
                syntax_errors.push(error);
            }
            emulated_collector_data = itered.itered_data;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Operator(x) if x.data.first.as_variable_type().unwrap().data.value == "test" && x.data.second.as_variable_type().unwrap().data.value == "test_second" && x.operator_collect == "+" && x.data.operator == ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Addition))
        );
    }
}
