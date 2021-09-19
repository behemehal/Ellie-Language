use alloc::string::String;
use ellie_core::definite;
use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct VariableType {
    pub value: String,
    pub pos: defs::Cursor,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize, Deserialize)]
pub struct VariableTypeCollector {
    pub data: VariableType,
    pub value_complete: bool,
}

impl VariableTypeCollector {
    pub fn to_definite(self) -> definite::types::variable::VariableType {
        definite::types::variable::VariableType {
            value: self.data.value,
            pos: self.data.pos,
        }
    }

    pub fn from_definite(self, from: definite::types::variable::VariableType) -> Self {
        VariableTypeCollector {
            data: VariableType {
                value: from.value,
                pos: from.pos,
            },
            value_complete: true,
        }
    }
}
