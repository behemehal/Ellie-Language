#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
//! EllieParser
//! This is the parser for the Ellie Language.
//!
//! Copyright (c) 2020 Behemehal. See license file for details

#[cfg(feature = "standard_rules")]
extern crate ellie_standard_rules;

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod deep_search_extensions;
pub mod parser;
pub mod processors;
pub mod utils;
