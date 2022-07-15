/*
    Copyright (c) 2020 Behemehal. See license file for details
*/

#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
extern crate ellie_core;
extern crate ellie_parser;
extern crate ellie_tokenizer;
extern crate ellie_vm;

#[macro_use]
extern crate lazy_static;
pub mod cli_outputs;
pub mod cli_utils;
pub mod compile_file;
pub mod engine_constants;
pub mod run_vm;
pub mod tokenize_file;
pub mod view_module;

#[cfg(feature = "build-cli")]
pub mod cli_options;
