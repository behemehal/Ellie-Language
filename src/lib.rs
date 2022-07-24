/*
    Copyright (c) 2020 Behemehal. See license file for details
*/

#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
extern crate ellie_core;

#[cfg(feature = "parser")]
extern crate ellie_parser;
#[cfg(feature = "tokenizer")]
extern crate ellie_tokenizer;
#[cfg(feature = "vm")]
extern crate ellie_vm;

#[macro_use]
extern crate lazy_static;
pub mod engine_constants;

#[cfg(feature = "tokenizer")]
pub mod tokenizer;
//Use cfg tokenizer and parser feature

#[cfg(all(feature = "tokenizer", feature = "parser"))]
pub mod compile;

#[cfg(feature = "terminal-utils")]
pub mod terminal_utils;

#[cfg(all(feature = "tokenizer", feature = "parser"))]
pub mod utils;

#[cfg(feature = "build-cli")]
pub mod cli_options;
#[cfg(feature = "build-cli")]
pub mod cli_outputs;
#[cfg(feature = "build-cli")]
pub mod cli_utils;
#[cfg(feature = "build-cli")]
pub mod compile_file;
#[cfg(feature = "build-cli")]
pub mod run_vm;
#[cfg(feature = "build-cli")]
pub mod tokenize_file;
#[cfg(feature = "build-cli")]
pub mod view_module;
