pub mod arithmetic_type;
pub mod array;
pub mod arrow_function;
pub mod assignment_type;
pub mod bool;
pub mod bracket_reference;
pub mod cloak;
pub mod collective;
pub mod comparison_type;
pub mod constructed_class;
pub mod ellie_char;
pub mod float;
pub mod function_call;
pub mod integer;
pub mod logical_type;
pub mod negative;
pub mod null_resolver;
pub mod operator;
pub mod reference;
pub mod string;
pub mod variable;

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
    BracketReference(bracket_reference::BracketReference),
    Operator(operator::OperatorType),
    Cloak(cloak::CloakType),
    Array(array::ArrayType),
    ArrowFunction(arrow_function::ArrowFunction),
    ConstructedClass(constructed_class::ConstructedClass),
    FunctionCall(function_call::FunctionCall),
    Void,
    NullResolver(null_resolver::NullResolver),
    Negative(negative::Negative),
    VariableType(variable::VariableType),
    Null,
}