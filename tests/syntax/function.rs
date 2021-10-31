#[cfg(test)]
mod function_tests {

    #[test]
    fn private_function_collected_with_no_error() {
        let code = "pri fn test() {}";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "void" && !x.data.public)
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn public_function_collected_with_no_error() {
        let code = "pub fn test() {}";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "void" && x.data.public)
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn function_collected_with_no_error() {
        let code = "
            fn test() {}
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "void" && !x.data.public)
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn function_return_type_collected_with_no_error() {
        let code = "
            fn test() > string {
                ret \"test\";
            }
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
                import_std: true,
                constants: true,
                parser_type: ellie_core::defs::ParserType::RawParser,
                allow_import: true,
            },
        );
        let parsed = emulated_parser.map();
        let last = parsed.parsed.items[parsed.parsed.items.len() - 1].clone();
        assert!(
            matches!(last, ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "string" && !x.data.public)
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn function_params_collected_with_no_error() {
        let code = "
            class string {} //Emulate string class
            fn test(test: string) {}
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[1].clone(), ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "void" && !x.data.public && x.data.parameters[0].name == "test" && x.data.parameters[0].rtype.raw_name() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }

    #[test]
    fn function_params_collected_with_typedef_error() {
        /*
            Emulate if string is unknown at compile time
        */

        let code = "
            fn test(test: string) {}
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "void" && !x.data.public && x.data.parameters[0].name == "test" && x.data.parameters[0].rtype.raw_name() == "string")
                && matches!(parsed.syntax_errors[0].clone(), x if x.code == 5 && x.builded_message.fields[0].value == "string")
        );
    }

    #[test]
    fn function_return_type_collected_with_typedef_error() {
        /*
            Emulate if string is unknown at compile time
        */
        let code = "
            fn test() > string {}
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
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
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Function(x) if x.data.name == "test" && x.data.return_type.raw_name() == "string" && !x.data.public)
                && matches!(parsed.syntax_errors[0].clone(), x if x.code == 5 && x.builded_message.fields[0].value == "string")
        );
    }
}
