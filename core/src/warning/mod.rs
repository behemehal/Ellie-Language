use alloc::{
    borrow::ToOwned,
    string::{String, ToString},
    vec::Vec,
};
use core::clone::Clone;
use serde::{Deserialize, Serialize};
#[doc(hidden)]
#[cfg(feature = "compiler_utils")]
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

/// A parser warning
/// ## Fields
/// * `code` - Warning code
/// * `path` - Path of warning
/// * `message` - Warning message
/// * `title` - Warning title
/// * `builded_message` - [`BuildedWarning`]
/// * `pos` - Error position in code by [`crate::defs::Cursor`]
/// * `reference_message` - If warning choses reference a code point this is the message, `reference_block` should be [`Some`] for this to be rendered
/// * `reference_block` - [`Option`] acquires a tuple of [`crate::defs::Cursor`] and [`String`] which is the path of referenced file
/// * `semi_assist` - Boolean value that indicates if the error is semi-assistive
/// * `full_assist` - Boolean value that indicates if the error is full-assistive
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Warning {
    pub code: u8,
    pub path: String,
    pub message: String,
    pub title: String,
    pub builded_message: BuildedWarning,
    pub pos: crate::defs::Cursor,
    pub reference_message: String,
    pub reference_block: Option<(crate::defs::Cursor, String)>,
    pub semi_assist: bool,
    pub full_assist: bool,
}

impl Warning {
    /// Create a new warning
    /// ## Arguments
    /// * `fields` - [`Vec`] of [`WarningBuildField`]
    /// * `path` - [`String`] path of warning
    /// * `pos` - Warning position in code by [`crate::defs::Cursor`]
    /// ## Returns
    /// [`Warning`]
    /// ## Example
    /// ```
    /// use ellie_core::warning::{Warning, WarningBuildField};
    /// let warning = warning::Warning {
    ///    code: 0x00,
    ///    title: "Title".to_owned(),
    ///    message: "Found '$current', expected mooo".to_owned(),
    ///    semi_assist: true,
    ///    ..Default::default()
    ///};
    /// let builded = warning.build(vec![
    ///    WarningBuildField {
    ///       key: String::from("current"),
    ///       value: String::from("Milk"),
    ///   }
    /// ],String::new("path/to/file"), Cursor::default());
    /// ```
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
