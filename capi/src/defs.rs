use libc::c_char;

#[repr(C)]
pub enum ParserType {
    RawParser,
    ClassParser,
    HeaderParser,
}

#[repr(C)]
pub struct ParserOptions {
    pub path: *mut c_char,
    pub functions: bool,
    pub break_on_error: bool,
    pub loops: bool,
    pub enums: bool,
    pub classes: bool,
    pub getters: bool,
    pub setters: bool,
    pub conditions: bool,
    pub global_variables: bool,
    pub line_ending: *mut c_char,
    pub dynamics: bool,
    pub collectives: bool,
    pub variables: bool,
    pub import_std: bool,
    pub constants: bool,
    pub parser_type: ParserType,
    pub allow_import: bool,
}

#[repr(C)]
pub struct CursorPosition(pub usize, pub usize);

#[repr(C)]
pub struct Cursor {
    pub range_start: CursorPosition,
    pub range_end: CursorPosition,
}
