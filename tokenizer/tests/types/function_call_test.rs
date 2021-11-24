#[cfg(test)]
mod function_call_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::types::TypeProcessor, processors::Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 7] = [
        ("1(1)", 9753756198596513807),
        (".1(.1)", 11706485926454468701),
        ("1.2(1.2, .1)", 8360476234598111572),
        ("'2'(1.2)", 230001760650757384),
        ("\"2\"(1, 't')", 5343104436414559361),
        ("test(1)", 10407447683126159163),
        ("test.test(test[1])", 13337036509974447290),
    ];

    #[test]
    fn function_call_with_no_error() {
        fn process(input: &str) -> Option<u64> {
            let mut pos = defs::CursorPosition::default();
            let mut errors: Vec<error::Error> = Vec::new();
            let mut processor: TypeProcessor = Processor::new();
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
