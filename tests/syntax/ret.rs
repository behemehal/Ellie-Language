#[cfg(test)]
mod ret_tests {

    #[test]
    fn ret_collected_with_no_error() {
        let code = "
            ret \"ok\";
        ";
        let emulated_parser = ellie_parser::parser::Parser::new(
            code.to_string(),
            |_, _, _| ellie_parser::parser::ResolvedImport::default(),
            |_| {},
            ellie_core::defs::ParserOptions::default(),
        );
        let parsed = emulated_parser.map();
        assert!(
            matches!(parsed.parsed.items[0].clone(), ellie_parser::parser::Collecting::Ret(x) if x.value.get_type() == "string")
                && parsed.syntax_errors.len() == 0
        );
    }
}
