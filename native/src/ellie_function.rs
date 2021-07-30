use libc::c_char;
use crate::ellie_type_define;
use crate::ellie_types;

#[repr(C)]
pub struct EllieFunctionDefineParameter {
    pub name: *mut c_char,
    pub data_type: ellie_type_define::EllieTypeDefines
}

#[repr(C)]
pub struct EllieFunction {
    pub name: *mut c_char,
    pub params: *mut EllieFunctionDefineParameter,
    pub returning: ellie_type_define::EllieTypeDefines,
    pub on_call: extern "C" fn(*mut ellie_types::EllieTypes) -> ellie_types::EllieTypes
}

#[no_mangle]
pub extern "C" fn new_ellie_fn(name: *mut c_char, params: *mut EllieFunctionDefineParameter, returning: ellie_type_define::EllieTypeDefines, on_call: extern "C" fn(*mut ellie_types::EllieTypes) -> ellie_types::EllieTypes) -> EllieFunction {
    EllieFunction {
        name: name,
        params: params,
        returning: returning,
        on_call: on_call,
    }
} 