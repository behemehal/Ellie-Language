use ellie_core::definite;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Eq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
}

impl CharType {
    pub fn to_definite(self) -> definite::types::ellie_char::CharType {
        definite::types::ellie_char::CharType { value: self.value }
    }
}
