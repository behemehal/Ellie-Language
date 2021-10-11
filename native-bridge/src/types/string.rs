use libc::c_char;

#[repr(C)]
pub struct StringType {
    pub value: *mut c_char,
}
