use crate::syntax::types;
use crate::mapper::defs;
pub mod helpers;

pub struct ReplaceChar {
    pub position: defs::CursorPosition,
    pub situation: u8
}

pub struct ReplaceRange {
    pub range: defs::Cursor,
    pub situation: u8
}

#[derive(Debug, Clone, PartialEq)]
pub enum SituationType {
    Empty,
    Info,
    Warning,
    Error,
    Improvement,
    Deprecated,
    Nightly,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Situations {
    Empty,
    RemoveChar,
    RemoveRange,
    ReplaceRange(String),
    ReplaceChar(String),
    ChangeVariableType(types::Types),
    ChangeVariableConstant,
    ChangeFunctionReturnType,
    RemoveFunctionReturnType
}

#[derive(Debug, Clone, PartialEq)]
pub struct Situation {
    pub identifier: Situations,
    pub action_type: SituationType,
    pub message: String,
}

impl Default for Situation {
    fn default() -> Situation {
        Situation {
            identifier: Situations::Empty,
            action_type: SituationType::Empty,
            message: String::from("")
        }
    }
}