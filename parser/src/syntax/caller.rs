
#[derive(PartialEq, Debug, Clone)]
pub enum CallerType {
    FunctionCaller,
    VariableCaller,
    Unknown,
}


#[derive(PartialEq, Debug, Clone)]
pub struct Caller {
    pub initialized: bool,
    pub name: String,
    pub named: bool,
    pub value_complete: bool,
    pub r#type: CallerType,
    pub raw_value: String,
    pub value: crate::syntax::types::Types,
    pub pos: crate::parser::defs::Cursor,
}
