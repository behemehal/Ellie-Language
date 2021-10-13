use crate::definite::types;
use crate::defs;
use alloc::boxed::Box;
use ellie_core::definite::types::reference;

#[repr(C)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: types::Types,
}

#[repr(C)]
pub struct ReferenceType {
    pub reference: Box<types::Types>,
    pub reference_pos: defs::Cursor,
    pub chain: *mut Chain,
}

pub unsafe fn build_reference_from(target: reference::ReferenceType) -> ReferenceType {
    ReferenceType {
        reference: Box::new(types::build_collecting_from(*target.reference)),
        reference_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                target.reference_pos.range_start.0,
                target.reference_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                target.reference_pos.range_end.0,
                target.reference_pos.range_end.1,
            ),
        },
        chain: target
            .chain
            .into_iter()
            .map(|chain| Chain {
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        chain.pos.range_start.0,
                        chain.pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(chain.pos.range_end.0, chain.pos.range_end.1),
                },
                value: types::build_collecting_from(chain.value),
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
