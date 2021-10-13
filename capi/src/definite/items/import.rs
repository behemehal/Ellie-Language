use crate::defs;
use libc::c_char;


#[repr(C)]
pub struct Import {
    pub path: *mut c_char,
    pub native: bool,
    pub public: bool,
    pub resolution_id: u64,
    pub id: u64,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
