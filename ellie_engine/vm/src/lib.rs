/*
    Copyright (c) 2020 Behemehal. See license file for details
*/
#![no_std]

#[cfg(feature = "std")]
extern crate std;

use mimalloc::MiMalloc;

#[global_allocator]
static GLOBAL: MiMalloc = MiMalloc;

extern crate alloc;

pub mod channel;
pub mod heap_memory;
pub mod instruction_utils;
pub mod instructions;
pub mod program;
pub mod stack_memory;
pub mod thread;
pub mod utils;
pub mod vm;
pub mod stack;
pub mod config;
