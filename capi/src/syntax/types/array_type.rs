use crate::syntax::types;

#[repr(C)]
pub struct ArrayEntry {
    pub value_complete: bool,
    pub value: Box<types::Types>,
}

#[repr(C)]
pub struct ArrayType {
    pub layer_size: usize,
    pub complete: bool,
    pub comma: bool,
    pub child_start: bool,
    pub collective: *const ArrayEntry,
}
