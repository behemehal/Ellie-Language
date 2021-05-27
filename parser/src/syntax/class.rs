use crate::parser::{defs, Collecting};
use crate::syntax::{types, variable};

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Property {}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct Class {
    name: String,
    properties: Vec,
}

#[repr(C)]
#[derive(PartialEq, Debug, Clone, Default)]
pub struct ClassCollector {}
