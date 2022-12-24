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

/// Instruction table
pub mod instruction_table;

/// Bytecode instructions
pub mod instructions;

/// Utils
pub mod utils;

//Transpilers
pub mod transpiler;

#[macro_use]
extern crate lazy_static;
