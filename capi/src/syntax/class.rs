use crate::parser::{defs, Collecting};
use crate::syntax::{types, variable};
use libc::c_char;

#[repr(C)]
pub struct Property {}

#[repr(C)]
pub struct Class {
    name:*const c_char,
    properties: *const Property,
}

#[repr(C)]
pub struct ClassCollector {}
