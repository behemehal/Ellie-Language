pub mod array;
pub mod as_keyword;
pub mod bool;
pub mod brace_reference;
pub mod class_call;
pub mod cloak;
pub mod collective;
pub mod ellie_char;
pub mod float;
pub mod function_call;
pub mod integer;
pub mod negative;
pub mod null_resolver;
pub mod operator;
pub mod reference;
pub mod string;
pub mod variable;
pub mod vector;
pub mod function;

use serde::{Deserialize, Serialize};

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Types {
    Integer(integer::IntegerType),
    Float(float::FloatType),
    Bool(bool::BoolType),
    String(string::StringType),
    Char(ellie_char::CharType),
    Collective(collective::CollectiveType),
    Reference(reference::ReferenceType),
    BraceReference(brace_reference::BraceReferenceType),
    Operator(operator::OperatorType),
    Cloak(cloak::CloakType),
    Array(array::ArrayType),
    Vector(vector::VectorType),
    Function(function::Function),
    ClassCall(class_call::ClassCall),
    FunctionCall(function_call::FunctionCall),
    Void,
    NullResolver(null_resolver::NullResolver),
    Negative(negative::Negative),
    VariableType(variable::VariableType),
    AsKeyword(as_keyword::AsKeyword),
    Null,
    Dynamic,
}

impl Default for Types {
    fn default() -> Self {
        Types::Null
    }
}
