use ellie_parser::parser::Parsed;
use alloc::string::String;

pub struct Runtime {
    pub start_point: String,
}

impl Runtime {
    pub fn new(main: String) -> Runtime {
        Runtime { start_point: main }
    } 
}
