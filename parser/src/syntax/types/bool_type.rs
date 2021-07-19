use alloc::string::String;
use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Hash)]
pub struct BoolType {
    pub value: bool,
    pub raw: String,
}
