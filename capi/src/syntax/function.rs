use crate::defs;
use crate::parser::Collecting;
use crate::syntax::{definers, types};
use libc::c_char;

#[repr(C)]
pub struct FunctionParameter {
    pub name: *const c_char,
    pub rtype: definers::DefinerCollecting,
}

#[repr(C)]
pub struct FunctionParameterCollector {
    pub data: FunctionParameter,
    pub named: bool,
    pub name_pos: defs::Cursor, //Function parameter name position fn test([parameterName] : String) ....
    pub colon_expected: bool,
    pub child_brace: i8,
    pub type_text: *const c_char,
    pub typed: bool,
    pub type_pos: defs::Cursor, //Function parameter type position fn test(parameterName : [String]) ....
}

#[repr(C)]
pub struct Function {
    pub name: *const c_char,
    pub parameters: *const FunctionParameterCollector,
    pub return_type: types::Types,
    pub inside_code: *const Collecting,
}

#[repr(C)]
pub struct FunctionCollector {
    pub data: Function,
    pub initialized: bool,
    pub named: bool,
    pub name_pos: defs::Cursor,

    pub parameter_wrote: bool,
    pub parameter_bracket_start_pos: defs::Cursor,
    pub parameter_bracket_end_pos: defs::Cursor,

    pub return_type_text: *const c_char,
    pub return_typed: bool,
    pub pointer_typed: bool,
    pub return_pointer_position: defs::Cursor,

    pub inside_object_start: bool,
    pub inside_object_count: i64,
    pub code_bracket_start: defs::Cursor,
    pub code_bracket_end: defs::Cursor,
    pub inside_code_wrote: bool,

    pub inside_code_string: *const c_char,

    pub complete: bool,
}
