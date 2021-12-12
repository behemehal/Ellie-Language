#[cfg(test)]
mod class_call_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::types::{Processor, TypeProcessor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 5] = [
        ("new Future<Array<int>>()", 12088462503636699572),
        (
            "new Future<Array<String, 12>>(123, 12)",
            7084790905372425830,
        ),
        ("new Future(1)", 8689931740247360668),
        ("new Future()", 13637466635196238734),
        ("new Future(1, 2)", 15914380915004135657),
    ];

    #[test]
    fn class_calls_with_no_error() {
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
