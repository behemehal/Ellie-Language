use crate::{processors::items::Processors, processors::types};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ForLoop {
    pub condition_filled: bool,
    pub variable: types::TypeProcessor,
    pub target_iterator: types::TypeProcessor,
    pub variable_filled: bool,
    pub parameter: defs::Cursor,
    pub body_pos: defs::Cursor,
    pub body: Vec<Processors>,
    pub iterator: Box<crate::iterator::Iterator>,
    pub brace_count: usize,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<ForLoop, ellie_core::definite::items::for_loop::ForLoop> for ForLoop {
    fn to_definite(self) -> ellie_core::definite::items::for_loop::ForLoop {
        ellie_core::definite::items::for_loop::ForLoop {
            variable: self.variable.current.to_definite(),
            iterator: self.target_iterator.current.to_definite(),
            parameter: self.parameter,
            body_pos: self.body_pos,
            body: self.body.into_iter().map(|x| x.to_definite()).collect(),
            pos: self.pos,
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
            body: from
                .body
                .into_iter()
                .map(|x| Processors::default().from_definite(x))
                .collect(),
            pos: from.pos,
            ..Default::default()
        }
    }
}
