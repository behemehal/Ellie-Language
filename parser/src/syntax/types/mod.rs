pub mod arithmetic_type;
pub mod array_type;
pub mod arrow_function;
pub mod bool_type;
pub mod char_type;
pub mod class_call;
pub mod cloak_type;
pub mod comparison_type;
pub mod double_type;
pub mod float_type;
pub mod function_call;
pub mod integer_type;
pub mod logical_type;
pub mod operator_type;
pub mod refference_type;
pub mod string_type;
pub mod variable_type;
pub mod collective_type;

use alloc::format;
use alloc::string::String;
use ellie_core::utils;
use enum_as_inner::EnumAsInner;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner)]
pub enum Types {
    Integer(integer_type::IntegerType),
    Float(float_type::FloatTypeCollector),
    Bool(bool_type::BoolType),
    String(string_type::StringType),
    Char(char_type::CharType),
    Collective(collective_type::CollectiveCollector), //Todo
    Refference(refference_type::RefferenceType),
    Operator(operator_type::OperatorTypeCollector),
    Cloak(cloak_type::CloakType),
    Array(array_type::ArrayType),
    ArrowFunction(arrow_function::ArrowFunctionCollector),
    ClassCall(class_call::ClassCallCollector),
    FunctionCall(function_call::FunctionCallCollector),
    Void,
    VariableType(variable_type::VariableType),
    Null,
}

impl Types {
    pub fn get_type(&self) -> String {
        let mut q: String = format!("{:?}", self);
        let bracket_offset = q.find('(').unwrap_or_else(|| q.len());
        q.replace_range(bracket_offset.., "");
        utils::lower_first_char(q)
    }

    pub fn is_type_complete(&self) -> bool {
        match &*self {
            Types::Integer(e) => e.complete,
            Types::Float(e) => e.complete,
            Types::Bool(_) => true,
            Types::String(data) => data.complete,
            Types::Char(data) => data.complete,
            Types::Collective(e) => e.complete,
            Types::Refference(data) => !data.on_dot,
            Types::Operator(e) => {
                e.first_filled
                    && e.data.operator != operator_type::Operators::Null
                    && (e.second_is_not_null && e.data.second.is_type_complete())
            }
            Types::Array(data) => data.complete,
            Types::Cloak(data) => data.complete,
            Types::ArrowFunction(data) => data.complete,
            Types::FunctionCall(data) => data.complete,
            Types::ClassCall(_) => true,
            Types::VariableType(_) => true,
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
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => true,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ClassCall(_) => false,
            Types::VariableType(_) => false,
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
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ClassCall(_) => false,
            Types::VariableType(_) => false,
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
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ClassCall(_) => false,
            Types::VariableType(_) => false,
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
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ClassCall(_) => false,
            Types::VariableType(_) => false,
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
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::ClassCall(_) => false,
            Types::VariableType(_) => true,
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
            Types::Refference(_) => (),
            Types::Operator(_) => (),
            Types::Array(e) => e.complete = true,
            Types::Cloak(e) => e.complete = true,
            Types::ArrowFunction(e) => e.complete = true,
            Types::FunctionCall(_) => (),
            Types::ClassCall(_) => (),
            Types::VariableType(_) => (),
            Types::Void => (),
            Types::Null => (),
        };
    }
}

impl Default for Types {
    fn default() -> Self {
        Types::Null
    }
}
