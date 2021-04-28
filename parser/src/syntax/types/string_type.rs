use serde::Serialize;

use alloc::string::String;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize)]
pub struct StringType {
    pub value: String,
    pub complete: bool,
}