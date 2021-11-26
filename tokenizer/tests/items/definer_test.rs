#[cfg(test)]
mod definer_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{processors::types::Processor, syntax::items::definers};
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn collective_with_no_error() {
        let code = "{int, int}";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 4827217106507216039);
    }

    #[test]
    fn nullable_collective_collected_with_no_error() {
        let code = "?{int, int}";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 11664837364969987321);
    }

    #[test]
    fn array_collected_with_no_error() {
        let code = "[int, 3]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 6034828774517186636);
    }

    #[test]
    fn variable_array_collected_with_no_error() {
        let code = "[int, refer]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 16571363607244013864);
    }

    #[test]
    fn vector_collected_with_no_error() {
        let code = "[int, *]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 16066019330082801417);
    }

    #[test]
    fn nullable_vector_collected_with_no_error() {
        let code = "?[int, *]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 16100666307933433542);
    }

    #[test]
    fn cloak_collected_with_no_error() {
        let code = "(int, int)";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 7520681682715042689);
    }

    #[test]
    fn nullable_cloak_collected_with_no_error() {
        let code = "?(int, int)";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 7952147402688194864);
    }

    #[test]
    fn generic_collected_with_no_error() {
        let code = "int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 1629535002954895172);
    }

    #[test]
    fn nullable_generic_collected_with_no_error() {
        let code = "?int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 17886873526644393922);
    }

    #[test]
    fn no_param_no_return_function_collected_with_no_error() {
        let code = "@()";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 17128602177685293589);
    }

    #[test]
    fn no_param_function_collected_with_no_error() {
        let code = "@():string";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 10184215308945865826);
    }

    #[test]
    fn no_return_function_collected_with_no_error() {
        let code = "@(string)";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 13817130764464377317);
    }

    #[test]
    fn function_collected_with_no_error() {
        let code = "@(string, int):int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 17261212020546398938);
    }

    #[test]
    fn nullable_function_collected_with_no_error() {
        let code = "?@(string, int):int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = definers::DefinerCollector::default();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 4604979302673474658);
    }
}
