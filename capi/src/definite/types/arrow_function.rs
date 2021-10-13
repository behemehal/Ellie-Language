use crate::definite::items::Collecting;
use crate::definite::{definers, items::function};
use crate::defs;
use ellie_core::definite::types::arrow_function;

#[repr(C)]
pub struct ArrowFunction {
    pub parameters: *mut function::FunctionParameter,
    pub return_type: definers::DefinerCollecting,
    pub inside_code: *mut Collecting,
    pub return_pos: defs::Cursor,
}

pub unsafe fn build_arrow_function_from(target: arrow_function::ArrowFunction) -> ArrowFunction {
    ArrowFunction {
        parameters: target
            .parameters
            .into_iter()
            .map(|param| function::FunctionParameter {
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
        return_type: definers::build_definer_from(target.return_type),
        inside_code: target
            .inside_code
            .into_iter()
            .map(|x| crate::definite::items::build_collecting_from(x))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        return_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                target.return_pos.range_start.0,
                target.return_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                target.return_pos.range_end.0,
                target.return_pos.range_end.1,
            ),
        },
    }
}
