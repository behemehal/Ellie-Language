use crate::parser::Collecting;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Constructor {
    pub name: String,                          //Function Name string
    pub parameters: Vec<ConstructorParameter>, //Parameter vector
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    pub parameters_pos: defs::Cursor,
    pub pos: defs::Cursor,
}

impl Constructor {
    pub fn to_definite(self) -> definite::items::constructor::Constructor {
        definite::items::constructor::Constructor {
            name: self.name,
            parameters: self
                .parameters
                .into_iter()
                .map(|x| definite::items::constructor::ConstructorParameter {
                    name: x.name,
                    pos: x.pos,
                })
                .collect(),
            inside_code: self
                .inside_code
                .into_iter()
                .map(|x| x.to_definite())
                .collect(),
            name_pos: self.name_pos,
            parameters_pos: self.parameters_pos,
            pos: self.pos,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct ConstructorCollector {
    pub data: Constructor,
    pub named: bool,
    pub parameter_wrote: bool,
    pub collecting: bool,
    pub brace_count: usize,
    pub has_code: bool,
    pub at_comma: bool,
    pub code: String,
}

impl ConstructorCollector {
    pub fn to_definite(self) -> definite::items::constructor::Constructor {
        self.data.to_definite()
    }

    pub fn is_parameters_complete(&self) -> bool {
        if self.data.parameters.is_empty() {
            true
        } else {
            !self.data.parameters[self.data.parameters.len() - 1]
                .name
                .is_empty()
        }
    }
}
