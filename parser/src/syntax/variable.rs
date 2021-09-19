use crate::syntax::{definers, types};
use alloc::string::String;
use ellie_core::{definite, defs};
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct Variable {
    pub name: String,
    pub dynamic: bool,
    pub constant: bool,
    pub public: bool,
    pub value: types::Types,
    pub pos: defs::Cursor,
    pub name_pos: defs::Cursor,
    pub value_pos: defs::Cursor,
    pub type_pos: defs::Cursor,
    pub rtype: definers::DefinerCollecting,
    pub hash: String,
}

impl Variable {
    pub fn to_definite(self) -> definite::items::variable::Variable {
        definite::items::variable::Variable {
            name: self.name,
            dynamic: self.dynamic,
            constant: self.constant,
            public: self.public,
            value: self.value.to_definite(),
            pos: self.pos,
            name_pos: self.name_pos,
            value_pos: self.value_pos,
            type_pos: self.type_pos,
            rtype: self.rtype.to_definite(),
            hash: self.hash,
        }
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize, Default)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub ignore_existence: bool,
    pub value_complete: bool,
    pub raw_value: String,
    pub collected_value: String,
    pub data: Variable,
}

impl VariableCollector {
    pub fn to_definite(self) -> definite::items::variable::Variable {
        self.data.to_definite()
    }

    pub fn from_definite(self, from: definite::items::variable::Variable) -> Self {
        VariableCollector {
            named: true,
            typed: true,
            value_complete: true,
            data: Variable {
                name: from.name,
                dynamic: from.dynamic,
                constant: from.constant,
                public: from.public,
                value: types::Types::default().from_definite(from.value),
                pos: from.pos,
                name_pos: from.name_pos,
                value_pos: from.value_pos,
                type_pos: from.type_pos,
                rtype: definers::DefinerCollecting::default().from_definite(from.rtype),
                hash: from.hash,
            },
            ..Default::default()
        }
    }
}
