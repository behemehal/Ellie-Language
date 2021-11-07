use crate::alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::clone::Clone;
use serde::{Deserialize, Serialize};
pub mod errorList;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: u8,
    pub path: String,
    pub message: String,
    pub title: String,
    pub builded_message: BuildedError,
    pub debug_message: String,
    pub pos: crate::defs::Cursor,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBuildField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
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
    pub fn build(
        self,
        fields: Vec<ErrorBuildField>,
        debug_message: String,
        pos: crate::defs::Cursor,
    ) -> Error {
        let mut error = self.clone();
        let mut builded_message = self.message.to_string();
        let mut used_fields = Vec::new();
        for field in fields.clone() {
            let key: String = '$'.to_string() + &field.key.to_owned();
            if let Some(pos) = builded_message.find(&key) {
                used_fields.push(ErrorBuildField {
                    key: field.key.clone(),
                    value: field.value.clone(),
                });
                builded_message.replace_range(pos..(pos + key.len()), &field.value)
            } else {
                panic!(
                    "Failed to parse error {}, {:#?}",
                    self.message,
                    fields.clone()
                );
            }
        }

        error.debug_message = debug_message;
        error.pos = pos;
        error.builded_message = BuildedError {
            builded: builded_message,
            fields: used_fields,
        };
        error
    }

    pub fn build_without_debug(
        &mut self,
        fields: Vec<ErrorBuildField>,
        pos: crate::defs::Cursor,
    ) -> Error {
        let mut builded_message = self.message.to_string();
        let mut used_fields = Vec::new();
        for field in fields.clone() {
            let key: String = '$'.to_string() + &field.key.to_owned();
            if let Some(pos) = builded_message.find(&key) {
                used_fields.push(ErrorBuildField {
                    key: field.key.clone(),
                    value: field.value.clone(),
                });
                builded_message.replace_range(pos..(pos + key.len()), &field.value)
            } else {
                panic!(
                    "Failed to parse error {}, {:#?}",
                    self.message,
                    fields.clone()
                );
            }
        }

        self.pos = pos;
        self.builded_message = BuildedError {
            builded: builded_message,
            fields: used_fields,
        };

        self.clone()
    }
}

impl Default for Error {
    fn default() -> Error {
        Error {
            path: "".to_owned(),
            debug_message: "".to_owned(),
            code: 0x00,
            title: "".to_owned(),
            message: "".to_owned(),
            builded_message: BuildedError::default(),
            pos: crate::defs::Cursor {
                range_start: crate::defs::CursorPosition(0, 0),
                range_end: crate::defs::CursorPosition(0, 0),
            },
        }
    }
}
