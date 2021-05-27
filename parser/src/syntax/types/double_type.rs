use serde::Serialize;

use alloc::string::String;

#[repr(C)]
#[no_mangle]
#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct DoubleType {
    pub value: f32,
    pub raw_value: String,
    pub complete: bool,
}
