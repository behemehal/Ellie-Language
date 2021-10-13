use crate::definite::definers;
use crate::definite::items::Collecting;
use crate::defs;
use libc::c_char;



#[repr(C)]

pub struct Getter {
    pub name: *mut c_char,
    pub name_pos: defs::Cursor,
    pub public: bool,
    pub rtype_pos: defs::Cursor,
    pub bracket_start_pos: defs::Cursor,
    pub bracket_end_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub code: Vec<Collecting>,
}
