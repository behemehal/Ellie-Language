use serde::Serialize;

use alloc::string::String;


#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct VariableType {
    pub value_complete: bool,
    pub value: String,
}
