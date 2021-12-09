/*
    Copyright (c) 2020 Behemehal. See license file for details
*/
#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]

#[cfg(feature = "std")]
extern crate std;
extern crate alloc;

pub mod parser;
pub mod processors;