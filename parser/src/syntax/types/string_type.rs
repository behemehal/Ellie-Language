use alloc::string::String;
use serde::Deserialize;
use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct StringType {
    pub value: String,
    pub complete: bool,
}
