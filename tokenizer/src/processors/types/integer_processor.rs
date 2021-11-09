use ellie_core::{definite::types::integer, defs, error};

#[derive(Default, Clone, Debug)]
pub struct IntegerProcessor {
    pub raw_size: String,
    pub size: integer::IntegerSize,
    pub errors: Vec<error::Error>,
    pub cursor: defs::CursorPosition,
    pub complete: bool,
}

impl Processor for IntegerProcessor {
    fn new() -> Self {
        IntegerProcessor::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char) {

        

    }
}