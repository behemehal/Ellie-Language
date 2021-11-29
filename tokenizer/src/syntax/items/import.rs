use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub public: bool,
    pub path_filled: bool,
    pub reference: String,
    pub reference_pos: defs::Cursor,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<Import, ellie_core::definite::items::import::Import> for Import {
    fn to_definite(self) -> ellie_core::definite::items::import::Import {
        ellie_core::definite::items::import::Import {
            path: self.path,
            public: self.public,
            reference: self.reference,
            path_pos: self.path_pos,
            pos: self.pos,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::import::Import) -> Import {
        Import {
            path: from.path,
            public: from.public,
            reference: from.reference,
            path_pos: from.path_pos,
            pos: from.pos,
            ..Default::default()
        }
    }
}
