/*
    Copyright (c) 2020 Behemehal. See license file for details
*/
#![no_std]

//!Ellie VM
//!This is the virtual machine for the Ellie Language.

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "mialloc")]
use mimalloc::MiMalloc;

#[cfg(feature = "mialloc")]
#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

extern crate alloc;

/// Ellie native channels
pub mod channel;
/// Memory Stack configuration for VM
pub mod config;
/// Ellie VM's heap memory
pub mod heap_memory;
/// Ellie VM's instruction utils
pub mod instruction_utils;
/// Ellie VM's instructions
pub mod instructions;
/// Ellie VM's program
pub mod program;
/// Ellie VM's stack
pub mod stack;
/// Ellie VM's stack memory
pub mod stack_memory;
/// Ellie VM's thread
pub mod thread;
/// Ellie VM's utils
pub mod utils;
/// Ellie VM's raw type
pub mod raw_type;
