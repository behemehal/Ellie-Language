use crate::{processors::items::Processors, syntax::items::definers};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};


#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor,
    pub rtype: definers::DefinerCollector,
    pub rtype_pos: defs::Cursor,
    // When class is defined, by the design variables are not
    pub body_element_defiener: bool,
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
                        rtype: p.rtype.to_definite(),
                        rtype_pos: p.rtype_pos,
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
                    rtype: definers::DefinerTypes::default().from_definite(p.rtype),
                    rtype_pos: p.rtype_pos,
                    body_element_defiener: false,
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
