use crate::processors::{items::Processors, types};
use ellie_core::{
    definite::Converter,
    defs::{self, Cursor},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Loop {
    pub condition: types::TypeProcessor,
    pub condition_pos: defs::Cursor,
    pub condition_filled: bool,
    pub body_pos: defs::Cursor,
    pub body: Vec<Processors>,
    pub iterator: Box<crate::iterator::Iterator>,
    pub iterator_pos: Cursor,
    pub brace_count: usize,
    pub pos: defs::Cursor,
    pub hash: usize,
    pub complete: bool,
}

impl Converter<Loop, ellie_core::definite::items::loop_type::Loop> for Loop {
    fn to_definite(self) -> ellie_core::definite::items::loop_type::Loop {
        ellie_core::definite::items::loop_type::Loop {
            body_pos: self.body_pos,
            pos: self.pos,
            inner_page_id: 0,
            condition: self.condition.current.to_definite(),
            hash: self.hash,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::loop_type::Loop) -> Loop {
        Loop {
            body_pos: from.body_pos,
            pos: from.pos,
            hash: from.hash,
            ..Default::default()
        }
    }
}
