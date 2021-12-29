use crate::processors::types;
use crate::syntax::items;
use alloc::boxed::Box;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AsKeyword {
    pub target: Box<types::Processors>,
    pub rtype: items::definers::DefinerCollector,
    pub pos: defs::Cursor,
    pub target_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct AsKeywordCollector {
    pub data: AsKeyword,
    pub itered_cache: items::definers::DefinerCollector,
    pub keyword_collected: bool,
    pub keyword_pos: u8,
    pub complete: bool,
}

impl definite::Converter<AsKeywordCollector, definite::types::as_keyword::AsKeyword>
    for AsKeywordCollector
{
    fn to_definite(self) -> definite::types::as_keyword::AsKeyword {
        definite::types::as_keyword::AsKeyword {
            target: Box::new(self.data.target.to_definite()),
            pos: self.data.pos,
            target_pos: self.data.target_pos,
            type_pos: self.data.type_pos,
            rtype: self.data.rtype.definer_type.to_definite(),
        }
    }

    fn from_definite(self, _from: definite::types::as_keyword::AsKeyword) -> AsKeywordCollector {
        todo!()
    }
}
