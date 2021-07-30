use alloc::string::String;
use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct BoolType {
    pub value: bool,
    pub raw: String,
}
