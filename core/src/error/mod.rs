use alloc::string::{String, ToString};
use alloc::vec::Vec;
use serde::Serialize;
use core::clone::Clone;
pub mod errorList;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Error {
    pub code: u8,
    pub scope: String,
    pub message: String,
    pub title: String,
    pub builded_message: BuildedError,
    pub debug_message: String,
    pub pos: crate::defs::Cursor,
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct ErrorBuildField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize)]
pub struct BuildedError {
    pub builded: String,
    pub fields: Vec<ErrorBuildField>,
}

impl BuildedError {
    pub fn build_from_string(message: String) -> BuildedError {
        BuildedError {
            builded: message,
            ..Default::default()
        }
    }
}

impl Error {
    pub fn build(body: String, fields: Vec<ErrorBuildField>) -> BuildedError {
        let mut builded_message = body.to_string();
        let mut used_fields = Vec::new();
        for field in fields.clone() {
            let key: String = '$'.to_string() + &field.key.to_string();
            if let Some(pos) = builded_message.find(&key) {
                used_fields.push(ErrorBuildField {
                    key: field.key.clone(),
                    value: field.value.clone()
                });
                builded_message.replace_range(pos..(pos + key.len()), &field.value)
            } else {
                panic!("Failed to parse error {}, {:#?}", body, fields.clone());
            }
        }
        BuildedError {
            builded: builded_message,
            fields: used_fields,
        }
    }
}

impl Default for Error {
    fn default() -> Error {
        Error {
            debug_message: "".to_string(),
            code: 0x00,
            scope: "".to_string(),
            title: "".to_string(),
            message: "".to_string(),
            builded_message: BuildedError::default(),
            pos: crate::defs::Cursor {
                range_start: crate::defs::CursorPosition(0, 0),
                range_end: crate::defs::CursorPosition(0, 0),
            },
        }
    }
}
