use crate::processors::types::{Processors, TypeProcessor};
use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Ret {
    pub value: TypeProcessor,
    pub keyword_pos: defs::Cursor,
    pub not_empty: bool,
    pub value_position: defs::Cursor,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<Ret, ellie_core::definite::items::ret::Ret> for Ret {
    fn to_definite(self) -> ellie_core::definite::items::ret::Ret {
        ellie_core::definite::items::ret::Ret {
            value: self.value.current.to_definite(),
            keyword_pos: self.keyword_pos,
            value_position: self.value_position,
            pos: self.pos,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::ret::Ret) -> Ret {
        Ret {
            value: TypeProcessor {
                current: Processors::default().from_definite(from.value),
                ignore: false,
            },
            keyword_pos: self.keyword_pos,
            value_position: self.value_position,
            pos: self.pos,
            complete: true,
            not_empty: false,
        }
    }
}
