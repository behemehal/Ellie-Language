use ellie_core::{defs, error};
pub mod items;
pub mod types;

#[derive(Debug, Clone, Copy)]
pub struct EscapeCharEmitter {
    pub emit: &'static [char; 3],
    pub increase_cursor: bool,
}

impl EscapeCharEmitter {
    pub fn dont_emit() -> Self {
        Self {
            emit: &['\0', '\0', '\0'],
            increase_cursor: false,
        }
    }

    pub fn is_emitting(&self) -> bool {
        self.emit[0] != '\0' && self.emit[1] != '\0' && self.emit[2] != '\0'
    }
}

pub trait Processor {
    /// Returns true if the processor emits line endings.
    /// For example, if variable's value is a string or char processor would return true because \n can be captured by them.
    fn emits_line_endings(&self) -> EscapeCharEmitter {
        EscapeCharEmitter {
            emit: &['\0', '\0', '\0'],
            increase_cursor: true,
        }
    }

    fn iterate(
        &mut self,
        errors: &mut Vec<error::Error>,
        cursor: defs::CursorPosition,
        last_char: char,
        letter_char: char,
    ) -> bool;
}
