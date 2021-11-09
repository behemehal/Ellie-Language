use ellie_tokenizer::processors::{items::definer_processor, Processor};

#[cfg(test)]
mod definer_tests {
    #[test]
    fn cloak_collected_with_no_error() {
        let code = "int";
        let mut pos = defs::CursorPosition::default();
        let mut processor: definer_processor::DefinerProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        assert!(
            !processor.has_error()
                && matches!(processor.definer_type, definer_processor::DefinerCollecting::Generic(x), if x.rtype == "int")
        );
    }

    #[test]
    fn generic_collected_with_no_error() {
        let code = "int";
        let mut pos = defs::CursorPosition::default();
        let mut processor: definer_processor::DefinerProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        assert!(
            !processor.has_error()
                && matches!(processor.definer_type, definer_processor::DefinerCollecting::Generic(x), if x.rtype == "int")
        );
    }

    #[test]
    fn nullable_generic_collected_with_no_error() {
        let code = "?int";
        let mut pos = defs::CursorPosition::default();
        let mut processor: definer_processor::DefinerProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        assert!(
            !processor.has_error()
                && matches!(processor.definer_type, definer_processor::DefinerCollecting::Nullable(x), if matches!(x,definer_processor::DefinerCollecting::Generic(e), if e.rtype == "int"))
        );
    }

    #[test]
    fn future_generic_collected_with_no_error() {
        let code = ">int";
        let mut pos = defs::CursorPosition::default();
        let mut processor: definer_processor::DefinerProcessor = Processor::new();
        let mut last_char = '\0';
        for letter_char in code.chars() {
            processor.iterate(pos, last_char, letter_char);
            pos.skip_char(1);
            last_char = letter_char;
        }
        assert!(
            !processor.has_error()
                && matches!(processor.definer_type, definer_processor::DefinerCollecting::Future(x), if matches!(x,definer_processor::DefinerCollecting::Generic(e), if e.rtype == "int"))
        );
    }
}
