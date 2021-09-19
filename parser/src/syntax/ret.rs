use crate::syntax::types;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Ret {
    pub value: types::Types,
    pub keyword_pos: defs::Cursor,
    pub value_position: defs::Cursor,
    pub pos: defs::Cursor,
}

impl Ret {
    pub fn to_definite(self) -> definite::items::ret::Ret {
        definite::items::ret::Ret {
            value: self.value.to_definite(),
            keyword_pos: self.keyword_pos,
            value_position: self.value_position,
            pos: self.pos,
        }
    }

    pub fn from_definite(self, from: definite::items::ret::Ret) -> Self {
        Ret {
            value: types::Types::default().from_definite(from.value),
            keyword_pos: from.keyword_pos,
            value_position: from.value_position,
            pos: from.pos,
        }
    }
}
