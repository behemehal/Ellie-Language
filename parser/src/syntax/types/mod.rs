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
pub mod number_type;
pub mod operator_type;
pub mod refference_type;
pub mod string_type;
pub mod variable_type;

use enum_as_inner::EnumAsInner;
use serde::Serialize;

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Serialize, EnumAsInner)]
pub enum Types {
    Number(number_type::NumberType),
    Bool(bool_type::BoolType),
    String(string_type::StringType),
    Char(char_type::CharType),
    Collective, //Todo
    Refference(refference_type::RefferenceType),
    Operator(operator_type::OperatorType),
    Cloak(cloak_type::CloakType),
    Array(array_type::ArrayType),
    ArrowFunction(arrow_function::ArrowFunctionCollector),
    FunctionCall(function_call::FunctionCall),
    Void,
    VariableType(variable_type::VariableType),
    Null,
}

impl Types {
    #[no_mangle]
    pub extern "C" fn is_string_open(&self) -> bool {
        match &*self {
            Types::Number(_) => true,
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
            Types::VariableType(_) => true,
            Types::Void => true,
            Types::Null => true,
        }
    }

    #[no_mangle]
    pub extern "C" fn is_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => true, //Always complete
            Types::Bool(_) => true,   //Always complete
            Types::String(data) => data.complete,
            Types::Char(data) => data.complete,
            Types::Collective => false,
            Types::Refference(data) => !data.on_dot,
            Types::Operator(e) => {
                e.first_filled
                    && e.operator != operator_type::Operators::Null
                    && (e.second_is_not_null && e.second.is_complete())
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

    #[no_mangle]
    pub extern "C" fn is_array(&self) -> bool {
        match *self {
            Types::Number(_) => false,
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
            Types::VariableType(_) => true,
            Types::Void => false,
            Types::Null => false,
        }
    }

    #[no_mangle]
    pub extern "C" fn is_string(&self) -> bool {
        match *self {
            Types::Number(_) => false,
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

    #[no_mangle]
    pub extern "C" fn is_string_non_complete(&self) -> bool {
        //TODO Char is might be buggy
        match &*self {
            Types::Number(_) => false,
            Types::Bool(_) => false,
            Types::String(data) => !data.complete,
            Types::Char(data) => !data.complete,
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

    #[no_mangle]
    pub extern "C" fn is_array_non_complete(&self) -> Option<bool> {
        match &*self {
            Types::Number(_) => None,
            Types::Bool(_) => None,
            Types::String(_) => None,
            Types::Char(_) => None,
            Types::Collective => None,
            Types::Refference(_) => None,
            Types::Operator(_) => None,
            Types::Array(a) => Some(!a.complete),
            Types::Cloak(_) => None,
            Types::ArrowFunction(_) => None,
            Types::FunctionCall(_) => None,
            Types::VariableType(_) => None,
            Types::Void => None,
            Types::Null => None,
        }
    }

    #[no_mangle]
    pub extern "C" fn is_array_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Char(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(a) => a.complete,
            Types::Cloak(_) => false,
            Types::ArrowFunction(_) => false,
            Types::FunctionCall(_) => false,
            Types::VariableType(_) => true,
            Types::Void => false,
            Types::Null => false,
        }
    }

    #[no_mangle]
    pub extern "C" fn make_complete(&mut self) {
        match self {
            Types::Number(e) => e.complete = true,
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
