use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use core::clone::Clone;
use serde::{Deserialize, Serialize};
pub mod warning_list;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WarningBuildField {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BuildedWarning {
    pub builded: String,
    pub fields: Vec<WarningBuildField>,
}

impl BuildedWarning {
    pub fn build_from_string(message: String) -> BuildedWarning {
        BuildedWarning {
            builded: message,
            ..Default::default()
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]

pub struct Warning {
    pub code: u8,
    pub path: String,
    pub message: String,
    pub title: String,
    pub reference_message: String,
    pub builded_message: BuildedWarning,
    pub reference_block: Option<(crate::defs::Cursor, String)>,
    pub pos: crate::defs::Cursor,
    pub semi_assist: bool,
    pub full_assist: bool,
}

impl Warning {
    pub fn build(
        self,
        fields: Vec<WarningBuildField>,
        path: String,
        pos: crate::defs::Cursor,
    ) -> Warning {
        let mut warning = self.clone();
        let mut builded_message = self.message.to_string();
        let mut used_fields = Vec::new();
        for field in fields.clone() {
            let key: String = '$'.to_string() + &field.key.to_owned();
            if let Some(pos) = builded_message.find(&key) {
                used_fields.push(WarningBuildField {
                    key: field.key.clone(),
                    value: field.value.clone(),
                });
                builded_message.replace_range(pos..(pos + key.len()), &field.value)
            } else {
                panic!(
                    "Failed to parse warning {}, {:#?}",
                    self.message,
                    fields.clone()
                );
            }
        }

        warning.path = path;
        warning.pos = pos;
        warning.builded_message = BuildedWarning {
            builded: builded_message,
            fields: used_fields,
        };
        warning
    }
}

impl Default for Warning {
    fn default() -> Warning {
        Warning {
            path: "".to_owned(),
            code: 0x00,
            title: "".to_owned(),
            message: "".to_owned(),
            reference_message: "".to_owned(),
            builded_message: BuildedWarning::default(),
            pos: crate::defs::Cursor::default(),
            reference_block: None,
            semi_assist: false,
            full_assist: false,
        }
    }
}
