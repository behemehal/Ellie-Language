use crate::syntax::types;
use crate::defs;
use libc::c_char;

#[repr(C)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct FunctionCall {
    pub name: *const c_char,
    pub name_pos: defs::Cursor,
    pub comma: bool,
    pub complete: bool,
    pub params: *const FunctionCallParameter,
}
