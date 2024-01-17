use crate::processors::{items::Processors, types};
use ellie_core::{
    definite::Converter,
    defs::{self, Cursor},
};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub condition_filled: bool,
    pub variable: types::TypeProcessor,
    pub variable_pos: Cursor,
    pub target_iterator: types::TypeProcessor,
    pub variable_filled: bool,
    pub parameter: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub body: Vec<Processors>,
    #[serde(skip)]
    pub iterator: Box<crate::iterator::Iterator>,
    pub iterator_pos: Cursor,
    pub brace_count: usize,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<ForLoop, ellie_core::definite::items::for_loop::ForLoop> for ForLoop {
    fn to_definite(self) -> ellie_core::definite::items::for_loop::ForLoop {
        ellie_core::definite::items::for_loop::ForLoop {
            variable: self.variable.current.to_definite(),
            iterator: self.target_iterator.current.to_definite(),
            variable_pos: self.variable_pos,
            iterator_pos: self.iterator_pos,
            body_pos: self.body_pos,
            pos: self.pos,
            parameter: self.parameter,
            inner_page_id: 0,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::for_loop::ForLoop) -> ForLoop {
        ForLoop {
            variable: types::TypeProcessor {
                current: types::Processors::default().from_definite(from.variable),
                ..Default::default()
            },
            target_iterator: types::TypeProcessor {
                current: types::Processors::default().from_definite(from.iterator),
                ..Default::default()
            },
            parameter: from.parameter,
            body_pos: from.body_pos,
            pos: from.pos,
            ..Default::default()
        }
    }
}
