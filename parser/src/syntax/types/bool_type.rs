use alloc::string::String;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct BoolType {
    pub value: bool,
    pub raw: String,
}
