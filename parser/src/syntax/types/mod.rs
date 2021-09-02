pub mod arithmetic_type;
pub mod array_type;
pub mod arrow_function;
pub mod bool_type;
pub mod brace_reference_type;
pub mod char_type;
pub mod cloak_type;
pub mod collective_type;
pub mod comparison_type;
pub mod constructed_class;
pub mod double_type;
pub mod float_type;
pub mod function_call;
pub mod integer_type;
pub mod logical_type;
pub mod negative_type;
pub mod operator_type;
pub mod reference_type;
pub mod string_type;
pub mod variable_type;

use alloc::string::{String, ToString};
use ellie_core::definite;
use enum_as_inner::EnumAsInner;
use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner, Deserialize)]
pub enum Types {
    Integer(integer_type::IntegerTypeCollector),
    Float(float_type::FloatTypeCollector),
    Bool(bool_type::BoolType),
    String(string_type::StringTypeCollector),
    Char(char_type::CharType),
    Collective(collective_type::CollectiveCollector),
    Reference(reference_type::ReferenceTypeCollector),
    Operator(operator_type::OperatorTypeCollector),
    Cloak(cloak_type::CloakTypeCollector),
    Array(array_type::ArrayTypeCollector),
    ArrowFunction(arrow_function::ArrowFunctionCollector),
    ConstructedClass(constructed_class::ConstructedClassCollector),
    FunctionCall(function_call::FunctionCallCollector),
    Void,
    Negative(negative_type::Negative),
    VariableType(variable_type::VariableTypeCollector),
    Null,
}

impl Types {
    pub fn to_definite(self) -> definite::types::Types {
        match self {
            Types::Integer(e) => definite::types::Types::Integer(e.to_definite()),
            Types::Float(e) => definite::types::Types::Float(e.to_definite()),
            Types::Bool(e) => definite::types::Types::Bool(e.to_definite()),
            Types::String(e) => definite::types::Types::String(e.to_definite()),
            Types::Char(e) => definite::types::Types::Char(e.to_definite()),
            Types::Collective(e) => definite::types::Types::Collective(e.to_definite()),
            Types::Reference(e) => definite::types::Types::Reference(e.to_definite()),
            Types::Operator(e) => definite::types::Types::Operator(e.to_definite()),
            Types::Cloak(e) => definite::types::Types::Cloak(e.to_definite()),
            Types::Array(e) => definite::types::Types::Array(e.to_definite()),
            Types::ArrowFunction(e) => definite::types::Types::ArrowFunction(e.to_definite()),
            Types::ConstructedClass(e) => definite::types::Types::ConstructedClass(e.to_definite()),
            Types::FunctionCall(e) => definite::types::Types::FunctionCall(e.to_definite()),
            Types::Void => definite::types::Types::Void,
            Types::Negative(e) => definite::types::Types::Negative(e.to_definite()),
            Types::VariableType(e) => definite::types::Types::VariableType(e.to_definite()),
            Types::Null => definite::types::Types::Null,
        }
    }

    pub fn get_type(&self) -> String {
        match *self {
            Types::Integer(_) => "int".to_string(),
            Types::Float(_) => "float".to_string(),
            Types::Bool(_) => "bool".to_string(),
            Types::String(_) => "string".to_string(),
            Types::Char(_) => "char".to_string(),
            Types::Collective(_) => "collective".to_string(),
            Types::Reference(_) => "reference".to_string(),
            Types::Operator(_) => "operator".to_string(),
            Types::Array(_) => "array".to_string(),
            Types::Cloak(_) => "cloak".to_string(),
            Types::ArrowFunction(_) => "arrowFunction".to_string(),
            Types::FunctionCall(_) => "functionCall".to_string(),
            Types::ConstructedClass(_) => "classCall".to_string(),
            Types::VariableType(_) => "variable".to_string(),
            Types::Negative(_) => "negative".to_string(),
            Types::Void => "void".to_string(),
            Types::Null => "null".to_string(),
        }
    }

    pub fn is_type_complete(&self) -> bool {
        match &*self {
            Types::Integer(e) => e.complete,
            Types::Float(e) => e.complete,
            Types::Bool(_) => true,
            Types::String(data) => data.complete,
            Types::Char(data) => data.complete,
            Types::Collective(e) => e.complete,
            Types::Reference(data) => !data.on_dot && data.complete,

            Types::Operator(e) => {
                e.first_filled
                    && e.data.operator != operator_type::Operators::Null
                    && (e.second_is_not_null && e.data.second.is_type_complete())
            }
            Types::Array(data) => data.complete,
            Types::Cloak(data) => data.complete,
            Types::ArrowFunction(data) => data.complete,
            Types::FunctionCall(data) => data.complete,
            Types::ConstructedClass(_) => true,
            Types::VariableType(_) => true,
            Types::Negative(e) => e.value.is_type_complete(),
            Types::Void => false,
            Types::Null => true,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false,   //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(_) => false,

            Types::Operator(_) => false,
            Types::Array(_) => true,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match *self {
            Types::Integer(_) => true, //Always complete
            Types::Float(_) => false,  //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(_) => false,

            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => true,    //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(_) => false,

            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false,   //Always complete
            Types::Bool(_) => true,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(_) => false,

            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => false,
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false,   //Always complete
            Types::Bool(_) => false,
            Types::String(_) => true,
            Types::Char(_) => false,
            Types::Collective(_) => false,
            Types::Reference(_) => false,

            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ConstructedClass(_) => false,
            Types::VariableType(_) => true,
            Types::Negative(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn make_complete(&mut self) {
        match self {
            Types::Integer(e) => e.complete = true,
            Types::Float(e) => e.complete = true,
            Types::Bool(_) => (),
            Types::String(e) => e.complete = true,
            Types::Char(e) => e.complete = true,
            Types::Collective(_) => (),
            Types::Reference(_) => (),
            Types::Operator(_) => (),
            Types::Array(e) => e.complete = true,
            Types::Cloak(e) => e.complete = true,
            Types::ArrowFunction(e) => e.complete = true,
            Types::FunctionCall(_) => (),
            Types::ConstructedClass(_) => (),
            Types::VariableType(_) => (),
            Types::Negative(_) => (),
            Types::Void => (),
            Types::Null => (),
        };
    }

    /*
    pub fn build_type_from_definer(rtype: crate::syntax::definers::DefinerCollecting) -> Types {
        match rtype {
            crate::definers::DefinerCollecting::Array(e) => {
                Types::Array(array_type::ArrayTypeCollector {
                    value: array_type::ArrayType {
                        layer_size: e.len,
                        collective: (0..e.len).into_iter().map(|x| {
                            array_type::ArrayEntry {
                                value_complete: true,
                                value: Box::new(Types::build_type_from_definer(*e.rtype)),
                                ..Default::default()
                            }
                        })
                    },
                    ..Default::default()
                })
            },
            crate::definers::DefinerCollecting::GrowableArray(_) => todo!(),
            crate::definers::DefinerCollecting::Generic(e) => {
                match e {
                    "string" => {
                        string_type::StringTypeCollector::default()
                    },
                    "char" => {
                        char_type::CharType::default()
                    },
                    "float" => {
                        string_type::::default()
                    },
                    "int" => {
                        integer_type::IntegerTypeCollector::default()
                    }
                }
            },
            crate::definers::DefinerCollecting::Function(_) => todo!(),
            crate::definers::DefinerCollecting::Cloak(_) => todo!(),
            crate::definers::DefinerCollecting::Collective(_) => todo!(),
            crate::definers::DefinerCollecting::Nullable(_) => todo!(),
            crate::definers::DefinerCollecting::Dynamic => todo!(),
        }
    }
    */
}

impl Default for Types {
    fn default() -> Self {
        Types::Null
    }
}
