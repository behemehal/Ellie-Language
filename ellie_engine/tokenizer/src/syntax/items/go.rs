use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Go {
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<Go, ellie_core::definite::items::go::Go> for Go {
    fn to_definite(self) -> ellie_core::definite::items::go::Go {
        ellie_core::definite::items::go::Go { pos: self.pos }
    }

    fn from_definite(self, from: ellie_core::definite::items::go::Go) -> Go {
        Go {
            pos: from.pos,
            complete: false,
        }
    }
}
