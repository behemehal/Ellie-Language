#[cfg(test)]
mod definer_tests {
    use ellie_core::{defs, error};
    use ellie_tokenizer::{
        processors::{items::definer_processor, Processor},
        syntax::items::definers,
    };
    use std::{
        collections::hash_map::DefaultHasher,
        hash::{Hash, Hasher},
    };

    #[test]
    fn vector_collective_with_no_error() {
        let code = "{int, int}";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 9256950456458196198);
    }

    #[test]
    fn nullable_collective_collected_with_no_error() {
        let code = "?{int, int}";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 1456385834464223876);
    }

    #[test]
    fn future_collective_collected_with_no_error() {
        let code = ">{int, int}";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 285075723699432689);
    }

    #[test]
    fn vector_collected_with_no_error() {
        let code = "[int, *]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 1717801901622985198);
    }

    #[test]
    fn nullable_vector_collected_with_no_error() {
        let code = "?[int, *]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 2095084249452940953);
    }

    #[test]
    fn future_vector_collected_with_no_error() {
        let code = ">[int, *]";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 10061816298405055815);
    }

    #[test]
    fn cloak_collected_with_no_error() {
        let code = "(>int, int)";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 8269887764424291153);
    }

    #[test]
    fn nullable_cloak_collected_with_no_error() {
        let code = "?(>int, int)";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 13448889993066499606);
    }

    #[test]
    fn future_cloak_collected_with_no_error() {
        let code = ">(>int, int)";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }

        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 14203049144237276059);
    }

    #[test]
    fn generic_collected_with_no_error() {
        let code = "int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 12618347469620875205);
    }

    #[test]
    fn nullable_generic_collected_with_no_error() {
        let code = "?int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 11751167107972902825);
    }

    #[test]
    fn future_generic_collected_with_no_error() {
        let code = ">int";
        let mut pos = defs::CursorPosition::default();
        let mut errors: Vec<error::Error> = Vec::new();
        let mut processor: definers::DefinerCollector = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(&mut errors, pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        let mut result_hash = DefaultHasher::new();
        format!("{:?}", processor.definer_type).hash(&mut result_hash);
        assert!(errors.is_empty() && result_hash.finish() == 12752362452955238480);
    }
}
