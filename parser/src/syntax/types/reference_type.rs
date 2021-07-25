use crate::syntax::types;
use ellie_core::defs;
use serde::Serialize;

use alloc::boxed::Box;
use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct GetterChain {
    pub value: String,
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct SetterChain {
    pub name: String,
    pub value: types::Types,
    pub name_set: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum ChainType {
    Getter(GetterChain),
    Setter(SetterChain),
    FunctionCall(types::function_call::FunctionCallCollector),
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct Chain {
    pub pos: defs::Cursor,
    pub value: ChainType,
}

impl Default for ChainType {
    fn default() -> Self {
        ChainType::Getter(GetterChain::default())
    }
}

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct ReferenceType {
    pub reference: Box<types::Types>,
    pub on_dot: bool,
    pub chain: Vec<Chain>,
}
