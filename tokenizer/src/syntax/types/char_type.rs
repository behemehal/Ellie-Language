use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct CharType {
    pub value: char,
    pub complete: bool,
    pub pos: defs::Cursor,
    pub comma_started: bool,
}

impl definite::Converter<CharType, definite::types::ellie_char::CharType> for CharType {
    fn to_definite(self) -> definite::types::ellie_char::CharType {
        definite::types::ellie_char::CharType { value: self.value }
    }

    fn from_definite(self, from: definite::types::ellie_char::CharType) -> Self {
        CharType {
            value: from.value,
            complete: true,
            ..Default::default()
        }
    }
}
