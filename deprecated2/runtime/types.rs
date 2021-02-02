use crate::collectors;

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Function {
    pub Dynamic: bool,
    pub Internal: bool,
    pub compiled_items: collectors::CompiledItems,
    pub parent: String
}

pub enum FunctionReturn {
    Return(Return),
    None
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Return {
    pub type_of: String,
    pub value: collectors::value_collector::ValueTypes
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Variable {
    pub data: collectors::variable::Variable,
    pub muteable : bool,
    pub parent: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Caller {
    pub data: collectors::Callers,
    pub parent: String
}

//#[derive(Debug, Clone , PartialEq, Eq)]
//pub struct Function {
//    code: 
//}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum ReturnedType {
    Null(bool),
    FunctionReturn(bool),
    SyntaxError
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct RuntimeReturn {
    pub has_error: bool,
    pub syntax_errors: Vec<collectors::SyntaxError>,
    pub returned: ReturnedType
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct DynamicCaller {
    pub pos: collectors::PositionOfElement,
    pub parameters: Vec<collectors::variable::Variable> //runtime_variable::Variable
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct DynamicFunction {
    pub name: String,
    pub parameters: Vec<collectors::function::Parameter>,
    pub return_type: String,
    pub global: bool,
    pub caller: fn(DynamicCaller) -> DynamicResponse
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum DynamicResponse {
    Void,
    Number(i32),
    Bool(bool),
    String(String),
    Collective(Vec<collectors::value_collector::Collective>)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct RuntimeOptions {
    pub allow_functions: bool,
    pub allow_variables: bool,
    pub allow_loop: bool,
    pub global_variables: Vec<collectors::variable::Variable>,
    pub global_functions: Vec<collectors::variable::Variable>
}