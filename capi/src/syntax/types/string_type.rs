use libc::c_char;

#[repr(C)]
pub struct StringType {
    pub value: *const c_char,
    pub complete: bool,
}
