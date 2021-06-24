pub mod arithmetic_type;
pub mod array_type;
pub mod arrow_function;
pub mod bool_type;
pub mod char_type;
pub mod cloak_type;
pub mod comparison_type;
pub mod double_type;
pub mod function_call;
pub mod logical_type;
pub mod operator_type;
pub mod refference_type;
pub mod string_type;
pub mod variable_type;
pub mod integer_type;
pub mod float_type;

use enum_as_inner::EnumAsInner;
use serde::Serialize;
use alloc::string::String;
use alloc::format;
use ellie_core::utils;

#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner)]
pub enum Types {
    Integer(integer_type::IntegerType),
    Float(float_type::FloatTypeCollector),
    Bool(bool_type::BoolType),
    String(string_type::StringType),
    Char(char_type::CharType),
    Collective, //Todo
    Refference(refference_type::RefferenceType),
    Operator(operator_type::OperatorTypeCollector),
    Cloak(cloak_type::CloakType),
    Array(array_type::ArrayType),
    ArrowFunction(arrow_function::ArrowFunctionCollector),
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

    pub fn is_string_open(&self) -> bool {
        match &*self {
            Types::Integer(_) => true, //Always complete
            Types::Float(_) => true, //Always complete
            Types::Bool(_) => true,
            Types::String(data) => !data.complete,
            Types::Char(data) => !data.complete,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Collective => false,
            Types::Array(data) => {
                if !data.complete {
                    if data.collective.is_empty() {
                        false
                    } else {
                        !data.collective[data.collective.len() - 1].value_complete
                    }
                } else {
                    false
                }
            }
            Types::Cloak(data) => {
                if !data.complete {
                    if data.collective.is_empty() {
                        false
                    } else {
                        !data.collective[data.collective.len() - 1].value_complete
                    }
                } else {
                    false
                }
            }
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::VariableType(_) => false,
            Types::Void => true,
            Types::Null => true,
        }
    }

    pub fn is_type_complete(&self) -> bool {
        match &*self {
            Types::Integer(e) => e.complete,
            Types::Float(e) => e.complete,
            Types::Bool(_) => true,
            Types::String(data) => data.complete,
            Types::Char(data) => data.complete,
            Types::Collective => false,
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
            Types::VariableType(_) => true,
            Types::Void => false,
            Types::Null => true,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false, //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => true,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::VariableType(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_integer(&self) -> bool {
        match *self {
            Types::Integer(_) => true, //Always complete
            Types::Float(_) => false, //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::VariableType(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_float(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => true, //Always complete
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::VariableType(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_bool(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false, //Always complete
            Types::Bool(_) => true,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::VariableType(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_string(&self) -> bool {
        match *self {
            Types::Integer(_) => false, //Always complete
            Types::Float(_) => false, //Always complete
            Types::Bool(_) => false,
            Types::String(_) => true,
            Types::Char(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
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
            Types::Collective => (),
            Types::Refference(_) => (),
            Types::Operator(_) => (),
            Types::Array(e) => e.complete = true,
            Types::Cloak(e) => e.complete = true,
            Types::ArrowFunction(e) => e.complete = true,
            Types::FunctionCall(_) => (),
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
