use crate::definite::types;
use crate::defs;
use alloc::boxed::Box;
use ellie_core::definite::types::constructed_class;

#[repr(C)]
pub struct ConstructedClassParameter {
    pub value: types::Types,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct ConstructedClass {
    pub value: Box<types::Types>,
    pub keyword_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub params: *mut ConstructedClassParameter,
}

pub unsafe fn build_constructed_class_from(
    target: constructed_class::ConstructedClass,
) -> ConstructedClass {
    ConstructedClass {
        value: Box::new(types::build_collecting_from(*target.value)),
        keyword_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                target.keyword_pos.range_start.0,
                target.keyword_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                target.keyword_pos.range_end.0,
                target.keyword_pos.range_end.1,
            ),
        },
        value_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                target.value_pos.range_start.0,
                target.value_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                target.value_pos.range_end.0,
                target.value_pos.range_end.1,
            ),
        },
        params: target
            .params
            .into_iter()
            .map(|param| ConstructedClassParameter {
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
