pub mod array_type;
pub mod bool_type;
pub mod comparison_type;
pub mod double_type;
pub mod function_call;
pub mod logical_type;
pub mod number_type;
pub mod refference_type;
pub mod string_type;
pub mod operator_type;

use serde::Serialize;
/*
#[derive(PartialEq, Default, Debug, Clone)]
pub struct CollectiveEntry {
    pub key: String,
    pub dynamic: bool,
    pub key_named: bool,
    pub r#type: String,
    pub typed: bool,
    pub value_complete: bool,
    pub raw_value: String,
    pub value: Box<Types>,
}

#[derive(PartialEq, Default, Debug, Clone)]
pub struct CollectiveType {
    pub layer_size: usize,
    pub collective: Vec<CollectiveEntry>,
}
*/

pub enum __ArithmeticOperator {
    Addition,
    Subtraction,
    Multiplication,
    Exponentiation,
    Division,
    Modulus,
    Increment,
    Decrement,
}

#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Types {
    Number(number_type::NumberType),
    Double(double_type::DoubleType),
    Bool(bool_type::BoolType),
    String(string_type::StringType),
    Collective, //DEPRECATED
    Refference(refference_type::RefferenceType),
    Operator(operator_type::OperatorType),
    Array(array_type::ArrayType),
    Function,
    FunctionCall(function_call::FunctionCall),
    Void,
    Null,
}

impl Types {
    pub fn is_string_open(&self) -> bool {
        match &*self {
            Types::Number(_) => true,
            Types::Double(_) => true,
            Types::Bool(_) => true,
            Types::String(data) => !data.complete,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Collective => false,
            Types::Array(data) => {
                if !data.complete {
                    if data.collective.len() == 0 {
                        false
                    } else {
                        !data.collective[data.collective.len() - 1].value_complete
                    }
                } else {
                    false
                }
            }
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => true,
            Types::Null => true,
        }
    }

    pub fn is_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => true, //Always complete
            Types::Double(_) => true, //Always complete
            Types::Bool(_) => true,   //Always complete
            Types::String(data) => data.complete,
            Types::Collective => false,
            Types::Refference(data) => !data.on_dot,
            Types::Operator(_) => false,
            Types::Array(data) => data.complete,
            Types::Function => false,
            Types::FunctionCall(data) => data.complete,
            Types::Void => true,
            Types::Null => true,
        }
    }

    pub fn is_array(&self) -> bool {
        match *self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => true,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }
    pub fn is_string(&self) -> bool {
        match *self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => true,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_string_non_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(data) => !data.complete,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(_) => false,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn is_array_non_complete(&self) -> Option<bool> {
        match &*self {
            Types::Number(_) => None,
            Types::Double(_) => None,
            Types::Bool(_) => None,
            Types::String(_) => None,
            Types::Collective => None,
            Types::Refference(_) => None,
            Types::Operator(_) => None,
            Types::Array(a) => Some(!a.complete),
            Types::Function => None,
            Types::FunctionCall(_) => None,
            Types::Void => None,
            Types::Null => None,
        }
    }

    pub fn is_array_complete(&self) -> bool {
        match &*self {
            Types::Number(_) => false,
            Types::Double(_) => false,
            Types::Bool(_) => false,
            Types::String(_) => false,
            Types::Collective => false,
            Types::Refference(_) => false,
            Types::Operator(_) => false,
            Types::Array(a) => a.complete,
            Types::Function => false,
            Types::FunctionCall(_) => false,
            Types::Void => false,
            Types::Null => false,
        }
    }

    pub fn make_complete(&mut self) {
        match self {
            Types::Number(e) => e.complete = true,
            Types::Double(e) => e.complete = true,
            Types::Bool(_) => (),
            Types::String(e) => e.complete = true,
            Types::Collective => (),
            Types::Refference(_) => (),
            Types::Operator(_) => (),
            Types::Array(e) => e.complete = true,
            Types::Function => (),
            Types::FunctionCall(_) => (),
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
