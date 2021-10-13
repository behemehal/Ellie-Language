use crate::definite::definers;
use crate::definite::items::Collecting;
use crate::defs;
use libc::c_char;



#[repr(C)]
pub struct FunctionParameter {
    pub name: *mut c_char,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[repr(C)]
pub struct Function {
    pub name: *mut c_char,
    pub parameters: Vec<FunctionParameter>,
    pub return_type: definers::DefinerCollecting,
    pub public: bool,
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor,
    pub code_bracket_start: defs::Cursor,
    pub code_bracket_end: defs::Cursor,
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
