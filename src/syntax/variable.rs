use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct Variable {
    pub name: String,
    pub dynamic:bool,
    pub value: crate::syntax::types::Types,
    pub pos : crate::mapper::defs::Cursor
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct VariableCollector {
    pub initialized: bool,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub r#type: String,
    pub raw_value: String,
    pub data: Variable
}