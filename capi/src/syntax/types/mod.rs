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


#[repr(C)]
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