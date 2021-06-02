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
<<<<<<< HEAD
    pub rtype: Vec<DefinerCollecting>,
=======
<<<<<<< HEAD
    pub rtype: Vec<DefinerCollecting>,
=======
    pub r#type: Vec<DefinerCollecting>,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
    pub rtype: Vec<DefinerCollecting>,
>>>>>>> FFI
    pub bracket_inserted: bool,
    pub at_comma: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct ArrayType {
    pub complete: bool,
<<<<<<< HEAD
<<<<<<< HEAD
    pub rtype: Box<DefinerCollecting>,
=======
<<<<<<< HEAD
    pub rtype: Box<DefinerCollecting>,
=======
    pub r#type: Box<DefinerCollecting>,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
    pub rtype: Box<DefinerCollecting>,
>>>>>>> FFI
    pub bracket_inserted: bool,
    pub len: crate::syntax::types::Types,
    pub at_comma: bool,
    pub typed: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct GrowableArrayType {
    pub complete: bool,
<<<<<<< HEAD
<<<<<<< HEAD
    pub rtype: Box<DefinerCollecting>,
=======
<<<<<<< HEAD
    pub rtype: Box<DefinerCollecting>,
=======
    pub r#type: Box<DefinerCollecting>,
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
    pub rtype: Box<DefinerCollecting>,
>>>>>>> FFI
    pub bracket_inserted: bool,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct GenericType {
    pub rtype: String,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum DefinerCollecting {
    Array(ArrayType),
    GrowableArray(GrowableArrayType),
    Generic(GenericType),
    Function(FunctionType),
    Cloak(CloakType),
<<<<<<< HEAD
<<<<<<< HEAD
    Dynamic,
=======
<<<<<<< HEAD
    Dynamic,
=======
    Dynamic
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
    Dynamic,
>>>>>>> FFI
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
<<<<<<< HEAD
            DefinerCollecting::DynamicArray(data) => !data.complete,
<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
            DefinerCollecting::Generic(data) => data.rtype.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Dynamic => false,
<<<<<<< HEAD
=======
=======
            DefinerCollecting::Generic(data) => data.r#type.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Dynamic => false
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
            DefinerCollecting::GrowableArray(data) => !data.complete,
            DefinerCollecting::Generic(data) => data.rtype.is_empty(),
            DefinerCollecting::Function(data) => !data.complete,
            DefinerCollecting::Cloak(data) => !data.complete,
            DefinerCollecting::Dynamic => false,
>>>>>>> FFI
        }
    }

    pub fn is_definer_complete(&self) -> bool {
        match self {
            DefinerCollecting::Array(data) => data.complete,
<<<<<<< HEAD
            DefinerCollecting::DynamicArray(data) => data.complete,
<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
            DefinerCollecting::Generic(data) => !data.rtype.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Dynamic => true,
<<<<<<< HEAD
=======
=======
            DefinerCollecting::Generic(data) => !data.r#type.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Dynamic => true
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
            DefinerCollecting::GrowableArray(data) => data.complete,
            DefinerCollecting::Generic(data) => !data.rtype.is_empty(),
            DefinerCollecting::Function(data) => data.complete,
            DefinerCollecting::Cloak(data) => data.complete,
            DefinerCollecting::Dynamic => true,
        }
    }

    pub fn is_generic(&self) -> bool {
        match self {
            DefinerCollecting::Array(_) => false,
            DefinerCollecting::GrowableArray(_) => false,
            DefinerCollecting::Generic(_) => true,
            DefinerCollecting::Function(_) => false,
            DefinerCollecting::Cloak(_) => false,
            DefinerCollecting::Dynamic => true,
>>>>>>> FFI
        }
    }

    pub fn raw_name(&self) -> String {
        match self {
            DefinerCollecting::Array(_) => "array".to_string(),
<<<<<<< HEAD
            DefinerCollecting::DynamicArray(_) => "dynamic_array".to_string(),
<<<<<<< HEAD
=======
<<<<<<< HEAD
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Dynamic => "dynamic".to_string(),
<<<<<<< HEAD
=======
=======
            DefinerCollecting::Generic(data) => data.r#type.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Dynamic => "dynamic".to_string()
>>>>>>> cc9fcde44426e37e6f25176d90bb7b1900459e53
>>>>>>> 538bf62052a58de02e9b66352faed443e69c3ea2
=======
            DefinerCollecting::GrowableArray(_) => "dynamic_array".to_string(),
            DefinerCollecting::Generic(data) => data.rtype.clone(),
            DefinerCollecting::Function(_) => "function".to_string(),
            DefinerCollecting::Cloak(_) => "cloak".to_string(),
            DefinerCollecting::Dynamic => "dynamic".to_string(),
>>>>>>> FFI
        }
    }
}
