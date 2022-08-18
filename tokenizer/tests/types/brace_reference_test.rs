#[cfg(test)]
mod brace_reference_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::{types::TypeProcessor, Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 7] = [
        ("1[1]", 16767165626425121393),
        (".1[1]", 5601372900266423814),
        ("1.2[1]", 1918174091482911851),
        ("'2'[1]", 17852188756335713310),
        ("\"2\"[1]", 8718289645514959000),
        ("test[1]", 12215492312650117402),
        ("test[test[1]]", 2429100454996154895),
    ];

    #[test]
    fn brace_with_no_error() {
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
