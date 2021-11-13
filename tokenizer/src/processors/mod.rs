use ellie_core::{defs, error};

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

pub mod items;
pub mod types;
