use crate::syntax;
use crate::parser;

#[derive(PartialEq, Debug, Clone)]
pub enum CallerType {
    FunctionCaller,
    VariableCaller,
    Unknown,
}

#[derive(PartialEq, Debug, Clone)]
pub struct Caller {
    pub initialized: bool,
    pub name: *const c_char,
    pub named: bool,
    pub value_complete: bool,
    pub r#type: CallerType,
    pub raw_value: *const c_char,
    pub value: syntax::types::Types,
    pub pos: parser::defs::Cursor,
}
