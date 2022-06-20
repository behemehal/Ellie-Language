use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Import {
    pub path: String,
    pub link_module: bool,
    pub public: bool,
    pub path_filled: bool,
    pub reference: String,
    pub reference_pos: defs::Cursor,
    pub path_pos: defs::Cursor,
    pub pos: defs::Cursor,
    pub complete: bool,
    pub hash: usize,
}

impl Converter<Import, ellie_core::definite::items::import::Import> for Import {
    fn to_definite(self) -> ellie_core::definite::items::import::Import {
        ellie_core::definite::items::import::Import {
            path: self.path,
            link_module: self.link_module,
            public: self.public,
            reference: self.reference,
            path_pos: self.path_pos,
            pos: self.pos,
            hash: self.hash,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::import::Import) -> Import {
        Import {
            path: from.path,
            link_module: self.link_module,
            public: from.public,
            reference: from.reference,
            path_pos: from.path_pos,
            pos: from.pos,
            hash: self.hash,
            ..Default::default()
        }
    }
}
