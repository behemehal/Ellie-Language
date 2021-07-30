#[cfg(test)]
mod variable_tests {

    #[test]
    fn private_variable_collected_with_no_error() {
        let code = "
            class string {}
            pri v test : string = \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_| ellie_parser::parser::ResolvedImport::default(),
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[1].clone(), ellie_parser::parser::Collecting::Variable(x) if x.data.name == "test" && !x.data.dynamic && !x.data.constant && !x.data.public && x.data.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn public_variable_collected_with_no_error() {
        let code = "
            class string {}
            pub v test : string = \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_| ellie_parser::parser::ResolvedImport::default(),
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[1].clone(), ellie_parser::parser::Collecting::Variable(x) if x.data.name == "test" && !x.data.dynamic && !x.data.constant && x.data.public && x.data.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn variable_collected_with_no_error() {
        let code = "
            class string {}
            v test : string = \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_| ellie_parser::parser::ResolvedImport::default(),
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[1].clone(), ellie_parser::parser::Collecting::Variable(x) if x.data.name == "test" && !x.data.dynamic && !x.data.constant && !x.data.public && x.data.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn constant_collected_with_no_error() {
        let code = "
            class string {}
            c test : string = \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_| ellie_parser::parser::ResolvedImport::default(),
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[1].clone(), ellie_parser::parser::Collecting::Variable(x) if x.data.name == "test" && !x.data.dynamic && x.data.constant && !x.data.public && x.data.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn dynamic_collected_with_correct_type_and_no_error() {
        let code = "
            class string {}
            d test= \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_| ellie_parser::parser::ResolvedImport::default(),
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[1].clone(), ellie_parser::parser::Collecting::Variable(x) if x.data.name == "test" && x.data.dynamic && !x.data.constant && !x.data.public && x.data.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn variable_collected_with_typedef_error() {
        let code = "
            v test : string = \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_| ellie_parser::parser::ResolvedImport::default(),
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Variable(x) if x.data.name == "test" && !x.data.dynamic && !x.data.constant && !x.data.public && x.data.value.get_type() == "string")
                && matches!(parsed.syntax_errors[0].clone(), x if x.code == 5 && x.builded_message.fields[0].value == "string")
        );
    }
}
