use crate::alloc::borrow::ToOwned;
use alloc::string::String;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum TokenizerType {
    Raw,
    ClassParser,
    FunctionParser,
    HeaderParser,
}

impl Default for TokenizerType {
    fn default() -> Self {
        TokenizerType::Raw
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub struct TokenizerOptions {
    pub path: String,
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub enums: bool,
    pub classes: bool,
    pub getters: bool,
    pub setters: bool,
    pub conditions: bool,
    pub global_variables: bool,
    pub line_ending: String,
    pub dynamics: bool,
    pub collectives: bool,
    pub variables: bool,
    pub import_std: bool,
    pub constants: bool,
    pub ignore_imports: bool,
    pub parser_type: TokenizerType,
    pub allow_import: bool,
}

impl Default for TokenizerOptions {
    fn default() -> Self {
        TokenizerOptions {
            path: "".to_owned(),
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            getters: true,
            setters: true,
            classes: true,
            enums: true,
            global_variables: true,
            line_ending: "\\r\\n".to_owned(),
            dynamics: true,
            import_std: true,
            collectives: true,
            ignore_imports: false,
            variables: true,
            constants: true,
            parser_type: TokenizerType::Raw,
            allow_import: true,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
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

    pub fn increase_line(&mut self, n: usize) -> CursorPosition {
        let mut clone = *self;
        clone.0 += n;
        clone
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize, Deserialize)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition,
}

impl Cursor {
    pub fn is_zero(&self) -> bool {
        self.range_start.is_zero() && self.range_end.is_zero()
    }

    pub fn is_bigger(&self, than: Cursor) -> bool {
        if than.range_end.0 == self.range_end.0 {
            self.range_end.1 > than.range_end.1
        } else if than.range_end.0 > self.range_end.0 {
            return false;
        } else {
            return true;
        }
    }

    pub fn build_with_skip_char(range_start: CursorPosition) -> Self {
        Cursor {
            range_start,
            range_end: range_start.clone().skip_char(1),
        }
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

pub trait NativePlugin {}
