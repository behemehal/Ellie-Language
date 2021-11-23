use crate::processors::Processor;
use crate::syntax::types::array_type;
use ellie_core::{defs, error, utils::reliable_name_range};

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
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) {
        todo!()
    }
}
