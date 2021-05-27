use serde::Serialize;

use alloc::string::String;

#[repr(C)]
#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct VariableType {
    pub value_complete: bool,
    pub value: String,
}
