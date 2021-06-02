use libc::c_char;

#[repr(C)]
pub struct Error {
    pub code: u8,
    pub message: *const c_char,
    pub title: *const c_char,
    pub builded_message: *const c_char,
    pub debug_message: *const c_char,
    pub pos: crate::defs::Cursor,
}

#[repr(C)]
pub struct ErrorBuildField {
    pub key: *const c_char,
    pub value: *const c_char,
}
