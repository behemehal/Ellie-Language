#![allow(warnings)]

use std::ops::{Deref, DerefMut};

lazy_static! {
    //pub static ref FOO : ::std::time::SystemTime = ::std::time::SystemTime::now();
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub static ref error_s1: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x00,
        title: "SyntaxError".to_string(),
        message: "Unexpected Token '$token'".to_string(),
        builded_message: "".to_string(),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };


    pub static ref error_s2: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x01,
        title: "SyntaxError".to_string(),
        message: "Expected return type".to_string(),
        builded_message: "".to_string(),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };

    pub static ref error_s3: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x02,
        title: "RefferenceError".to_string(),
        message: "Expected '$token1' found '$token2'".to_string(),
        builded_message: "".to_string(),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };

    pub static ref error_s4: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x03,
        title: "RefferenceError".to_string(),
        message: "Targeted variable '$token' not found in scope".to_string(),
        builded_message: "".to_string(),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };

    pub static ref error_s5: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x04,
        title: "RefferenceError".to_string(),
        message: "Unexpected Return Type '$token'".to_string(),
        builded_message: "".to_string(),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };

    pub static ref error_s6: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x05,
        title: "RefferenceError".to_string(),
        message: "'$token' is not defined".to_string(),
        builded_message: String::from(""),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };

    pub static ref error_s7: crate::error::Error = crate::error::Error {
        debug_message: "".to_string(),
        code: 0x06,
        title: "RefferenceError".to_string(),
        message: "Insufficent parameters supplied, Function requires '$token' parameters found '$token2' length of parameters".to_string(),
        builded_message: "".to_string(),
        pos: crate::mapper::defs::Cursor {
                    range_start: crate::mapper::defs::CursorPosition(0, 0),
                    range_end: crate::mapper::defs::CursorPosition(0, 0)
                }
    };
}
