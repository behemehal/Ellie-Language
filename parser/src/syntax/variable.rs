use crate::syntax::definers;
use ellie_core::defs;
use serde::Serialize;

use alloc::string::String;


#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct Variable {
    pub name: String,
    pub dynamic: bool,
    pub public: bool,
    pub value: crate::syntax::types::Types,
    pub pos: defs::Cursor,
}


#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
<<<<<<< HEAD
    pub rtype: definers::DefinerCollecting,
=======
    pub r#type: definers::DefinerCollecting,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
    pub raw_value: String,
    pub data: Variable,
}
