use serde::Serialize;
use alloc::string::String;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct BoolType {
    pub value: bool,
    pub raw: String
}
