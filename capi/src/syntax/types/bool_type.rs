use libc::c_char;

#[repr(C)]
pub struct BoolType {
    pub value: bool,
}
