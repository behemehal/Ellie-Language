use ellie_core::{defs, error};
pub mod definer_processor;
pub mod variable_processor;

pub trait Processor {
    fn new() -> Self;
    fn keyword(&self) -> &str;
    fn has_accessibility(&self) -> bool;
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    );
}
