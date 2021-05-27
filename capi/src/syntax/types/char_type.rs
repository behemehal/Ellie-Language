use libc::c_char;

#[repr(C)]
pub struct CharType {
    pub value: c_char,
    pub complete: bool,
}
