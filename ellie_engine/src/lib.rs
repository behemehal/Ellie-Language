#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
//! Ellie Engine
//! This is the main repository for the Ellie Language.
//!
//! Copyright (c) 2020 Behemehal. See license file for details
//! # Internal Dependencies
//! - ellie_core - Contains various functions and structs used by Ellie.
//! - ellie_tokenizer - Contains the tokenizer for Ellie.
//! - ellie_parser - Contains the parser for Ellie.
//! - ellie_formatter - Contains the formatter for Ellie.
//! - ellie_bytecode - Contains the bytecode for Ellie.
//! - ellie_vm - Contains the virtual machine for Ellie

#[cfg(any(feature = "std", feature = "cli-utils"))]
extern crate std;

extern crate alloc;

/// EllieBytecode is the bytecode assembler for EllieVm.
#[cfg(feature = "compiler")]
pub extern crate ellie_bytecode;
/// EllieCore contains various functions and structs used by Ellie.
pub extern crate ellie_core;
/// EllieFmt is the formatter for Ellie.
#[cfg(feature = "fmt")]
pub extern crate ellie_fmt;
/// EllieParser is the parser for tokenized code of Ellie.
#[cfg(feature = "compiler")]
pub extern crate ellie_parser;
/// EllieRendererUtils contains various utilities for elliec, elliefmt and ellievm binaries.
#[cfg(any(feature = "renderer_utils", feature = "cli-utils"))]
pub extern crate ellie_renderer_utils;
/// EllieTokenizer is the tokenizer for Ellie.
#[cfg(feature = "compiler")]
pub extern crate ellie_tokenizer;
/// EllieVm is the virtual machine for Ellie.
#[cfg(feature = "vm")]
pub extern crate ellie_vm;

#[doc(hidden)]
pub mod engine_constants;

/// This module contains utilities that easing up usage of parser
#[cfg(feature = "compiler")]
pub mod compiler;
/// This module contains utilities that easing up usage of tokenizer
#[cfg(feature = "compiler")]
pub mod tokenizer;
/// This module contains utilities that easing up usage of vm
#[cfg(feature = "vm")]
pub mod vm;

/// Various utilities for all modules.
pub mod utils;
