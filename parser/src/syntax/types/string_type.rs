use serde::Serialize;

use alloc::string::String;

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Hash)]
pub struct StringType {
    pub value: String,
    pub complete: bool,
}
