pub mod variable;
pub mod string;
pub mod bool;
pub mod float;
pub mod integer;
pub mod ellie_char;
pub mod negative;
pub mod function_call;
pub mod collective;
pub mod reference;
pub mod operator;
pub mod arithmetic_type;
pub mod comparison_type;
pub mod logical_type;
pub mod cloak;
pub mod array;
pub mod arrow_function;
pub mod constructed_class;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Types {
    Integer(integer::IntegerType),
    Float(float::FloatType),
    Bool(bool::BoolType),
    String(string::StringType),
    Char(ellie_char::CharType),
    Collective(collective::Collective),
    Reference(reference::ReferenceType),
    Operator(operator::OperatorType),
    Cloak(cloak::CloakType),
    Array(array::ArrayType),
    ArrowFunction(arrow_function::ArrowFunction),
    ConstructedClass(constructed_class::ConstructedClass),
    FunctionCall(function_call::FunctionCall),
    Void,
    Negative(negative::Negative),
    VariableType(variable::VariableType),
    Null,
}