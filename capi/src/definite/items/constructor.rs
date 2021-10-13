use crate::definite::items as crate_definite;
use crate::definite::items::Collecting;
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct ConstructorParameter {
    pub name: *mut c_char,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct Constructor {
    pub name: *mut c_char,
    pub parameters: *mut ConstructorParameter,
    pub inside_code: *mut Collecting,
    pub name_pos: defs::Cursor,
    pub parameters_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

pub unsafe fn build_constructor_from(from: items::constructor::Constructor) -> Constructor {
    Constructor {
        name: from.name.as_ptr() as *mut i8,
        parameters: from
            .parameters
            .into_iter()
            .map(|param| ConstructorParameter {
                name: param.name.as_ptr() as *mut i8,
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
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
