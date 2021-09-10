#[cfg(test)]
mod float_tests {

    #[test]
    fn dot_start_float_collected_with_no_error() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2

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
        assert!(emulated_collector_data.data.value.is_float());
        assert!(emulated_collector_data.data.value.is_type_complete());
    }

    #[test]
    fn float_collected_with_no_error() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            0.2

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
        assert!(emulated_collector_data.data.value.is_float());
        assert!(emulated_collector_data.data.value.is_type_complete());
    }

    /*
        #[test]
        fn dot_start_float_prototype_collected() {
            let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
            let mut emulated_collector_data =
                ellie_parser::syntax::variable::VariableCollector::default();
            let mut syntax_errors = vec![];
            emulated_collector_data.data.dynamic = true;
            let code = "

                .2.len

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

        #[test]
        fn float_prototype_collected() {
            let emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
            let mut emulated_collector_data =
                ellie_parser::syntax::variable::VariableCollector::default();
            let mut syntax_errors = vec![];
            emulated_collector_data.data.dynamic = true;
            let code = "

                0.2.len

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
    fn dot_start_float_operators_collected() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2 == .2

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
    }

    #[test]
    fn float_operators_collected() {
        let mut emulated_parser = ellie_parser::parser::Parser::new(
            "".to_owned(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors: Vec<ellie_core::error::Error> = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2 == .2

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
