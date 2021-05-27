#![allow(warnings)] //TODO Remove this
use core::fmt;
use serde::Serialize;

#[repr(C)]
#[no_mangle]
#[derive(PartialEq, Debug, Clone)]
pub struct ParserOptions {
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub global_variables: bool,
    pub dynamics: bool,
    pub collectives: bool,
    pub variables: bool,
}

impl Default for ParserOptions {
    fn default() -> Self {
        ParserOptions {
            functions: true,
            break_on_error: false,
            loops: true,
            global_variables: true,
            dynamics: true,
            collectives: true,
            variables: true,
        }
    }
}

#[repr(C)]
#[no_mangle]
#[derive(PartialEq, Debug, Clone, Copy, Serialize)]
pub struct CursorPosition(pub i64, pub i64);

impl Default for CursorPosition {
    fn default() -> Self {
        CursorPosition(0, 0)
    }
}

impl CursorPosition {
    
    #[no_mangle]
    pub extern "C" fn skipChar(&mut self, n: i64) -> CursorPosition {
        self.1 += n;
        return self.clone();
    }

    #[no_mangle]
    pub extern "C" fn popChar(&mut self, n: i64) -> CursorPosition {
        self.1 -= n;
        return self.clone();
    }
}

#[repr(C)]
#[no_mangle]
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

#[repr(C)]
#[no_mangle]
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
                self.error.builded_message
            )
            .to_string(),
        )
    }
}
