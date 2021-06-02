use crate::syntax::{condition, function, variable};
use crate::{defs, error, parser};
use libc::c_char;
use std::ffi::CString;

#[repr(C)]
pub struct Parsed {
    pub items: *const Collecting,
    pub syntax_errors: *const error::Error,
}

impl Default for Parsed {
    fn default() -> Self {
        let a: [parser::Collecting; 0] = [];
        let b: [error::Error; 0] = [];
        Parsed {
            items: &a as *const parser::Collecting,
            syntax_errors: &b as *const error::Error,
        }
    }
}

#[repr(C)]
pub enum Collecting {
    Variable(variable::VariableCollector),
    Function(function::FunctionCollector),
    Condition(condition::ConditionCollector),
    None,
}

#[repr(C)]
pub struct Parser {
    pub code: *const c_char,
    pub options: defs::ParserOptions,
    pub collected: *const Collecting,
    pub pos: defs::CursorPosition,
    pub ignore_line: bool,
    pub current: Collecting,
    pub keyword_catch: *const c_char,
}

impl Default for Parser {
    fn default() -> Self {
        let c_str = CString::new("").unwrap();
        let a: [parser::Collecting; 0] = [];
        Parser {
            code: c_str.as_ptr() as *const i8,
            options: defs::ParserOptions::default(),
            collected: &a as *const parser::Collecting,
            pos: defs::CursorPosition::default(),
            ignore_line: false,
            current: Collecting::None,
            keyword_catch: c_str.as_ptr() as *const i8,
        }
    }
}
