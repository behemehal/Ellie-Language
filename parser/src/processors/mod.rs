use crate::parser::Parser;
pub mod class;
pub mod constructor;
pub mod definer_processor;
pub mod file_key;
pub mod function;
pub mod import;
pub mod type_processor;
pub mod variable;

pub trait Processor {
    fn process(self, parser: &mut Parser, page_id: u64);
}
