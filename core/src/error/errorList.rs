#![allow(warnings)]
use alloc::string::ToString;
use std::ops::{Deref, DerefMut};

lazy_static! {
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
        title: "ReferenceError".to_string(),
        message: "Expected '$token1' found '$token2'".to_string(),
        ..Default::default()
    };
    pub static ref error_s4: crate::error::Error = crate::error::Error {
        code: 0x03,
        title: "ReferenceError".to_string(),
        message: "Targeted variable '$token' not found in scope".to_string(),
        ..Default::default()
    };
    pub static ref error_s5: crate::error::Error = crate::error::Error {
        code: 0x04,
        title: "ReferenceError".to_string(),
        message: "Unexpected Return Type '$token'".to_string(),
        ..Default::default()
    };
    pub static ref error_s6: crate::error::Error = crate::error::Error {
        code: 0x05,
        title: "ReferenceError".to_string(),
        message: "'$token' is not defined".to_string(),
        ..Default::default()
    };
    pub static ref error_s7: crate::error::Error = crate::error::Error {
        code: 0x06,
        title: "ReferenceError".to_string(),
        message: "$name requires '$token' parameters, found '$token2' length of parameters"
            .to_string(),
        ..Default::default()
    };
    pub static ref error_s8: crate::error::Error = crate::error::Error {
        code: 0x07,
        title: "ReferenceError".to_string(),
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
        message: "Duplicate parameter".to_string(),
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
        message: "Char type can take one character only".to_string(),
        ..Default::default()
    };
    pub static ref error_s16: crate::error::Error = crate::error::Error {
        code: 0x15,
        title: "OverflowError".to_string(),
        message: "The value '$val' cannot fit to integer".to_string(),
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
    pub static ref error_s19: crate::error::Error = crate::error::Error {
        code: 0x18,
        title: "OverflowError".to_string(),
        message: "Fixed size exceeded: expected '$token' elements, got '$token2' elements"
            .to_string(),
        ..Default::default()
    };
    pub static ref error_s20: crate::error::Error = crate::error::Error {
        code: 0x19,
        title: "TypeError".to_string(),
        message: "Fixed size required".to_string(),
        ..Default::default()
    };
    pub static ref error_s21: crate::error::Error = crate::error::Error {
        code: 0x20,
        title: "SyntaxError".to_string(),
        message: "Reserved keyword '$token'".to_string(),
        ..Default::default()
    };
    pub static ref error_s22: crate::error::Error = crate::error::Error {
        code: 0x21,
        title: "SyntaxError".to_string(),
        message: "Constructor name should be same as class name".to_string(),
        ..Default::default()
    };
    pub static ref error_s23: crate::error::Error = crate::error::Error {
        code: 0x22,
        title: "SyntaxError".to_string(),
        message: "'$token' is not recognized as a command (Possible unimplemented)".to_string(),
        ..Default::default()
    };
    pub static ref error_s24: crate::error::Error = crate::error::Error {
        code: 0x23,
        title: "ReferenceError".to_string(),
        message: "'$token' is already defined".to_string(),
        ..Default::default()
    };
    pub static ref error_s25: crate::error::Error = crate::error::Error {
        code: 0x24,
        title: "TypeError".to_string(),
        message: "'$token' is not a function".to_string(),
        ..Default::default()
    };
    pub static ref error_s26: crate::error::Error = crate::error::Error {
        code: 0x25,
        title: "SyntaxError".to_string(),
        message: "Unexpected ending".to_string(),
        ..Default::default()
    };
    pub static ref error_s27: crate::error::Error = crate::error::Error {
        code: 0x26,
        title: "TypeError".to_string(),
        message: "Cannot apply data to generic type".to_string(),
        ..Default::default()
    };
    pub static ref error_s28: crate::error::Error = crate::error::Error {
        code: 0x27,
        title: "ImportError".to_string(),
        message: "Cannot resolve '$token' module".to_string(),
        ..Default::default()
    };
    pub static ref error_s29: crate::error::Error = crate::error::Error {
        code: 0x28,
        title: "TypeError".to_string(),
        message: "Unexpected Token '$token' is not iterable".to_string(),
        ..Default::default()
    };
    pub static ref error_s30: crate::error::Error = crate::error::Error {
        code: 0x29,
        title: "TypeError".to_string(),
        message: "Class can only have one constructor".to_string(),
        ..Default::default()
    };
    pub static ref error_s31: crate::error::Error = crate::error::Error {
        code: 0x30,
        title: "TypeError".to_string(),
        message: "'$token' is not a constructable".to_string(),
        ..Default::default()
    };
    pub static ref error_s32: crate::error::Error = crate::error::Error {
        code: 0x31,
        title: "ImportError".to_string(),
        message: "$token".to_string(),
        ..Default::default()
    };
    pub static ref error_s33: crate::error::Error = crate::error::Error {
        code: 0x32,
        title: "ImportError".to_string(),
        message: "Cannot compile '$token'".to_string(),
        ..Default::default()
    };
    pub static ref error_s34: crate::error::Error = crate::error::Error {
        code: 0x33,
        title: "TypeError".to_string(),
        message: "'$token' is not found in properties".to_string(),
        ..Default::default()
    };
}
