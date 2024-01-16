use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

use crate::processors::items::Processors;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Constructor {
    pub parameters: Vec<ConstructorParameter>,
    pub parameter_collected: bool,
    pub comma: bool,
    pub code_started: bool,
    pub continuum_collected: bool,
    pub inside_code: Vec<Processors>,
    pub name_pos: defs::Cursor,
    pub parameters_pos: defs::Cursor,
    pub brace_count: usize,
    #[serde(skip)]
    pub iterator: Box<crate::iterator::Iterator>,
    pub pos: defs::Cursor,
    pub complete: bool,
}

impl Converter<Constructor, ellie_core::definite::items::constructor::Constructor> for Constructor {
    fn to_definite(self) -> ellie_core::definite::items::constructor::Constructor {
        ellie_core::definite::items::constructor::Constructor {
            parameters: self
                .parameters
                .into_iter()
                .map(
                    |p| ellie_core::definite::items::constructor::ConstructorParameter {
                        name: p.name,
                        pos: p.pos,
                    },
                )
                .collect(),
            name_pos: self.name_pos,
            parameters_pos: self.parameters_pos,
            pos: self.pos,
            inner_page_id: 0,
            class_hash: 0,
        }
    }

    fn from_definite(
        self,
        from: ellie_core::definite::items::constructor::Constructor,
    ) -> Constructor {
        Constructor {
            parameters: from
                .parameters
                .into_iter()
                .map(|p| ConstructorParameter {
                    name: p.name,
                    pos: p.pos,
                })
                .collect(),
            inside_code: vec![],
            name_pos: from.name_pos,
            parameters_pos: from.parameters_pos,
            pos: from.pos,
            ..Default::default()
        }
    }
}
