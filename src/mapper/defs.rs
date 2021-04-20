#![allow(warnings)] //TODO Remove this
use serde::Serialize;
use core::fmt;

#[derive(PartialEq, Debug, Clone)]
pub struct MapperOptions {
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub global_variables: bool,
    pub collectives: bool,
    pub variables: bool
}

impl Default for MapperOptions {
    fn default() -> Self { MapperOptions {functions: true, break_on_error: false, loops: true, global_variables: true, collectives: true, variables: true} }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct CursorPosition(pub i64,pub i64);

impl Default for CursorPosition {
    fn default() -> Self { CursorPosition(0, 0) }
}

impl CursorPosition {
    pub fn skipChar(&mut self, n: i64) -> CursorPosition {
        self.1 += n;
        return self.clone();
    }

    pub fn popChar(&mut self, n: i64) -> CursorPosition {
        self.1 -= n;
        return self.clone();
    }
}

#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition
}

impl Default for Cursor {
    fn default() -> Self {
        Cursor {range_start: CursorPosition::default(), range_end: CursorPosition::default()}
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
        formatter.write_str(&format!("{} {}",if self.debugText != "" {"[".to_owned() + &self.debugText + "]"} else {"".to_string()}, self.error.builded_message).to_string())   
    }
}