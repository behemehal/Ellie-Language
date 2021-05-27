use crate::parser::Collecting;
use crate::syntax::{definers, types};
use ellie_core::defs;
use serde::Serialize;

use alloc::string::String;
use alloc::vec::Vec;


#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct FunctionParameter {
    pub name: String,
    pub r#type: definers::DefinerCollecting,
}


#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct FunctionParameterCollector {
    pub data: FunctionParameter,
    pub named: bool,
    pub name_pos: defs::Cursor, //Function parameter name position fn test([parameterName] : String) ....
    pub colon_expected: bool,
    pub child_brace: i8,
    pub type_text: String,
    pub typed: bool,
    pub type_pos: defs::Cursor, //Function parameter type position fn test(parameterName : [String]) ....
}


#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Function {
    pub name: String,                                //Function Name string
    pub parameters: Vec<FunctionParameterCollector>, //Parameter vector
    pub return_type: types::Types,                   //Return type from enum
    pub inside_code: Vec<Collecting>,
}


#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct FunctionCollector {
    pub data: Function,
    pub initialized: bool,
    pub named: bool,            //Function named
    pub name_pos: defs::Cursor, //Name position fn [test] ......

    pub parameter_wrote: bool,                     //Parameter type complete
    pub parameter_bracket_start_pos: defs::Cursor, //Bracket start [(] )
    pub parameter_bracket_end_pos: defs::Cursor,   //Bracket end ( [)]

    pub return_type_text: String, //Collected return type text will be matched with syntax::types::Types
    pub return_typed: bool,       //Function return typed
    pub pointer_typed: bool,
    pub return_pointer_position: defs::Cursor, //Function's type pointer position fn test() [>] String {.....

    pub inside_object_start: bool, //When a { detected we make this variable true so we will be sure that the } is not the } we will close the function
    pub inside_object_count: i64, //When a { detected we make this variable true so we will be sure that the } is not the } we will close the function
    pub code_bracket_start: defs::Cursor, //Bracket start fn test() > String [{]
    pub code_bracket_end: defs::Cursor, //Bracket start fn test() > String { ... [}]
    pub inside_code_wrote: bool,

    pub inside_code_string: String,

    pub complete: bool, //Fill this when end bracket placed
}
