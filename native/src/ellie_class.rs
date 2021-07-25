use libc::c_char;
use crate::ellie_type_define;
use crate::ellie_types;

pub struct EllieClassConstructor {
    pub params: *mut *mut c_char
}

struct EllieClass {
    pub name: *mut c_char,
    pub constructor: EllieClassConstructor,
}

impl EllieClass {
    pub extern "C" fn new(name: *const c_char) -> Self {
        EllieClass {
            name: name,
            constructor: EllieClassConstructor {
                params: [],
            }
        }
    }
}