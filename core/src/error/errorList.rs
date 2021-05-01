#![allow(warnings)]
use std::ops::{Deref, DerefMut};

lazy_static! {
    #[derive(PartialEq, Debug, Clone, Copy)]
    pub static ref error_s1: crate::error::Error = crate::error::Error {
        code: 0x00,
        title: "SyntaxError".to_string(),
        message: "Unexpected Token '$token'".to_string(),
        ..Default::default()
    };

    pub static ref error_s2: crate::error::Error = crate::error::Error {
        code: 0x01,
        title: "SyntaxError".to_string(),
        message: "Expected return type".to_string(),
        ..Default::default()
    };

    pub static ref error_s3: crate::error::Error = crate::error::Error {
        code: 0x02,
        title: "RefferenceError".to_string(),
        message: "Expected '$token1' found '$token2'".to_string(),
        ..Default::default()
    };

    pub static ref error_s4: crate::error::Error = crate::error::Error {
        code: 0x03,
        title: "RefferenceError".to_string(),
        message: "Targeted variable '$token' not found in scope".to_string(),
        ..Default::default()
    };

    pub static ref error_s5: crate::error::Error = crate::error::Error {
        code: 0x04,
        title: "RefferenceError".to_string(),
        message: "Unexpected Return Type '$token'".to_string(),
        ..Default::default()
    };

    pub static ref error_s6: crate::error::Error = crate::error::Error {
        code: 0x05,
        title: "RefferenceError".to_string(),
        message: "'$token' is not defined".to_string(),
        ..Default::default()
    };

    pub static ref error_s7: crate::error::Error = crate::error::Error {
        code: 0x06,
        title: "RefferenceError".to_string(),
        message: "Insufficent parameters supplied, Function requires '$token' parameters found '$token2' length of parameters".to_string(),
        ..Default::default()
    };

    pub static ref error_s8: crate::error::Error = crate::error::Error {
        code: 0x07,
        title: "RefferenceError".to_string(),
        message: "Expected type annotations".to_string(),
        ..Default::default()
    };

    pub static ref error_s9: crate::error::Error = crate::error::Error {
        code: 0x08,
        title: "TypeError".to_string(),
        message: "Unknown operator '$token'".to_string(),
        ..Default::default()
    };

    pub static ref error_s10: crate::error::Error = crate::error::Error {
        code: 0x09,
        title: "TypeError".to_string(),
        message: "Duplicate parameter found".to_string(),
        ..Default::default()
    };

    pub static ref error_s11: crate::error::Error = crate::error::Error {
        code: 0x10,
        title: "TypeError".to_string(),
        message: "Cannot set type annotations on dynamic variable".to_string(),
        ..Default::default()
    };

    pub static ref error_s12: crate::error::Error = crate::error::Error {
        code: 0x11,
        title: "SyntaxError".to_string(),
        message: "Expected operator found value instead, '$token'".to_string(),
        ..Default::default()
    };

    pub static ref error_s13: crate::error::Error = crate::error::Error {
        code: 0x12,
        title: "SyntaxError".to_string(),
        message: "Expected operator found '$token'".to_string(),
        ..Default::default()
    };

    pub static ref error_s14: crate::error::Error = crate::error::Error {
        code: 0x13,
        title: "TypeError".to_string(),
        message: "Cannot leave char empty".to_string(),
        ..Default::default()
    };

    pub static ref error_s15: crate::error::Error = crate::error::Error {
        code: 0x14,
        title: "TypeError".to_string(),
        message: "Char type can take one charachter only".to_string(),
        ..Default::default()
    };

    pub static ref error_s16: crate::error::Error = crate::error::Error {
        code: 0x15,
        title: "OverflowError".to_string(),
        message: "The value '$val' cannot fit to type '$type'".to_string(),
        ..Default::default()
    };

    pub static ref error_s17: crate::error::Error = crate::error::Error {
        code: 0x16,
        title: "OverflowError".to_string(),
        message: "The value '$val' has infinite size".to_string(),
        ..Default::default()
    };

    pub static ref error_s18: crate::error::Error = crate::error::Error {
        code: 0x17,
        title: "TypeError".to_string(),
        message: "Types cannot have number properties".to_string(),
        ..Default::default()
    };
}