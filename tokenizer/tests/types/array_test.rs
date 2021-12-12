#[cfg(test)]
mod array_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::types::{Processor, TypeProcessor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 2] = [
        ("[2, 4]", 6140056764232043685),
        ("['2', \"4\"] || 1", 12834758856132739949),
    ];

    #[test]
    fn array_with_no_error() {
        fn process(input: &str) -> Option<u64> {
            let mut pos = defs::CursorPosition::default();
            let mut errors: Vec<error::Error> = Vec::new();
            let mut processor: TypeProcessor = TypeProcessor::default();
            let mut last_char = '\0';
            for letter_char in input.chars() {
                processor.iterate(&mut errors, pos, last_char, letter_char);
                pos.skip_char(1);
                last_char = letter_char;
            }

            if errors.is_empty() {
                let mut result_hash = DefaultHasher::new();
                format!("{:?}", processor).hash(&mut result_hash);
                Some(result_hash.finish())
            } else {
                None
            }
        }

        let mut has_err = false;

        for i in TESTS {
            has_err = match process(i.0) {
                Some(e) => e != i.1,
                None => true,
            };
        }
        assert!(!has_err);
    }
}
