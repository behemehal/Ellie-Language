use crate::defs;
use ellie_core::definite::items;
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

pub unsafe fn build_import_from(from: items::import::Import) -> Import {
    Import {
        path: from.path.as_ptr() as *mut i8,
        native: from.native,
        public: from.public,
        resolution_id: from.resolution_id,
        id: from.id,
        path_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.path_pos.range_start.0,
                from.path_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.path_pos.range_end.0, from.path_pos.range_end.1),
        },
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
