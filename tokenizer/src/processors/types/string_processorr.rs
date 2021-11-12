use crate::processors::{reliable_char, Processor};
use ellie_core::{definite::types::integer, defs, error};

#[derive(Default, Clone, Debug)]
pub struct StringProcessor {
    pub string: String,
    pub comma_started: bool,
    pub errors: Vec<error::Error>,
    pub cursor: defs::CursorPosition,
    pub forward: Option<ellie_core::definite::types::Types>,
    pub complete: bool,
}

impl Processor<ellie_core::definite::types::Types> for StringProcessor {
    fn new() -> Self {
        StringProcessor::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char) {
        
        if !self.comma_started {
            
        }

    }

    fn has_error(&self) -> bool {
        !self.errors.is_empty()
    }

    fn errors(&self) -> Vec<error::Error> {
        self.errors.clone()
    }

    fn is_complete(&self) -> bool {
        self.complete
    }

    fn is_forwarded(&self) -> Option<ellie_core::definite::types::Types> {
        self.forward.clone()
    }
}
