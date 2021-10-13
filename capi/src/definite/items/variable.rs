use crate::definite::{definers, types};
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct Variable {
    pub name: *mut c_char,
    pub dynamic: bool,
    pub constant: bool,
    pub public: bool,
    pub value: types::Types,
    pub pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub hash: *mut c_char,
}

pub unsafe fn build_variable_from(from: items::variable::Variable) -> Variable {
    Variable {
        name: from.name.as_ptr() as *mut i8,
        dynamic: from.dynamic,
        constant: from.constant,
        public: from.public,
        value: types::Types::Null,
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.name_pos.range_start.0,
                from.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        value_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.value_pos.range_start.0,
                from.value_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.value_pos.range_end.0, from.value_pos.range_end.1),
        },
        type_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.type_pos.range_start.0,
                from.type_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.type_pos.range_end.0, from.type_pos.range_end.1),
        },
        rtype: definers::DefinerCollecting::Dynamic,
        hash: from.hash.as_ptr() as *mut i8,
    }
}
