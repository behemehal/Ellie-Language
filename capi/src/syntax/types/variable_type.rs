use libc::c_char;

#[repr(C)]
pub struct VariableType {
    pub value_complete: bool,
    pub value: *const c_char,
}
