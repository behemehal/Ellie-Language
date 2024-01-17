use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Brk {
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<Brk, ellie_core::definite::items::brk::Brk> for Brk {
    fn to_definite(self) -> ellie_core::definite::items::brk::Brk {
        ellie_core::definite::items::brk::Brk { pos: self.pos }
    }

    fn from_definite(self, from: ellie_core::definite::items::brk::Brk) -> Brk {
        Brk {
            pos: from.pos,
            complete: false,
        }
    }
}
