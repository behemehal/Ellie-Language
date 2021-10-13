use crate::definite::types;
use crate::defs;
use alloc::boxed::Box;
use ellie_core::definite::types::array;

#[repr(C)]
pub struct ArrayEntry {
    pub value: Box<types::Types>,
    pub location: defs::Cursor,
}

#[repr(C)]
pub struct ArrayType {
    pub layer_size: usize,
    pub collective: *mut ArrayEntry,
}

pub unsafe fn build_array_from(target: array::ArrayType) -> ArrayType {
    ArrayType {
        layer_size: target.layer_size,
        collective: target
            .collective
            .into_iter()
            .map(|entry| ArrayEntry {
                value: Box::new(types::build_collecting_from(*entry.value)),
                location: defs::Cursor {
                    range_start: defs::CursorPosition(
                        entry.location.range_start.0,
                        entry.location.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        entry.location.range_end.0,
                        entry.location.range_end.1,
                    ),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
