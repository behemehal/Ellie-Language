#[derive(Debug)]
pub struct VariableLineSpecifier {
    pub v_keyword: i32,
    pub name: [i32; 2],
    pub colon: i32,
    pub equal_mark: i32,
    pub value: [i32; 2]
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Line {
    pub line: usize,
    pub colmn: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct SyntaxError {
    pub pos: Line,
    pub message: String,
    pub line: String,
    pub file: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct NumberType {
    pub name: String,
    pub value: String,
    pub line: i32
}
#[derive(Debug, Clone , PartialEq, Eq)]
pub struct StringType {
    pub name: String,
    pub value: String,
    pub line: i32
}
#[derive(Debug, Clone , PartialEq, Eq)]
pub struct BoolType   {
    pub name: String,
    pub value: String,
    pub line: i32
}
#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ArrayType  {
    pub name: String,
    pub value: String,
    pub line: i32
}
#[derive(Debug, Clone , PartialEq, Eq)]
pub enum Typos {
    NumberVariable(NumberType),
    StringVariable(StringType),
    BoolVariable(BoolType),
    #[allow(dead_code)]
    ArrayVariable(ArrayType)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Variable {
    pub variable_type: String,
    pub value: Typos,
    pub line: Line
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Parameter {
    pub name: String,
    pub typ: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub parameters: Vec<Parameter>,
    pub code: Compiled,
    pub line: Line
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct CalledParameter {
    pub name: String,
    pub value: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Caller {
    pub name: String,
    pub called_paramaters: Vec<CalledParameter>,
    pub line: Line
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Condition {
    pub condition_type: String,
    pub given_conditions: String,
    pub code: String,//Compiled, 
    pub start_line: i32,
    pub end_line: i32
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Conditions {
    pub chains: Vec<Condition>,
    pub start_line: i32
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Compiled {
    pub variables: Vec<Variable>,
    pub functions: Vec<Function>,
    pub conditions: Vec<Conditions>,
    pub callers: Vec<Caller>,
    pub errors: Vec<SyntaxError>
}

#[derive(Debug)]
pub struct StartedEnded {
    pub started: bool,
    pub ended: bool
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct VariableStarted {
    pub started: bool,
    pub name: bool,
    pub typo: bool,
    pub typo_started: bool,
    pub value: bool,
    pub value_started: bool,
    pub name_text: String,
    pub type_text: String,
    pub value_text: String,
    pub qual_exist: bool,
    pub ignore_qual: bool,
    pub string_started: bool,
    pub string_closed: bool,
    pub line_started: i32
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct FunctionStarted {
    pub started: bool,
    pub name: bool,
    pub code: String,
    pub name_text: String,
    pub parameter_started: bool,
    pub paramater_text: String,
    pub paramater_type_text: String,
    pub has_paramater_type: bool,
    pub has_paramater_text: bool,
    pub parameters: Vec<Parameter>,
    pub bracket_started: bool,
    pub bracket_collect: bool,
    pub bracked_ended: bool,
    pub return_started: bool,
    pub return_text: String,
    pub ignore_qual: bool,
    pub ignore_brace: bool,
    pub line_started: i32
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ConditionStarted {
    pub started: bool,
    pub cond_type: String,
    pub await_if: bool,
    pub collecting_inner_code: bool,
    pub collect_if: String,
    pub condition_started: bool,
    pub condition_brace_started: bool,
    pub condition_brace: String,
    pub inner_code: String,
    pub chains: Vec<Condition>,
    pub ignore_qual: bool,
    pub ignore_brace: bool,
    pub line_started: i32,
    pub if_started: i32
}

pub fn get_variable(variable_data: VariableStarted) -> Typos {
    if variable_data.type_text == "number" {
        return Typos::NumberVariable(NumberType {
            name: variable_data.name_text,
            value: variable_data.value_text,
            line: variable_data.line_started
        })
    } else if variable_data.type_text == "string" {
        return Typos::StringVariable(StringType {
            name: variable_data.name_text,
            value: variable_data.value_text,
            line: variable_data.line_started
        })
    } else if variable_data.type_text == "bool" {
        return Typos::BoolVariable(BoolType {
            name: variable_data.name_text,
            value: variable_data.value_text,
            line: variable_data.line_started
        })
    } else if variable_data.type_text == "array" {
        return Typos::ArrayVariable(ArrayType {
            name: variable_data.name_text,
            value: variable_data.value_text,
            line: variable_data.line_started
        })
    } else {
        return Typos::StringVariable(StringType {
            name: variable_data.name_text,
            value: variable_data.value_text,
            line: variable_data.line_started
        })
    }
}