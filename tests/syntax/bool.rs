#[cfg(test)]
mod boolean_tests {

    #[test]
    fn boolean_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            true

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
        assert!(emulated_collector_data.data.value.is_bool());
        assert!(emulated_collector_data.data.value.is_type_complete());
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Bool(x) if x.value)
        );
    }

    /*
        #[test]
        fn bool_prototype_collected() {
            let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
            let mut emulated_collector_data =
                ellie_parser::syntax::variable::VariableCollector::default();
            let mut syntax_errors = vec![];
            emulated_collector_data.data.dynamic = true;
            let code = "

                false.len

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
            assert!(emulated_collector_data.data.value.is_type_complete());
            assert_eq!(emulated_collector_data.data.value.get_type(), "reference");
        }
            */

    #[test]
    fn bool_operators_collected() {
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

            false == true

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
        assert!(emulated_collector_data.data.value.is_type_complete());
        assert_eq!(syntax_errors.len(), 0);
        assert_eq!(emulated_collector_data.data.value.get_type(), "operator");
    }
}
