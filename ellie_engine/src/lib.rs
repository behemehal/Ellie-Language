#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
//! Ellie Engine
//! This is the main repository for the Ellie Language.
//!
//! Copyright (c) 2020 Behemehal. See license file for details

#[cfg(feature = "build-cli")]
extern crate std;

extern crate alloc;

#[cfg(feature = "compiler")]
pub extern crate ellie_bytecode;
/// EllieCore contains various functions and structs used by Ellie.
pub extern crate ellie_core;
#[cfg(feature = "fmt")]
pub extern crate ellie_fmt;
#[cfg(feature = "compiler")]
pub extern crate ellie_parser;
#[cfg(any(feature = "renderer_utils", feature = "cli-utils"))]
pub extern crate ellie_renderer_utils;
#[cfg(feature = "compiler")]
pub extern crate ellie_tokenizer;
#[cfg(feature = "vm")]
pub extern crate ellie_vm;

#[doc(hidden)]
pub mod engine_constants;

#[cfg(feature = "compiler")]
pub mod compiler;
#[cfg(feature = "compiler")]
pub mod tokenizer;
#[cfg(feature = "vm")]
pub mod vm;

pub mod utils;
