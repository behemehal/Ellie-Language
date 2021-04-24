use crate::mapper::{defs, Collecting};
use crate::syntax::{types, variable};

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Property {

}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct Class {
    name: String,
    properties: Vec,
}

#[derive(PartialEq, Debug, Clone, Default)]
pub struct ClassCollector {

}