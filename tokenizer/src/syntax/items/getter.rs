use crate::{processors::items::Processors, syntax::items::definers};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Getter {
    pub name_collected: bool,
    pub return_keyword_collected: bool,
    pub return_collected: bool,
    pub brace_count: usize,
    pub iterator: Box<crate::iterator::Iterator>,

    pub complete: bool,
    pub name: String,
    pub name_pos: defs::Cursor,
    pub public: bool,
    pub return_type: definers::DefinerCollector,
    pub return_pos: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub body: Vec<Processors>,
    pub pos: defs::Cursor,
    pub hash: u64,
}

impl Converter<Getter, ellie_core::definite::items::getter::Getter> for Getter {
    fn to_definite(self) -> ellie_core::definite::items::getter::Getter {
        ellie_core::definite::items::getter::Getter {
            name: self.name,
            return_type: self.return_type.definer_type.to_definite(),
            public: self.public,
            name_pos: self.name_pos,
            return_pos: self.return_pos,
            pos: self.pos,
            body_pos: self.body_pos,
            hash: self.hash,
            inner_page_id: 0,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::getter::Getter) -> Getter {
        Getter {
            name: from.name,
            name_pos: from.name_pos,
            public: from.public,
            return_type: definers::DefinerCollector {
                definer_type: definers::DefinerTypes::default().from_definite(from.return_type),
                complete: true,
            },
            return_pos: from.return_pos,
            body_pos: from.body_pos,
            body: vec![],
            pos: from.pos,
            hash: from.hash,
            ..Default::default()
        }
    }
}
