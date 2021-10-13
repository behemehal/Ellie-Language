use crate::definite::definers;
use crate::defs;
use libc::c_char;



#[repr(C)]
pub struct NativeFunctionParameter {
    pub name: *mut c_char,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[repr(C)]
pub struct NativeFunction {
    pub name: *mut c_char,                             //NativeFunction Name c_char
    pub parameters: Vec<NativeFunctionParameter>, //Parameter vector
    pub return_type: definers::DefinerCollecting, //Return type from enum
    pub public: bool,
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
}
