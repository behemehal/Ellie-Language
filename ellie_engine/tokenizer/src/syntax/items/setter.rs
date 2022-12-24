use std::vec;

use crate::{processors::items::Processors, syntax::items::definers};
use ellie_core::{definite::Converter, defs};
use serde::{Deserialize, Serialize};

use super::function::FunctionParameter;

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Setter {
    pub name_collected: bool,
    pub brace_count: usize,
    pub parameters_collected: bool,
    pub code_start_collected: bool,
    pub iterator: Box<crate::iterator::Iterator>,

    pub key_collected: bool,
    pub complete: bool,
    pub name: String,
    pub name_pos: defs::Cursor,
    pub public: bool,
    pub parameters: Vec<FunctionParameter>,
    pub parameters_pos: defs::Cursor,

    pub body_pos: defs::Cursor,
    pub body: Vec<Processors>,
    pub pos: defs::Cursor,
    pub hash: usize,
}

impl Converter<Setter, ellie_core::definite::items::setter::Setter> for Setter {
    fn to_definite(self) -> ellie_core::definite::items::setter::Setter {
        ellie_core::definite::items::setter::Setter {
            name: self.name,
            public: self.public,
            name_pos: self.name_pos,
            pos: self.pos,
            body_pos: self.body_pos,
            hash: self.hash,
            inner_page_id: 0,
            rtype_pos: self.parameters[0].rtype_pos,
            file_keys: Vec::new(),
            param_name_pos: self.parameters[0].name_pos,
            rtype: self.parameters[0].rtype.definer_type.clone().to_definite(),
            param_name: self.parameters[0].clone().name,
            parameters_pos: self.parameters_pos,
        }
    }

    fn from_definite(self, from: ellie_core::definite::items::setter::Setter) -> Setter {
        Setter {
            name: from.name,
            name_pos: from.name_pos,
            public: from.public,
            body_pos: from.body_pos,
            body: vec![],
            pos: from.pos,
            hash: from.hash,
            parameters: vec![FunctionParameter {
                name: from.param_name,
                name_pos: from.param_name_pos,
                rtype_pos: from.rtype_pos,
                rtype: definers::DefinerCollector {
                    definer_type: definers::DefinerTypes::Dynamic.from_definite(from.rtype),
                    complete: true,
                },
                multi_capture: false,
            }],
            parameters_pos: from.parameters_pos,
            ..Default::default()
        }
    }
}
