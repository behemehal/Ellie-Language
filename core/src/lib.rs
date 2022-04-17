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

#[macro_use]
extern crate lazy_static;
pub mod com;
#[doc(hidden)]
pub mod definite;
pub mod defs;
pub mod error;
pub mod information;
pub mod module_path;
#[doc(hidden)]
pub mod native;
pub mod utils;
pub mod warning;
