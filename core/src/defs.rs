#![allow(warnings)] //TODO Remove this
use crate::alloc::borrow::ToOwned;
use alloc::format;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::fmt;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum ParserType {
    RawParser,
    ClassParser,
}

impl Default for ParserType {
    fn default() -> Self {
        ParserType::RawParser
    }
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct ParserOptions {
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub classes: bool,
    pub conditions: bool,
    pub global_variables: bool,
    pub line_ending: String,
    pub dynamics: bool,
    pub collectives: bool,
    pub variables: bool,
    pub constants: bool,
    pub parser_type: ParserType,
}

impl Default for ParserOptions {
    fn default() -> Self {
        ParserOptions {
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            classes: true,
            global_variables: true,
            line_ending: "\\r\\n".to_string(),
            dynamics: true,
            collectives: true,
            variables: true,
            constants: true,
            parser_type: ParserType::RawParser,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct CursorPosition(pub usize, pub usize);

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition(0, 0)
    }
}

impl CursorPosition {
    pub fn skipChar(&mut self, n: usize) -> CursorPosition {
        self.1 += n;
        return self.clone();
    }

    pub fn popChar(&mut self, n: usize) -> CursorPosition {
        self.1 -= n;
        return self.clone();
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition,
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            range_start: CursorPosition::default(),
            range_end: CursorPosition::default(),
        }
    }
}

pub struct SyntaxError {
    error: crate::error::Error,
    position: Cursor,
    fields: Vec<crate::error::ErrorBuildField>,
    debugText: String,
}

impl fmt::Display for SyntaxError {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        // We need to remove "-" from the number output.
        formatter.write_str(
            &format!(
                "{} {}",
                if self.debugText != "" {
                    "[".to_owned() + &self.debugText + "]"
                } else {
                    "".to_string()
                },
                self.error.builded_message.builded
            )
            .to_string(),
        )
    }
}
