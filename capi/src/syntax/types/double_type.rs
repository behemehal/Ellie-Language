use libc::c_char;

#[repr(C)]
pub struct DoubleType {
    pub value: f32,
    pub raw_value: *const c_char,
    pub complete: bool,
}
