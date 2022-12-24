/*
    Copyright (c) 2020 Behemehal. See license file for details
*/

#[macro_use]
extern crate lazy_static;
pub mod rules;

pub struct Rule<T, E> {
    pub warning_id: u8,
    pub worker: fn(T) -> E,
}
