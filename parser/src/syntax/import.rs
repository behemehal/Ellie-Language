use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Import {
    pub path: String,
    pub pri_keyword: bool,
    pub native: bool,
    pub public: bool,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

impl Import {
    pub fn to_definite(self) -> definite::items::import::Import {
        definite::items::import::Import {
            path: self.path,
            pri_keyword: self.pri_keyword,
            native: self.native,
            public: self.public,
            path_pos: self.path_pos,
            pos: self.pos,
        }
    }

    pub fn from_definite(self, from: definite::items::import::Import) -> Self {
        Import {
            path: from.path,
            pri_keyword: from.pri_keyword,
            native: from.native,
            public: from.public,
            path_pos: from.path_pos,
            pos: from.pos,
        }
    }
}
