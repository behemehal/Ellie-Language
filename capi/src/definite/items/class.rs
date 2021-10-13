use crate::definite::items::{constructor, function, getter, setter, variable};
use crate::defs;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct GenericDefining {
    pub name: *mut c_char,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct Class {
    pub name: *mut c_char,
    pub public: bool,
    pub constructor: constructor::Constructor,
    pub generic_definings: *mut GenericDefining,
    pub properties: *mut variable::Variable,
    pub getters: *mut getter::Getter,
    pub setters: *mut setter::Setter,
    pub methods: *mut function::Function,
    pub name_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

pub unsafe fn build_class_from(from: items::class::Class) -> Class {
    Class {
        name: from.name.as_ptr() as *mut i8,
        public: from.public,
        constructor: constructor::build_constructor_from(from.constructor),
        generic_definings: from
            .generic_definings
            .into_iter()
            .map(|generic| GenericDefining {
                name: generic.name.as_ptr() as *mut i8,
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        generic.pos.range_start.0,
                        generic.pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(
                        generic.pos.range_end.0,
                        generic.pos.range_end.1,
                    ),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        properties: from
            .properties
            .into_iter()
            .map(|item| variable::build_variable_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        getters: from
            .getters
            .into_iter()
            .map(|item| getter::build_getter_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        setters: from
            .setters
            .into_iter()
            .map(|item| setter::build_setter_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        methods: from
            .methods
            .into_iter()
            .map(|item| function::build_function_from(item))
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.name_pos.range_start.0,
                from.name_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
