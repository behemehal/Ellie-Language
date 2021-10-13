
use libc::c_char;

#[repr(C)]
pub struct DoubleType {
    pub value: f32,
    pub complete: bool,
}
