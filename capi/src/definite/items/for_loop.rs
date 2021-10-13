use crate::alloc::boxed::Box;
use crate::definite::items as crate_definite;
use crate::definite::types;
use crate::defs;
use ellie_core::definite::items;

#[repr(C)]
pub struct ForLoop {
    pub parameter: Box<types::Types>,
    pub parameter_pos: defs::Cursor,
    pub code: *mut crate_definite::Collecting,
    pub pos: defs::Cursor,
}

pub unsafe fn build_for_loop_from(from: items::for_loop::ForLoop) -> ForLoop {
    ForLoop {
        parameter: Box::new(types::build_collecting_from(*from.parameter)),
        parameter_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.parameter_pos.range_start.0,
                from.parameter_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.parameter_pos.range_end.0,
                from.parameter_pos.range_end.1,
            ),
        },
        code: from
            .code
            .into_iter()
            .map(|item| crate_definite::build_collecting_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
