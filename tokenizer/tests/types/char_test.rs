#[cfg(test)]
mod char_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::Processor, syntax::types::char_type};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn escape_char_with_no_error() {
        let code = "'\0'";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: char_type::CharType = char_type::CharType::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.1 += 1;
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 958559777172932442);
    }

    #[test]
    fn char_with_no_error() {
        let code = "'e'";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: char_type::CharType = char_type::CharType::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.1 += 1;
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 1701879649747228875);
    }
}
