use crate::parser::Parser;
pub mod type_processor;
pub mod definer_processor;
pub mod variable;
pub mod class;
pub mod import;

pub trait Processor {
    fn process(
        self,
        parser: &mut Parser,
        page_id: u64,
    );
}
