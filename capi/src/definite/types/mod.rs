pub mod arithmetic_type;
pub mod array;
pub mod arrow_function;
pub mod bool;
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
use ellie_core::definite;

#[repr(C)]
pub enum Types {
    Integer(integer::IntegerType),
    Float(float::FloatType),
    Bool(bool::BoolType),
    Char(ellie_char::CharType),
    String(string::StringType),
    Collective(collective::Collective),
    Reference(reference::ReferenceType),
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

pub unsafe fn build_collecting_from(target: definite::types::Types) -> Types {
    match target {
        definite::types::Types::Integer(e) => Types::Integer(integer::build_integer_from(e)),
        definite::types::Types::Float(e) => Types::Float(float::build_float_from(e)),
        definite::types::Types::Bool(e) => Types::Bool(bool::build_bool_from(e)),
        definite::types::Types::String(e) => Types::String(string::build_string_from(e)),
        definite::types::Types::Char(e) => Types::Char(ellie_char::build_char_from(e)),
        definite::types::Types::Collective(e) => {
            Types::Collective(collective::build_collective_from(e))
        }
        definite::types::Types::Reference(e) => {
            Types::Reference(reference::build_reference_from(e))
        }
        definite::types::Types::Operator(e) => Types::Operator(operator::build_operator_from(e)),
        definite::types::Types::Cloak(e) => Types::Cloak(cloak::build_cloak_from(e)),
        definite::types::Types::Array(e) => Types::Array(array::build_array_from(e)),
        definite::types::Types::ArrowFunction(e) => {
            Types::ArrowFunction(arrow_function::build_arrow_function_from(e))
        }
        definite::types::Types::ConstructedClass(e) => {
            Types::ConstructedClass(constructed_class::build_constructed_class_from(e))
        }
        definite::types::Types::FunctionCall(e) => {
            Types::FunctionCall(function_call::build_function_call_from(e))
        }
        definite::types::Types::Void => Types::Void,
        definite::types::Types::NullResolver(e) => {
            Types::NullResolver(null_resolver::build_null_resolver_from(e))
        }
        definite::types::Types::Negative(e) => Types::Negative(negative::build_negative_from(e)),
        definite::types::Types::VariableType(e) => {
            Types::VariableType(variable::build_variable_type_from(e))
        }
        definite::types::Types::Null => Types::Null,
    }
}
