use crate::definite::definers;
use libc::c_char;



#[repr(C)]
pub struct EnumItem {
    pub has_type: bool,
    pub identifier: *mut c_char,
    pub enum_type: definers::DefinerCollecting,
    pub identifier_pos: crate::defs::Cursor,
    pub type_pos: crate::defs::Cursor,
}

#[repr(C)]
pub struct EnumType {
    pub public: bool,
    pub name: *mut c_char,
    pub name_pos: crate::defs::Cursor,
    pub brace_start_pos: crate::defs::Cursor,
    pub brace_end_pos: crate::defs::Cursor,
    pub items: Vec<EnumItem>,
}
