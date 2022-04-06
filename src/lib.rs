/*
    Copyright (c) 2020 Behemehal. See license file for details
*/

#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]

//TODO: no-std use core::alloc::Layout; Nightly
//TODO: no-std extern crate alloc; Nightly

extern crate ellie_core;
extern crate ellie_tokenizer;

#[cfg(feature = "build-cli")]
#[macro_use]
extern crate lazy_static;
pub mod cli_outputs;
#[cfg(feature = "build-cli")]
pub mod cli_utils;
pub mod compile_file;
pub mod engine_constants;
pub mod tokenize_file;
pub mod view_module;
