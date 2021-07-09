use alloc::string::{String, ToString};
use core::hash::Hash;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
pub enum ParserType {
    RawParser,
    ClassParser,
}

impl Default for ParserType {
    fn default() -> Self {
        ParserType::RawParser
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Hash)]
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
    pub allow_import: bool,
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
            allow_import: true,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Hash)]
pub struct CursorPosition(pub usize, pub usize);

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition(0, 0)
    }
}

impl CursorPosition {
    pub fn skip_char(&mut self, n: usize) -> CursorPosition {
        let mut clone = self.clone();
        clone.1 += n;
        clone
    }

    pub fn pop_char(&mut self, n: usize) -> CursorPosition {
        let mut clone = *self;
        clone.1 -= n;
        clone
    }

    pub fn is_zero(&self) -> bool {
        self.0 == 0 && self.1 == 0
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Hash)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition,
}

impl Cursor {
    pub fn is_zero(&self) -> bool {
        self.range_start.is_zero() && self.range_end.is_zero()
    }
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {
            range_start: CursorPosition::default(),
            range_end: CursorPosition::default(),
        }
    }
}