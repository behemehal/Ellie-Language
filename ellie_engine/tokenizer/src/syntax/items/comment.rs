use ellie_core::defs;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
    pub complete: bool,
    pub pos: defs::Cursor,
    pub content: Vec<String>,
    pub first_char_collected: bool,
    pub type_collected: bool,
    pub line_comment: bool,
}
