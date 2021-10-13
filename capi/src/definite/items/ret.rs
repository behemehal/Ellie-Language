use crate::definite::types;
use crate::defs;
use ellie_core::definite::items;

#[repr(C)]
pub struct Ret {
    pub value: types::Types,
    pub keyword_pos: defs::Cursor,
    pub value_position: defs::Cursor,
    pub pos: defs::Cursor,
}

pub unsafe fn build_ret_from(from: items::ret::Ret) -> Ret {
    Ret {
        value: types::build_collecting_from(from.value),
        keyword_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.keyword_pos.range_start.0,
                from.keyword_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.keyword_pos.range_end.0,
                from.keyword_pos.range_end.1,
            ),
        },
        value_position: defs::Cursor {
            range_start: defs::CursorPosition(
                from.value_position.range_start.0,
                from.value_position.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.value_position.range_end.0,
                from.value_position.range_end.1,
            ),
        },
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
