use crate::defs;
use libc::c_char;
use ellie_core::definite::types::string;

#[repr(C)]
pub struct StringType {
    pub value: *mut c_char,
    pub comma_start_pos: defs::Cursor,
    pub comma_end_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

pub fn build_string_from(target: string::StringType) -> StringType {
    StringType {
        value: target.value.as_ptr() as *mut i8,
        comma_start_pos: defs::Cursor {
            range_start: defs::CursorPosition(target.comma_start_pos.range_start.0, target.comma_start_pos.range_start.1),
            range_end: defs::CursorPosition(target.comma_start_pos.range_end.0, target.comma_start_pos.range_end.1),
        },
        comma_end_pos: defs::Cursor {
            range_start: defs::CursorPosition(target.comma_end_pos.range_start.0, target.comma_end_pos.range_start.1),
            range_end: defs::CursorPosition(target.comma_end_pos.range_end.0, target.comma_end_pos.range_end.1),
        },
        value_pos: defs::Cursor {
            range_start: defs::CursorPosition(target.comma_start_pos.range_start.0, target.comma_start_pos.range_start.1),
            range_end: defs::CursorPosition(target.comma_start_pos.range_end.0, target.comma_start_pos.range_end.1),
        },
    }
}
