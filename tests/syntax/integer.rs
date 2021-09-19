#[cfg(test)]
mod integer_tests {

    #[test]
    fn integer_collected_with_no_error() {
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
            }
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            123

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
        assert!(emulated_collector_data.data.value.is_integer());
        assert!(emulated_collector_data.data.value.is_type_complete());
    }

    /*
        #[test]
        fn integer_prototype_collected() {
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
            }
        );
            let mut emulated_collector_data =
                ellie_parser::syntax::variable::VariableCollector::default();
            let mut syntax_errors = vec![];
            emulated_collector_data.data.dynamic = true;
            let code = "

                123.len

            ";

    let content = code.chars().collect::<Vec<_>>();
    for i in 0..content.len() {
        let char = content[i];
        let letter_char = &char.to_string();
        let last_char = if i == 0 {
            "".to_owned()
        } else {
            content[i - 1].to_string()
        };
        let next_char = if i + 1 > content.len() - 1 {
            "".to_owned()
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
    fn integer_operators_collected() {
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
            }
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            1 == 0

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
        assert!(emulated_collector_data.data.value.is_type_complete());
        assert_eq!(syntax_errors.len(), 0);
        assert_eq!(emulated_collector_data.data.value.get_type(), "operator");
    }
}
