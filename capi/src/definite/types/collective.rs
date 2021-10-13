use crate::definite::types;
use crate::defs;
use alloc::boxed::Box;
use ellie_core::definite::types::collective;

#[repr(C)]
pub struct CollectiveEntry {
    pub key: Box<types::Types>,
    pub value: Box<types::Types>,
    pub key_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
}

#[repr(C)]
pub struct Collective {
    pub entries: *mut CollectiveEntry,
}

pub unsafe fn build_collective_from(target: collective::Collective) -> Collective {
    Collective {
        entries: target
            .entries
            .into_iter()
            .map(|entry| CollectiveEntry {
                key: Box::new(types::build_collecting_from(*entry.key)),
                value: Box::new(types::build_collecting_from(*entry.value)),
                key_pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        entry.key_pos.range_start.0,
                        entry.key_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        entry.key_pos.range_end.0,
                        entry.key_pos.range_end.1,
                    ),
                },
                value_pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        entry.value_pos.range_start.0,
                        entry.value_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        entry.value_pos.range_end.0,
                        entry.value_pos.range_end.1,
                    ),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
