use serde::Serialize;
use alloc::string::String;
use alloc::boxed::Box;

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct FunctionType {
    pub complete: bool,
    pub params: Vec<Collecting>,
    pub returning: Box<Collecting>,
    pub return_typed: bool,
    pub return_keyword: i8,
    pub parameter_collected: bool,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct CloakType {
    pub complete: bool,
    pub data: Vec<Collecting>,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct ArrayType {
    pub complete: bool,
    pub r#type: Box<Collecting>,
    pub bracket_inserted: bool,
    pub len: crate::syntax::types::Types,
    pub at_comma: bool,
    pub typed: bool
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct GenericType {
    pub r#type: String
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Collecting {
    Array(ArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType)
}

impl Default for Collecting {
    fn default() -> Self {
        Collecting::Generic(GenericType::default())
    }
}

impl Collecting {
    pub fn is_type_empty(&self) -> bool {
        match self {
            Collecting::Array(data) => data.complete,
            Collecting::Generic(data) => data.r#type.is_empty(),
            Collecting::Function(data) => data.complete,
            Collecting::Cloak(_) => false
        }
    }

    pub fn raw_name(&self) -> String {
        match self {
            Collecting::Array(_) => "array".to_string(),
            Collecting::Generic(data) => data.r#type.clone(),
            Collecting::Function(_) => "function".to_string(),
            Collecting::Cloak(_) => "cloak".to_string()
        }
    }
}
