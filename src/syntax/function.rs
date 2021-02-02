use crate::mapper::{defs, Collecting};
use crate::syntax::types;

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionParameter {
    pub name: String,
    pub named: bool,
    pub name_pos: defs::Cursor, //Function parameter name position fn test([parameterName] : String) ....
    pub colon_expected: bool,
    pub r#type: types::Types,
    pub type_text: String,
    pub typed: bool,
    pub type_pos: defs::Cursor, //Function parameter type position fn test(parameterName : [String]) ....
}

impl Default for FunctionParameter {
    fn default() -> FunctionParameter {
        FunctionParameter {
            name: String::new(),
            named: false,
            name_pos: defs::Cursor::default(),
            colon_expected: false,
            r#type: types::Types::default(),
            type_text: String::new(),
            typed: false,
            type_pos: defs::Cursor::default()
        }
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct FunctionCollector {
    pub initialized: bool,
    pub name: String, //Function Name string
    pub named: bool, //Function named
    pub name_pos: defs::Cursor, //Name position fn [test] ......
    
    pub parameter_wrote: bool,  //Parameter type complete
    pub parameters: Vec<FunctionParameter>, //Parameter vector
    pub parameter_bracket_start_pos: defs::Cursor, //Bracket start [(] )
    pub parameter_bracket_end_pos: defs::Cursor, //Bracket end ( [)]

    pub return_type_text: String, //Collected return type text will be matched with syntax::types::Types
    pub return_type: types::Types, //Return type from enum
    pub return_typed: bool, //Function return typed
    pub pointer_typed: bool,
    pub return_pointer_position: defs::Cursor,//Function's type pointer position fn test() [>] String {.....

    pub inside_object_start: bool, //When a { detected we make this variable true so we will be sure that the } is not the } we will close the function 
    pub inside_object_count: i64, //When a { detected we make this variable true so we will be sure that the } is not the } we will close the function 
    pub code_bracket_start: defs::Cursor, //Bracket start fn test() > String [{]
    pub code_bracket_end: defs::Cursor, //Bracket start fn test() > String { ... [}]
    pub inside_code_wrote: bool, 
    pub inside_code: Vec<Collecting>,
    pub inside_code_string: String,

    pub complete: bool, //Fill this when end bracket placed
}

impl Default for FunctionCollector {
    fn default() -> FunctionCollector {
        FunctionCollector {
            initialized: false,
            name: String::new(),
            named: false,
            name_pos: defs::Cursor::default(),
            parameter_wrote: false,
            parameters: Vec::new(),
            parameter_bracket_start_pos: defs::Cursor::default(),
            parameter_bracket_end_pos: defs::Cursor::default(),
            return_type_text: String::new(),
            return_type: types::Types::default(),
            return_typed: false,
            pointer_typed: false,
            return_pointer_position: defs::Cursor::default(),
            inside_object_start: false,
            inside_object_count: 0,
            code_bracket_start: defs::Cursor::default(),
            code_bracket_end: defs::Cursor::default(),
            inside_code_wrote: false,
            inside_code_string: String::new(),
            inside_code: Vec::new(),
            complete: false
        }
    }
}