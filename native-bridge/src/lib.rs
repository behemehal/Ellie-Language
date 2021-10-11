pub mod type_definings;
pub mod types;
use libc::c_char;

#[repr(C)]
pub struct EllieException {
    pub code: u32,
    pub error_name: *mut c_char,
    pub reason: *mut c_char,
}

#[repr(C)]
pub enum NativeFunctionReturn {
    Exception(EllieException),
    Response(types::EllieType),
}

#[repr(C)]
pub struct NativeFunctionParameter {
    pub name: *mut c_char,
    pub ptype: types::EllieType,
}

#[repr(C)]
pub struct NativeFunction {
    pub name: *mut c_char,
    pub return_type: type_definings::EllieTypeDefinings,
    pub parameters: *mut NativeFunctionParameter,
}

#[must_use]
#[repr(C)]
pub struct NativeLib {
    pub name: *mut c_char,
    pub on_load: extern "C" fn(),
    pub on_function_call: extern "C" fn(function: NativeFunction) -> NativeFunctionReturn,
}
