use crate::processors::Processor;
use crate::syntax::types::array_type;
use ellie_core::{defs, error};

impl Processor for array_type::ArrayTypeCollector {
    fn new() -> Self {
        array_type::ArrayTypeCollector::default()
    }

    fn keyword(&self) -> &str {
        ""
    }

    fn has_accessibility(&self) -> bool {
        false
    }

    fn iterate(
        &mut self,
        _errors: &mut Vec<error::Error>,
        _cursor: defs::CursorPosition,
        _last_char: char,
        _letter_char: char,
    ) {
        todo!()
    }
}
