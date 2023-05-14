#[cfg(test)]
mod string_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::Processor, syntax::types::string_type};

    #[test]
    fn string_with_no_error() {
        let code = "\"ellie\"";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: string_type::StringTypeCollector =
            string_type::StringTypeCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.1 += 1;
            last_char = letter_char;
        }
        assert!(errors.is_empty() && processor.data.value == "ellie");
    }

    #[test]
    fn wrong_escape_char_with_error() {
        let code = "\"\\ellie\"";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: string_type::StringTypeCollector =
            string_type::StringTypeCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.1 += 1;
            last_char = letter_char;
        }
        assert!(
            errors.len() != 0
                && errors[0].code == 0
                && errors[0].builded_message.fields[0].value == "e"
        );
    }
}
