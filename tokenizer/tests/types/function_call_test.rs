#[cfg(test)]
mod function_call_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::types::{Processor, TypeProcessor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 7] = [
        ("1(1)", 16894146993788279578),
        (".1(.1)", 4312058473575010974),
        ("1.2(1.2, .1)", 3369295565294702178),
        ("'2'(1.2)", 4363779510867426687),
        ("\"2\"(1, 't')", 2024870518245809355),
        ("test(1)", 16092232105369503417),
        ("test.test(test[1])", 13180962598878762174),
    ];

    #[test]
    fn function_call_with_no_error() {
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
