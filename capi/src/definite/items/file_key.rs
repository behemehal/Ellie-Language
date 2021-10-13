use crate::definite::types;
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct FileKey {
    pub key_name: *mut c_char,
    pub value: types::Types,
    pub key_name_location: defs::Cursor,
    pub value_location: defs::Cursor,
    pub pos: defs::Cursor,
}

pub unsafe fn build_file_key_from(from: items::file_key::FileKey) -> FileKey {
    FileKey {
        key_name: from.key_name.as_ptr() as *mut i8,
        value: types::build_collecting_from(from.value),
        key_name_location: defs::Cursor {
            range_start: defs::CursorPosition(
                from.key_name_location.range_start.0,
                from.key_name_location.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.key_name_location.range_end.0,
                from.key_name_location.range_end.1,
            ),
        },
        value_location: defs::Cursor {
            range_start: defs::CursorPosition(
                from.value_location.range_start.0,
                from.value_location.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.value_location.range_end.0,
                from.value_location.range_end.1,
            ),
        },
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
