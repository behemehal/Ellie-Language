use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::{
    items::constructor_parameter::ConstructorParameter as DefiniteConstructorParameter, Converter,
};
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub rtype: ellie_core::definite::definers::DefinerCollecting,
    pub pos: defs::Cursor,
    pub hash: usize,
}

impl Default for ConstructorParameter {
    fn default() -> Self {
        ConstructorParameter {
            rtype: DefinerCollecting::Dynamic,
            name: Default::default(),
            pos: Default::default(),
            hash: Default::default(),
        }
    }
}

impl Converter<ConstructorParameter, DefiniteConstructorParameter> for ConstructorParameter {
    fn to_definite(self) -> DefiniteConstructorParameter {
        DefiniteConstructorParameter {
            name: self.name,
            rtype: self.rtype,
            hash: self.hash,
            pos: self.pos,
        }
    }

    fn from_definite(self, from: DefiniteConstructorParameter) -> ConstructorParameter {
        ConstructorParameter {
            rtype: from.rtype,
            name: from.name,
            hash: from.hash,
            pos: from.pos,
        }
    }
}
