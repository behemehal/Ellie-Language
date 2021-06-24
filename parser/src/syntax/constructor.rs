use crate::parser::Collecting;
use ellie_core::defs;
use serde::Serialize;

use alloc::string::String;
use alloc::vec::Vec;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ConstructorParameter {
    pub name: String,
    pub pos: defs::Cursor
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Constructor {
    pub name: String, //Function Name string
    pub parameters: Vec<ConstructorParameter>, //Parameter vector
    pub inside_code: Vec<Collecting>,
    pub name_pos: defs::Cursor,           //Name position fn [test] ......
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ConstructorCollector {
    pub data: Constructor,
    pub initialized: bool,
    pub named: bool,                               //Function named
    pub parameter_wrote: bool,  
    pub parameter_brace_open: bool,                   //Parameter type complete    //Function return typed
    pub code_brace_open: bool,
    pub brace_count: usize, //When a { detected we make this variable true so we will be sure that the } is not the } we will close the function
    pub collecting_code: bool,
    pub inside_code_wrote: bool,
    pub inside_code_string: String,
    pub complete: bool, //Fill this when end bracket placed
}
