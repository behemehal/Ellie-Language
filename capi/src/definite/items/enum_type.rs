use crate::definite::definers;
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct EnumItem {
    pub has_type: bool,
    pub identifier: *mut c_char,
    pub enum_type: definers::DefinerCollecting,
    pub identifier_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[repr(C)]
pub struct EnumType {
    pub public: bool,
    pub name: *mut c_char,
    pub name_pos: defs::Cursor,
    pub brace_start_pos: defs::Cursor,
    pub brace_end_pos: defs::Cursor,
    pub items: *mut EnumItem,
}

pub unsafe fn build_enum_from(from: items::enum_type::EnumType) -> EnumType {
    EnumType {
        public: from.public,
        name: from.name.as_ptr() as *mut i8,
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.name_pos.range_start.0,
                from.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        brace_start_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.brace_start_pos.range_start.0,
                from.brace_start_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.brace_start_pos.range_end.0,
                from.brace_start_pos.range_end.1,
            ),
        },
        brace_end_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.brace_end_pos.range_start.0,
                from.brace_end_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.brace_end_pos.range_end.0,
                from.brace_end_pos.range_end.1,
            ),
        },
        items: from
            .items
            .into_iter()
            .map(|item| EnumItem {
                has_type: item.has_type,
                identifier: item.identifier.as_ptr() as *mut i8,
                enum_type: definers::build_definer_from(item.enum_type),
                identifier_pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        item.identifier_pos.range_start.0,
                        item.identifier_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        item.identifier_pos.range_end.0,
                        item.identifier_pos.range_end.1,
                    ),
                },
                type_pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        item.type_pos.range_start.0,
                        item.type_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        item.type_pos.range_end.0,
                        item.type_pos.range_end.1,
                    ),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
