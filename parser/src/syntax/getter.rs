use crate::parser::{Collecting, RawParser};
use crate::syntax::definers;
use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]

pub struct Getter {
    pub name: String,
    pub name_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub rtype_pos: defs::Cursor,
    pub bracket_start_pos: defs::Cursor,
    pub bracket_end_pos: defs::Cursor,
    pub code: Vec<Collecting>,
}

impl Getter {
    pub fn to_definite(self) -> definite::items::getter::Getter {
        definite::items::getter::Getter {
            name: self.name,
            rtype: self.rtype.to_definite(),
            code: self.code.into_iter().map(|x| x.to_definite()).collect(),
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetterCollector {
    pub data: Getter,
    pub name_wrote: bool,
    pub type_wrote: bool,
    pub param_bracket_opened: bool,
    pub brace_count: usize,
    pub inside_code: Box<RawParser>,
}

impl GetterCollector {
    pub fn to_definite(self) -> definite::items::getter::Getter {
        self.data.to_definite()
    }
}
