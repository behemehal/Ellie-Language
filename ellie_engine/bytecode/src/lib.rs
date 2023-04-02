#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
//! Ellie Bytecode
//! This is the bytecode assembler for the Ellie Language.
//!
//! Copyright (c) 2020 Behemehal. See license file for details

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
pub mod addressing_modes;
pub mod macros;
pub mod transpiler;
pub mod types;
