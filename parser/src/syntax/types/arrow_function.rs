use serde::Serialize;

//use ellie_core::{defs};
//use crate::syntax::function::Function;
//use alloc::string::String;



#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ArrowFunction {
    
    pub complete: bool,
}