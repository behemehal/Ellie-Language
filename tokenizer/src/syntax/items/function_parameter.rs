use ellie_core::definite::definers::DefinerCollecting;
use ellie_core::definite::Converter;
use ellie_core::defs;
use serde::Deserialize;
use serde::Serialize;

use super::definers::DefinerCollector;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionParameter {
    pub name: String,
    pub reference: bool,
    pub rtype: ellie_core::definite::definers::DefinerCollecting,
    pub name_pos: defs::Cursor,
    pub rtype_pos: defs::Cursor,
}

impl Default for FunctionParameter {
    fn default() -> Self {
        FunctionParameter {
            name: String::new(),
            reference: false,
            rtype: DefinerCollecting::Dynamic,
            name_pos: Default::default(),
            rtype_pos: Default::default(),
        }
    }
}

impl
    Converter<FunctionParameter, ellie_core::definite::items::function_parameter::FunctionParameter>
    for FunctionParameter
{
    fn to_definite(self) -> ellie_core::definite::items::function_parameter::FunctionParameter {
        unreachable!()
    }

    fn from_definite(
        self,
        from: ellie_core::definite::items::function_parameter::FunctionParameter,
    ) -> FunctionParameter {
        FunctionParameter {
            rtype: from.rtype,
            name: from.name,
            reference: false,
            name_pos: from.name_pos,
            rtype_pos: from.rtype_pos,
        }
    }
}
