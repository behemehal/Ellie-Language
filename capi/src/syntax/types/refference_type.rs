use crate::syntax::types;
use libc::c_char;

#[repr(C)]
pub struct RefferenceType {
    pub refference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: *const *const c_char,
}
