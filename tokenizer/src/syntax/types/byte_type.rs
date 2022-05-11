use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct ByteType {
    pub value: i8,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl definite::Converter<ByteType, definite::types::byte::ByteType> for ByteType {
    fn to_definite(self) -> definite::types::byte::ByteType {
        definite::types::byte::ByteType {
            value: self.value,
            pos: self.pos,
        }
    }

    fn from_definite(self, from: definite::types::byte::ByteType) -> Self {
        ByteType {
            value: from.value,
            pos: from.pos,
            complete: true,
        }
    }
}
