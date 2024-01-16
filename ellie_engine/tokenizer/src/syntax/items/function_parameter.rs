use ellie_core::definite::{
    definers::DefinerCollecting,
    items::function_parameter::FunctionParameter as DefiniteFunctionParameter, Converter,
};
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub reference: bool,
    pub rtype: DefinerCollecting,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
    pub hash: usize,
}

impl Default for FunctionParameter {
    fn default() -> Self {
        FunctionParameter {
            rtype: DefinerCollecting::Dynamic,
            name: Default::default(),
            reference: Default::default(),
            name_pos: Default::default(),
            rtype_pos: Default::default(),
            hash: Default::default(),
            //..Default::default() Generates a weird warning
        }
    }
}

impl Converter<FunctionParameter, DefiniteFunctionParameter> for FunctionParameter {
    fn to_definite(self) -> DefiniteFunctionParameter {
        DefiniteFunctionParameter {
            name: self.name,
            rtype: self.rtype,
            name_pos: self.name_pos,
            rtype_pos: self.rtype_pos,
            hash: self.hash,
        }
    }

    fn from_definite(self, from: DefiniteFunctionParameter) -> FunctionParameter {
        FunctionParameter {
            rtype: from.rtype,
            name: from.name,
            reference: false,
            name_pos: from.name_pos,
            rtype_pos: from.rtype_pos,
            hash: from.hash,
        }
    }
}
