use crate::definite::types;
use crate::defs;
use ellie_core::definite::types::function_call;

use libc::c_char;

#[repr(C)]
pub struct FunctionCallParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct FunctionCall {
    pub name: *mut c_char,
    pub name_pos: defs::Cursor,
    pub params: *mut FunctionCallParameter,
}

pub unsafe fn build_function_call_from(target: function_call::FunctionCall) -> FunctionCall {
    FunctionCall {
        name: target.name.as_ptr() as *mut i8,
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                target.name_pos.range_start.0,
                target.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                target.name_pos.range_end.0,
                target.name_pos.range_end.1,
            ),
        },
        params: target
            .params
            .into_iter()
            .map(|param| FunctionCallParameter {
                value: types::build_collecting_from(param.value),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        param.pos.range_start.0,
                        param.pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(param.pos.range_end.0, param.pos.range_end.1),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
    }
}
