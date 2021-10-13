use crate::definite::definers;
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct NativeFunctionParameter {
    pub name: *mut c_char,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[repr(C)]
pub struct NativeFunction {
    pub name: *mut c_char,                        //NativeFunction Name c_char
    pub parameters: *mut NativeFunctionParameter, //Parameter vector
    pub return_type: definers::DefinerCollecting, //Return type from enum
    pub public: bool,
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

pub unsafe fn build_native_function_from(
    from: items::native_function::NativeFunction,
) -> NativeFunction {
    NativeFunction {
        name: from.name.as_ptr() as *mut i8,
        parameters: from
            .parameters
            .into_iter()
            .map(|param| NativeFunctionParameter {
                name: param.name.as_ptr() as *mut i8,
                rtype: definers::build_definer_from(param.rtype),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        param.pos.range_start.0,
                        param.pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(param.pos.range_end.0, param.pos.range_end.1),
                },
                multi_capture: param.multi_capture,
                name_pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        param.name_pos.range_start.0,
                        param.name_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        param.name_pos.range_end.0,
                        param.name_pos.range_end.1,
                    ),
                },
                type_pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        param.name_pos.range_start.0,
                        param.name_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        param.name_pos.range_end.0,
                        param.name_pos.range_end.1,
                    ),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        return_type: definers::build_definer_from(from.return_type),
        public: from.public,
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.name_pos.range_start.0,
                from.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        parameters_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.parameters_pos.range_start.0,
                from.parameters_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.parameters_pos.range_end.0,
                from.parameters_pos.range_end.1,
            ),
        },
        return_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.return_pos.range_start.0,
                from.return_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.return_pos.range_end.0,
                from.return_pos.range_end.1,
            ),
        },
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
