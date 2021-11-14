use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
    pub comma_start_pos: defs::Cursor,
    pub comma_end_pos: defs::Cursor,
    pub comma_started: bool,
}

impl CharType {
    pub fn to_definite(self) -> definite::types::ellie_char::CharType {
        definite::types::ellie_char::CharType { value: self.value }
    }

    pub fn from_definite(self, from: definite::types::ellie_char::CharType) -> Self {
        CharType {
            value: from.value,
            complete: true,
            ..Default::default()
        }
    }
}
