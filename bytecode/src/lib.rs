/*
    Copyright (c) 2020 Behemehal. See license file for details
*/
#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

/// Assembles ellie elements
pub mod assembler;

/// Bytecode instructions
pub mod instructions;

/// Utils
pub mod utils;
