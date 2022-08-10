#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
/*
    Copyright (c) 2020 Behemehal. See license file for details
*/

#[cfg(feature = "build-cli")]
extern crate std;

extern crate alloc;

#[cfg(feature = "compiler")]
pub extern crate ellie_bytecode;
/// EllieCore contains various functions and structs used by Ellie.
pub extern crate ellie_core;
#[cfg(feature = "compiler")]
pub extern crate ellie_parser;
#[cfg(feature = "compiler")]
pub extern crate ellie_tokenizer;
#[cfg(feature = "vm")]
pub extern crate ellie_vm;

#[cfg(feature = "compiler")]
pub mod engine_constants;

#[cfg(feature = "compiler")]
pub mod compiler;
#[cfg(feature = "compiler")]
pub mod tokenizer;
#[cfg(feature = "vm")]
pub mod vm;

pub mod utils;

#[cfg(feature = "build-cli")]
pub mod terminal_utils;

/// Virtual channels for communication between compiler and code

#[cfg(feature = "build-cli")]
pub mod binary_tools;
