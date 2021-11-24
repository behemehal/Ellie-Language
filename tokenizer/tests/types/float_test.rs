#[cfg(test)]
mod float_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::types::TypeProcessor, processors::Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn float_with_no_error() {
        let code = "1.3";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: TypeProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 12120825843254719806);
    }

    #[test]
    fn dot_start_float_with_no_error() {
        let code = ".3";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: TypeProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 8118662713716061827);
    }
}
