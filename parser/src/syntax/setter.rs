use crate::parser::Collecting;
use crate::syntax::definers;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct Setter {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub rtype_pos: defs::Cursor,
    pub bracket_start_pos: defs::Cursor,
    pub bracket_end_pos: defs::Cursor,
    pub code: Vec<Collecting>,
}

impl Setter {
    pub fn to_definite(self) -> definite::items::setter::Setter {
        definite::items::setter::Setter {
            name: self.name,
            rtype: self.rtype.to_definite(),
            code: self.code.into_iter().map(|x| x.to_definite()).collect(),
        }
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct SetterCollector {
    pub data: Setter,
    pub name_wrote: bool,
    pub type_wrote: bool,
    pub param_bracket_opened: bool,
    pub brace_count: usize,
    pub inside_code: Box<crate::parser::RawParser>,
}

impl SetterCollector {
    pub fn to_definite(self) -> definite::items::setter::Setter {
        self.data.to_definite()
    }
}
