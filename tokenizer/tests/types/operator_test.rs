#[cfg(test)]
mod operator_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::processors::types::{TypeProcessor, Processor};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };
    const TESTS: [(&str, u64); 22] = [
        ("1 == 1", 1572873490758581171),
        ("1 != 1", 9413585100029929209),
        ("1 > 1", 13711150573099223623),
        ("1 < 1", 12676758993113139150),
        ("1 >= 1", 6824084718630420199),
        ("1 <= 1", 12415793322580424519),
        ("1 && 1", 4407946824911942592),
        ("1 || 1", 18140830169173162077),
        ("1 + 1", 3627442587398136094),
        ("1 - 1", 7939643668105705876),
        ("1 * 1", 10349208660342430509),
        ("1 / 1", 6922215629933148984),
        ("1 % 1", 9652318629991548007),
        ("1 = 1", 2373092736875450081),
        ("1 += 1", 7516030630301770133),
        ("1 -= 1", 418824619721974229),
        ("1 *= 1", 310090996770967782),
        ("1 /= 1", 2218490824804697390),
        ("1 %= 1", 3733652645927787300),
        ("1 **= 1", 1849410128329509803),
        ("1 + 1 * 2", 8914537564132841071),
        ("1 + 1 * 2 * 2", 9203198762435174590),
    ];

    #[test]
    fn operator_with_no_error() {
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
