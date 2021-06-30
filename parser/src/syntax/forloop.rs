use crate::syntax::types::cloak_type;
use alloc::string::String;
use ellie_core::defs;
use serde::Serialize;

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct Forloop {
    pub parameters: cloak_type::CloakType,
    pub parameters_pos: defs::Cursor,
    pub code: String,
}

#[derive(PartialEq, Debug, Clone, Default, Serialize)]
pub struct ForloopCollector {
    pub parameters_collected: bool,
    pub brace_count: usize,
    pub has_code: bool,
}
