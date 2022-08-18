#[cfg(test)]
mod float_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::{types::TypeProcessor, Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn float_with_no_error() {
        let code = "1.3";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: TypeProcessor = TypeProcessor::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.1 += 1;
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 3996366367111077397);
    }

    #[test]
    fn dot_start_float_with_no_error() {
        let code = ".3";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: TypeProcessor = TypeProcessor::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.1 += 1;
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 9786095407636100917);
    }
}
