use alloc::string::String;
use serde::Serialize;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct StringType {
    pub value: String,
    pub complete: bool,
}
