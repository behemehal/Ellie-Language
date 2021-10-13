use crate::defs;
use ellie_core::definite::types::variable;
use libc::c_char;

#[repr(C)]
pub struct VariableType {
    pub value: *mut c_char,
    pub pos: defs::Cursor,
}

pub fn build_variable_type_from(target: variable::VariableType) -> VariableType {
    VariableType {
        value: target.value.as_ptr() as *mut i8,
        pos: defs::Cursor {
            range_start: defs::CursorPosition(target.pos.range_start.0, target.pos.range_start.1),
            range_end: defs::CursorPosition(target.pos.range_end.0, target.pos.range_end.1),
        },
    }
}
