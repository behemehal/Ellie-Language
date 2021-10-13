use crate::definite::types;
use crate::defs;
use ellie_core::definite::items;

#[repr(C)]
pub enum Callers {
    FunctionCall(types::function_call::FunctionCall),
    ConstructedClass(types::constructed_class::ConstructedClass),
}

#[repr(C)]
pub struct Caller {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

pub unsafe fn build_caller_from(from: items::caller::Caller) -> Caller {
    Caller {
        value: types::build_collecting_from(from.value),
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
