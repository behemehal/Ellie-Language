#[cfg(test)]
mod float_tests {

    #[test]
    fn dot_start_float_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::default();
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2

        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
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
        assert!(emulated_collector_data.data.value.is_float());
        assert!(emulated_collector_data.data.value.is_type_complete());
    }

    #[test]
    fn float_collected_with_no_error() {
        let emulated_parser = ellie_parser::parser::Parser::default();
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            0.2

        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
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
        assert!(emulated_collector_data.data.value.is_float());
        assert!(emulated_collector_data.data.value.is_type_complete());
    }

    /*
    #[test]
    fn dot_start_float_prototype_collected() {
        let emulated_parser = ellie_parser::parser::Parser::default();
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2.len

        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
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
        let emulated_parser = ellie_parser::parser::Parser::default();
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            0.2.len

        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
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
    fn dot_start_float_operators_collected() {
        let emulated_parser = ellie_parser::parser::Parser::default();
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2 == .2

        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
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
    }

    #[test]
    fn float_operators_collected() {
        let emulated_parser = ellie_parser::parser::Parser::default();
        let mut emulated_collector_data =
            ellie_parser::syntax::variable::VariableCollector::default();
        let mut syntax_errors = vec![];
        emulated_collector_data.data.dynamic = true;
        let code = "

            .2 == .2

        ";

        for (index, char) in code.chars().enumerate() {
            if char == '\n' || char == '\r' {
                continue;
            }

            let letter_char = &char.to_string();
            let last_char =
                &ellie_core::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char =
                &ellie_core::utils::get_letter(code.to_string(), index, true).to_owned();
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
    */
}
