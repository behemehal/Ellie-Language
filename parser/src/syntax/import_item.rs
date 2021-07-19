pub use crate::parser;
use alloc::boxed::Box;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Serialize)]
pub struct ImportItem {
    pub item: Box<parser::Collecting>,
    pub public: bool,
}
