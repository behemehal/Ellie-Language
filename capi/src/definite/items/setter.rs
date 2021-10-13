use crate::definite::definers;
use crate::definite::items as crate_definite;
use crate::definite::items::Collecting;
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct Setter {
    pub name: *mut c_char,
    pub name_pos: defs::Cursor,
    pub public: bool,
    pub rtype_pos: defs::Cursor,
    pub bracket_start_pos: defs::Cursor,
    pub bracket_end_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub code: *mut Collecting,
}

pub unsafe fn build_setter_from(from: items::setter::Setter) -> Setter {
    Setter {
        name: from.name.as_ptr() as *mut i8,
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.name_pos.range_start.0,
                from.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        public: from.public,
        rtype_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.rtype_pos.range_start.0,
                from.rtype_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.rtype_pos.range_end.0, from.rtype_pos.range_end.1),
        },
        bracket_start_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.bracket_start_pos.range_start.0,
                from.bracket_start_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.bracket_start_pos.range_end.0,
                from.bracket_start_pos.range_end.1,
            ),
        },
        bracket_end_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.bracket_end_pos.range_start.0,
                from.bracket_end_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.bracket_end_pos.range_end.0,
                from.bracket_end_pos.range_end.1,
            ),
        },
        rtype: definers::build_definer_from(from.rtype),
        code: from
            .code
            .into_iter()
            .map(|item| crate_definite::build_collecting_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
