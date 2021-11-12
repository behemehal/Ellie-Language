#[cfg(test)]
mod integer_tests {
    use ellie_core::defs;
    use ellie_tokenizer::processors::{types::integer_processor, Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn integer_with_no_error() {
        let code = "123";
        let mut pos = defs::CursorPosition::default();
        let mut processor: integer_processor::IntegerProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(!processor.has_error() && result_hash.finish() == 17750485041522175744);
    }

    #[test]
    fn negative_integer_with_no_error() {
        let code = "-123";
        let mut pos = defs::CursorPosition::default();
        let mut processor: integer_processor::IntegerProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(!processor.has_error() && result_hash.finish() == 2695524581536680219);
    }
}
