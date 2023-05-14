//! Ellie Tokenizer
//! This is the code tokenizer for the Ellie Language.
//!
//! Copyright (c) 2020 Behemehal. See license file for details
pub mod iterator;
pub mod macros;
#[doc(hidden)]
pub mod processors;
#[doc(hidden)]
pub mod syntax;
pub mod tokenizer;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(feature = "std")]
extern crate std;
