use crate::definite::items::{constructor, function, getter, setter, variable};
use crate::defs;
use libc::c_char;
use ellie_core::definite::items;


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
        constructor: todo!(),
        generic_definings: todo!(),
        properties: todo!(),
        getters: todo!(),
        setters: todo!(),
        methods: todo!(),
        name_pos: defs::Cursor {
            range_start: defs::CursorPosition(from.name_pos.range_start.0, from.name_pos.range_start.1),
            range_end: defs::CursorPosition(from.name_pos.range_end.0, from.name_pos.range_end.1),
        },
        pos: defs::Cursor {
            range_start: defs::CursorPosition(from.pos.range_start.0, from.pos.range_start.1),
            range_end: defs::CursorPosition(from.pos.range_end.0, from.pos.range_end.1),
        },
    }
}
