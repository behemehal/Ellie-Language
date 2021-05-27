use libc::c_char;

#[repr(C)]
#[derive(Default)]
pub struct ParserOptions {
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub global_variables: bool,
    pub dynamics: bool,
    pub collectives: bool,
    pub variables: bool,
}

#[repr(C)]
#[derive(Default)]
pub struct CursorPosition(pub i64, pub i64);

#[repr(C)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition,
}

#[repr(C)]
pub struct SyntaxError {
    error: crate::error::Error,
    position: Cursor,
    fields: *const crate::error::ErrorBuildField,
    debugText: *const c_char,
}