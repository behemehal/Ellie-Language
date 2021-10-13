use crate::definite::definers;
use crate::definite::items as crate_definite;
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct FunctionParameter {
    pub name: *mut c_char,
    pub rtype: definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub multi_capture: bool,
    pub name_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[repr(C)]
pub struct Function {
    pub name: *mut c_char,
    pub parameters: *mut FunctionParameter,
    pub return_type: definers::DefinerCollecting,
    pub public: bool,
    pub inside_code: *mut crate_definite::Collecting,
    pub name_pos: defs::Cursor,
    pub code_bracket_start: defs::Cursor,
    pub code_bracket_end: defs::Cursor,
    pub parameters_pos: defs::Cursor,
    pub return_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

pub unsafe fn build_function_from(from: items::function::Function) -> Function {
    Function {
        name: from.name.as_ptr() as *mut i8,
        parameters: from
            .parameters
            .into_iter()
            .map(|param| FunctionParameter {
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
                        param.type_pos.range_start.0,
                        param.type_pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        param.type_pos.range_end.0,
                        param.type_pos.range_end.1,
                    ),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        return_type: definers::build_definer_from(from.return_type),
        public: from.public,
        inside_code: from
            .inside_code
            .into_iter()
            .map(|item| crate_definite::build_collecting_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.name_pos.range_start.0,
                from.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        code_bracket_start: defs::Cursor {
            range_start: defs::CursorPosition(
                from.code_bracket_start.range_start.0,
                from.code_bracket_start.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.code_bracket_start.range_end.0,
                from.code_bracket_start.range_end.1,
            ),
        },
        code_bracket_end: defs::Cursor {
            range_start: defs::CursorPosition(
                from.code_bracket_end.range_start.0,
                from.code_bracket_end.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.code_bracket_end.range_end.0,
                from.code_bracket_end.range_end.1,
            ),
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
