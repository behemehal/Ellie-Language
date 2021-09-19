#[cfg(test)]
mod char_tests {

    #[test]
    fn char_collected_with_no_error() {
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
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            \'c\'

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
        assert!(
            matches!(emulated_collector_data.data.value, ellie_parser::syntax::types::Types::Char(x) if x.value == 'c')
        );
    }

    #[test]
    fn char_operators_collected() {
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
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            \'t\' == \'t\'

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
