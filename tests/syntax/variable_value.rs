#[cfg(test)]
mod variable_value_tests {

    #[test]
    fn variable_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions {
                path: "<virtual>".to_string(),
                functions: true,
                break_on_error: true,
                loops: true,
                enums: true,
                classes: true,
                getters: true,
                setters: true,
                conditions: true,
                global_variables: true,
                line_ending: "\n\r".to_string(),
                dynamics: true,
                collectives: true,
                variables: true,
                import_std: false,
                constants: true,
                parser_type: ellie_core::defs::ParserType::RawParser,
                allow_import: true,
            },
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

        let mut content = code.split("").collect::<Vec<_>>();
        content.remove(0);
        content.remove(content.len() - 1);
        for i in 0..content.len() {
            let char = content[i].chars().nth(0).unwrap_or('\0');
            let letter_char = content[i];
            let last_char = if i == 0 { "" } else { content[i - 1] };
            let next_char = if i + 1 > content.len() - 1 {
                ""
            } else {
                content[i + 1]
            };
            if char == '\n' || char == '\r' {
                continue;
            }
            ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                &mut syntax_errors,
                letter_char,
                next_char,
                last_char,
            );
        }

        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::VariableType(x) if x.data.value == "test")
        );
    }

    #[test]
    fn variable_comparison_collected_with_no_error() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions {
                path: "<virtual>".to_string(),
                functions: true,
                break_on_error: true,
                loops: true,
                enums: true,
                classes: true,
                getters: true,
                setters: true,
                conditions: true,
                global_variables: true,
                line_ending: "\n\r".to_string(),
                dynamics: true,
                collectives: true,
                variables: true,
                import_std: false,
                constants: true,
                parser_type: ellie_core::defs::ParserType::RawParser,
                allow_import: true,
            },
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

        let mut content = code.split("").collect::<Vec<_>>();
        content.remove(0);
        content.remove(content.len() - 1);
        for i in 0..content.len() {
            let char = content[i].chars().nth(0).unwrap_or('\0');
            let letter_char = content[i];
            let last_char = if i == 0 { "" } else { content[i - 1] };
            let next_char = if i + 1 > content.len() - 1 {
                ""
            } else {
                content[i + 1]
            };
            if char == '\n' || char == '\r' {
                continue;
            }
            if char == '\n' || char == '\r' {
                continue;
            }
            ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                &mut syntax_errors,
                letter_char,
                next_char,
                last_char,
            );
            emulated_parser.pos.1 += 1;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Operator(x) if x.data.first.as_variable_type().unwrap().data.value == "test" && x.data.second.as_variable_type().unwrap().data.value == "test_second" && x.operator_collect == "==" && x.data.operator == ellie_parser::syntax::types::operator_type::Operators::ComparisonType(ellie_parser::syntax::types::comparison_type::ComparisonOperators::Equal))
        );
    }

    #[test]
    fn logical_equal_collected_with_no_error() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions {
                path: "<virtual>".to_string(),
                functions: true,
                break_on_error: true,
                loops: true,
                enums: true,
                classes: true,
                getters: true,
                setters: true,
                conditions: true,
                global_variables: true,
                line_ending: "\n\r".to_string(),
                dynamics: true,
                collectives: true,
                variables: true,
                import_std: false,
                constants: true,
                parser_type: ellie_core::defs::ParserType::RawParser,
                allow_import: true,
            },
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

        let mut content = code.split("").collect::<Vec<_>>();
        content.remove(0);
        content.remove(content.len() - 1);
        for i in 0..content.len() {
            let char = content[i].chars().nth(0).unwrap_or('\0');
            let letter_char = content[i];
            let last_char = if i == 0 { "" } else { content[i - 1] };
            let next_char = if i + 1 > content.len() - 1 {
                ""
            } else {
                content[i + 1]
            };
            if char == '\n' || char == '\r' {
                continue;
            }
            if char == '\n' || char == '\r' {
                continue;
            }
            ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                &mut syntax_errors,
                letter_char,
                next_char,
                last_char,
            );
            emulated_parser.pos.1 += 1;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Operator(x) if x.data.first.as_variable_type().unwrap().data.value == "test" && x.data.second.as_variable_type().unwrap().data.value == "test_second" && x.operator_collect == "&&" && x.data.operator == ellie_parser::syntax::types::operator_type::Operators::LogicalType(ellie_parser::syntax::types::logical_type::LogicalOperators::And))
        );
    }

    #[test]
    fn arithmetic_equal_collected_with_no_error() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions {
                path: "<virtual>".to_string(),
                functions: true,
                break_on_error: true,
                loops: true,
                enums: true,
                classes: true,
                getters: true,
                setters: true,
                conditions: true,
                global_variables: true,
                line_ending: "\n\r".to_string(),
                dynamics: true,
                collectives: true,
                variables: true,
                import_std: false,
                constants: true,
                parser_type: ellie_core::defs::ParserType::RawParser,
                allow_import: true,
            },
        );
        let mut emulated_collector_data = ellie_parser::syntax::variable::VariableCollector {
            ignore_existence: true,
            ..Default::default()
        };
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "
            test + test_second
        ";

        let mut content = code.split("").collect::<Vec<_>>();
        content.remove(0);
        content.remove(content.len() - 1);
        for i in 0..content.len() {
            let char = content[i].chars().nth(0).unwrap_or('\0');
            let letter_char = content[i];
            let last_char = if i == 0 { "" } else { content[i - 1] };
            let next_char = if i + 1 > content.len() - 1 {
                ""
            } else {
                content[i + 1]
            };
            if char == '\n' || char == '\r' {
                continue;
            }

            ellie_parser::processors::value_processor::collect_value(
                emulated_parser.clone(),
                &mut emulated_collector_data,
                &mut syntax_errors,
                letter_char,
                next_char,
                last_char,
            );
            emulated_parser.pos.1 += 1;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Operator(x) if x.data.first.as_variable_type().unwrap().data.value == "test" && x.data.second.as_variable_type().unwrap().data.value == "test_second" && x.operator_collect == "+" && x.data.operator == ellie_parser::syntax::types::operator_type::Operators::ArithmeticType(ellie_parser::syntax::types::arithmetic_type::ArithmeticOperators::Addition))
        );
    }
}
