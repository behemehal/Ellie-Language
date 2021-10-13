use crate::definite::types;
use crate::defs;
use libc::c_char;


#[repr(C)]
pub struct FileKey {
    pub key_name: *mut c_char,
    pub value: types::Types,
    pub key_name_location: defs::Cursor,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
}
