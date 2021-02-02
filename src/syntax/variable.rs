#[derive(PartialEq, Debug, Clone)]
pub struct VariableCollector {
    pub initialized: bool,
    pub dynamic:bool,
    pub name: String,
    pub named: bool,
    pub typed: bool,
    pub value_complete: bool,
    pub r#type: String,
    pub raw_value: String,
    pub value: crate::syntax::types::Types,
    pub pos : crate::mapper::defs::Cursor
}

impl Default for VariableCollector {
    fn default() -> VariableCollector {
        VariableCollector {
            initialized: false,
            dynamic: false,
            named: false,
            typed: false, 
            value_complete: false,
            name: "".to_string(),
            r#type: "".to_string(),
            raw_value: "".to_string(),
            value: crate::syntax::types::Types::Null,
            pos: crate::mapper::defs::Cursor::default()
        }
    }
}