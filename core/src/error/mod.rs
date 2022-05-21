use crate::alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec::Vec;
use core::clone::Clone;
use serde::{Deserialize, Serialize};
#[doc(hidden)]
pub mod error_list;

/// `Ellie Error` struct
/// ## Fields
/// * `code` - Error code
/// * `path` - Error path
/// * `message` - Error message
/// * `title` - Error title
/// * `builded_message` - [`BuildedError`]
/// * `debug_message` - Development error message used to identify the error in the language not presented to the user
/// * `pos` - Error position in code by [`crate::defs::Cursor`]
/// * `reference_message` - If error choses to reference a code point this is the message, `reference_block` should be [`Some`] for this to be rendered
/// * `reference_block` - [`Option`] acquires a tuple of [`crate::defs::Cursor`] and [`String`] which is the path of referenced file
/// * `semi_assist` - Boolean value that indicates if the error is semi-assistive
/// * `full_assist` - Boolean value that indicates if the error is full-assistive
///
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Error {
    pub code: u8,
    pub path: String,
    pub message: String,
    pub title: String,
    pub builded_message: BuildedError,
    pub debug_message: String,
    pub pos: crate::defs::Cursor,
    pub reference_message: String,
    pub reference_block: Option<(crate::defs::Cursor, String)>,
    pub semi_assist: bool,
    pub full_assist: bool,
}

/// Instance of [`BuildedError`] represents $token in error message
/// In this case key is `token` and value is pending to be write
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ErrorBuildField {
    pub key: String,
    pub value: String,
}

impl ErrorBuildField {
    pub fn new(key: &str, value: &String) -> Self {
        Self {
            key: key.to_string(),
            value: value.to_string(),
        }
    }
}

///`BuildedError` is a struct for modular error messages
/// ## Fields
/// * `builded` - Error message
/// * `fields` - [`Vec`] of [`ErrorBuildField`]
/// ## Example:
/// ```
/// use ellie_core::error::{BuildedError, ErrorBuildField};
/// let mut builded = BuildedError {
///     builded: String::from("Duck does say $thing when gets angry"),
///     fields: vec![    
///         ErrorBuildField {
///             key: String::from("thing"),
///             value: String::from("Mooo"),
///         }
///     ],
/// };
/// ```
#[derive(Debug, Clone, PartialEq, Default, Serialize, Deserialize)]
pub struct BuildedError {
    pub builded: String,
    pub fields: Vec<ErrorBuildField>,
}

impl BuildedError {
    #[deprecated]
    pub fn build_from_string(message: String) -> BuildedError {
        BuildedError {
            builded: message,
            ..Default::default()
        }
    }
}

impl Error {
    /// Create a new error
    /// ## Arguments
    /// * `fields` - [`Vec`] of [`ErrorBuildField`]
    /// * `debug_message` - Development error message used to identify the error in the language not presented to the user
    /// * `pos` - Error position in code by [`crate::defs::Cursor`]
    /// ## Returns
    /// [`Error`]
    /// ## Example
    /// ```
    /// use ellie_core::error::{BuildedError, ErrorBuildField};
    /// let error = error::Error {
    ///    code: 0x00,
    ///    title: "SyntaxError".to_owned(),
    ///    message: "Unexpected Token '$token'".to_owned(),
    ///    ..Default::default()
    /// };
    /// let builded = error.build(vec![
    ///    ErrorBuildField {
    ///       key: String::from("token"),
    ///       value: String::from("Mooo"),
    ///   }
    /// ],String::new("No Debug message for you"), Cursor::default());
    /// ```
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

    /// Create a new error but with path
    /// ## Arguments
    /// * `fields` - [`Vec`] of [`ErrorBuildField`]
    /// * `debug_message` - Development error message used to identify the error in the language not presented to the use
    /// * `path` - Error path
    /// * `pos` - Error position in code by [`crate::defs::Cursor`]
    pub fn build_with_path(
        self,
        fields: Vec<ErrorBuildField>,
        debug_message: String,
        path: String,
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

        error.path = path;
        error.debug_message = debug_message;
        error.pos = pos;
        error.builded_message = BuildedError {
            builded: builded_message,
            fields: used_fields,
        };
        error
    }

    /// Create a new error but without debug_message
    /// ## Arguments
    /// * `fields` - [`Vec`] of [`ErrorBuildField`]
    /// * `debug_message` - Development error message used to identify the error in the language not presented to the use
    /// * `path` - Error path
    /// * `pos` - Error position in code by [`crate::defs::Cursor`]
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
            reference_message: "".to_owned(),
            builded_message: BuildedError::default(),
            pos: crate::defs::Cursor::default(),
            reference_block: None,
            semi_assist: false,
            full_assist: false,
        }
    }
}
