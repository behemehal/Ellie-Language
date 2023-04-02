#![no_std]
#![allow(unused_assignments)]
#![allow(unknown_lints)]
#![warn(clippy::all)]
//! Ellie Core
//! This is the core utilities for the Ellie Language.
//!
//! Copyright (c) 2020 Behemehal. See license file for details
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

#[cfg(feature = "compiler_utils")]
#[macro_use]
extern crate lazy_static;
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
