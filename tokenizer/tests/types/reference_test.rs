#[cfg(test)]
mod reference_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::types::TypeProcessor, processors::Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 8] = [
        ("1.test", 10284710370696261359),
        ("1.1.test", 396191547936676625),
        (".1.test", 17478063398168761554),
        ("'e'.test", 5375207223945722073),
        ("\"ellie\".test", 8628058702185603235),
        ("!.1.test", 425552375002479630),
        ("1 && 1.test", 1716969059804811584),
        ("ellie.is.awsome", 16755652306243190260)
    ];

    #[test]
    fn reference_with_no_error() {
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
                Some(e) => {
                    if e != i.1 {
                        panic!("?");
                    }
                    e != i.1
                },
                None => {
                    panic!("?");
                    true
                },
            };
        }
        assert!(!has_err);
    }
}
