use ellie;

#[cfg(test)]
mod test {

    #[test]
    fn string_collected_with_no_error() {
        let pos = ellie::mapper::defs::CursorPosition(0, 0);
        let code = "\"merhaba\"";
        let mut emulated_collector_data = ellie::syntax::variable::VariableCollector::default();
        let mut syntax_errors= vec![];
        
        for (index, char) in code.chars().enumerate() {
            let letter_char = &char.to_string();
            let last_char = &ellie::utils::get_letter(code.to_string(), index, false).to_owned();
            let next_char = &ellie::utils::get_letter(code.to_string(), index, true).to_owned();
            let itered = ellie::processors::value_processor::collect(
                &mut emulated_collector_data,
                letter_char,
                next_char.to_string(),
                last_char.to_string(),
                pos,
            );

            for error in itered.errors {
                syntax_errors.push(error)
            }

            emulated_collector_data = itered.itered_data;
        }
        assert_eq!(syntax_errors.len(), 0);
        assert!(emulated_collector_data.data.value.is_string());
    }
}
