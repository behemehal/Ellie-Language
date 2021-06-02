use crate::syntax::types;
use libc::c_char;

#[repr(C)]
pub struct CloakEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
}

#[repr(C)]
pub struct CloakType {
    pub layer_size: usize,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
    pub collective: *const CloakEntry,
}
