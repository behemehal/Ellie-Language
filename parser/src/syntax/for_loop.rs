use crate::alloc::boxed::Box;
use crate::alloc::string::String;
use crate::alloc::vec::Vec;
use crate::parser::Collecting;
use crate::syntax::{types, variable};
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForLoop {
    pub parameter: Box<types::Types>,
    pub parameter_pos: defs::Cursor,
    pub code: Vec<Collecting>,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ForLoopCollector {
    pub parameters_collected: bool,
    pub cloak_itered_data: variable::VariableCollector,
    pub brace_count: usize,
    pub raw_parameter: String,
    pub has_code: bool,
    pub data: ForLoop,
    pub inside_object_start: bool,
    pub inside_object_count: i64,
    pub code: Box<crate::parser::RawParser>,
}

impl ForLoopCollector {
    pub fn to_definite(self) -> definite::items::for_loop::ForLoop {
        definite::items::for_loop::ForLoop {
            parameter: Box::new(self.data.parameter.to_definite()),
            parameter_pos: self.data.parameter_pos,
            code: self
                .data
                .code
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            pos: self.data.pos,
        }
    }

    pub fn from_definite(self, from: definite::items::for_loop::ForLoop) -> Self {
        ForLoopCollector {
            data: ForLoop {
                parameter: Box::new(types::Types::default().from_definite(*from.parameter)),
                parameter_pos: from.parameter_pos,
                code: from
                    .code
                    .into_iter()
                    .map(|x| Collecting::default().from_definite(x))
                    .collect(),
                pos: from.pos,
            },
            ..Default::default()
        }
    }
}
