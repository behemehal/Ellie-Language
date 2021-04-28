use crate::alloc::vec::Vec;
use crate::alloc::string::String;
use crate::syntax;
use ellie_core::{error, utils};


pub fn collect(
    type_data: &mut syntax::r#type::TypeConstructorCollector,
    errors: &mut Vec<error::Error>,
    letter_char: String,
    last_char: String,
) {
   
    type_data.data.name = type_data.collecting.clone();
}


/*
#[derive(PartialEq, Debug, Clone, Serialize)]
pub enum Child {
    Some(DirectType),
    None
}

impl Default for Child {
    fn default() -> Self {
        Child::None
    }
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct DirectType {
    pub name: String,
    pub child: Box<Option<Child>>,
}

#[derive(PartialEq, Debug, Clone, Serialize, Default)]
pub struct TypeConstructorCollector {
    pub data: DirectType,
    pub collecting: String,
    pub child_started: bool,
}
*/