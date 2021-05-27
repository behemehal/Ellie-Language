use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::Serialize;


#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct FunctionType {
    pub complete: bool,
    pub params: Vec<DefinerCollecting>,
    pub returning: Box<DefinerCollecting>,
    pub return_typed: bool,
    pub return_keyword: i8,
    pub parameter_collected: bool,
    pub bracket_inserted: bool,
    pub at_comma: bool,
}


#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct CloakType {
    pub complete: bool,
<<<<<<< HEAD
    pub rtype: Vec<DefinerCollecting>,
=======
    pub r#type: Vec<DefinerCollecting>,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
    pub bracket_inserted: bool,
    pub at_comma: bool,
}


#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct ArrayType {
    pub complete: bool,
<<<<<<< HEAD
    pub rtype: Box<DefinerCollecting>,
=======
    pub r#type: Box<DefinerCollecting>,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
    pub bracket_inserted: bool,
    pub len: crate::syntax::types::Types,
    pub at_comma: bool,
    pub typed: bool,
}


#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct DynamicArrayType {
    pub complete: bool,
<<<<<<< HEAD
    pub rtype: Box<DefinerCollecting>,
=======
    pub r#type: Box<DefinerCollecting>,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
    pub bracket_inserted: bool,
}


#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct GenericType {
    pub rtype: String,
}


#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum DefinerCollecting {
    Array(ArrayType),
    DynamicArray(DynamicArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
<<<<<<< HEAD
    Dynamic,
=======
    Dynamic
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
}

impl Default for DefinerCollecting {
    fn default() -> Self {
        DefinerCollecting::Generic(GenericType::default())
    }
}

impl DefinerCollecting {
    pub fn is_type_empty(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => !data.complete,
            DefinerCollecting::DynamicArray(data) => !data.complete,
<<<<<<< HEAD
            DefinerCollecting::Generic(data) => data.rtype.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Dynamic => false,
=======
            DefinerCollecting::Generic(data) => data.r#type.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Dynamic => false
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
        }
    }

    pub fn is_definer_complete(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => data.complete,
            DefinerCollecting::DynamicArray(data) => data.complete,
<<<<<<< HEAD
            DefinerCollecting::Generic(data) => !data.rtype.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Dynamic => true,
=======
            DefinerCollecting::Generic(data) => !data.r#type.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Dynamic => true
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
        }
    }

    pub fn raw_name(&self) -> String {
        match self {
            DefinerCollecting::Array(_) => "array".to_string(),
            DefinerCollecting::DynamicArray(_) => "dynamic_array".to_string(),
<<<<<<< HEAD
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Dynamic => "dynamic".to_string(),
=======
            DefinerCollecting::Generic(data) => data.r#type.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Dynamic => "dynamic".to_string()
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
        }
    }
}
