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

#[cfg(feature = "compiler_utils")]
#[macro_use]
extern crate lazy_static;
pub mod com;
#[doc(hidden)]
#[cfg(feature = "compiler_utils")]
pub mod definite;
pub mod defs;
pub mod error;
#[cfg(feature = "compiler_utils")]
pub mod information;
#[cfg(feature = "compiler_utils")]
pub mod module_path;
#[doc(hidden)]
pub mod native;
pub mod raw_type;
#[cfg(feature = "compiler_utils")]
pub mod utils;
pub mod warning;
