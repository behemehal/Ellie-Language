#[cfg(test)]
mod string_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::Processor, syntax::types::string_type};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn escape_char_with_no_error() {
        let code = "\"\\\"\"";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: string_type::StringTypeCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 6305661421804005034);
    }

    #[test]
    fn string_with_no_error() {
        let code = "\"ellie\"";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: string_type::StringTypeCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 5232957081096344953);
    }

    #[test]
    fn wrong_escape_char_with_error() {
        let code = "\"\\ellie\"";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: string_type::StringTypeCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", errors).hash(&mut result_hash);
        assert!(result_hash.finish() == 12570213819031701426);
    }
}
