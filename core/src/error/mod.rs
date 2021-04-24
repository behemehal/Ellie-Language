use serde::Serialize;
pub mod errorList;

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct Error {
    pub code: u8,
    pub message: String,
    pub title: String,
    pub builded_message: String,
    pub debug_message: String,
    pub pos: crate::defs::Cursor
}


pub struct ErrorBuildField {
    pub key: String,
    pub value: String
}

impl Error {
    pub fn build(body: String, fields: Vec<ErrorBuildField>) -> String {
        let mut builded_message = body.to_string();
        for field in fields {
            let key: String = '$'.to_string() + &field.key.to_string();
            if let Some(pos) = builded_message.find(&key) {
                builded_message.replace_range(pos..(pos+key.len()), &field.value)
            } else {
                panic!("Failed to parse error '{}'", body);
            }
        }
        builded_message
    }
}

impl Default for Error {
    fn default() -> Error {
        Error {
            debug_message: "".to_string(),
            code: 0x00,
            title: "".to_string(),
            message: "".to_string(),
            builded_message: "".to_string(),
            pos: crate::defs::Cursor {
                range_start: crate::defs::CursorPosition(0, 0), 
                range_end: crate::defs::CursorPosition(0, 0)
            }
        }
    }
}