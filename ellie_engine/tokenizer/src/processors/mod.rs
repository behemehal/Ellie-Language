use ellie_core::{defs, error};
pub mod items;
pub mod types;

pub trait Processor {
    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool;
}
