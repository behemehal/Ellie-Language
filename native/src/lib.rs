#![feature(lang_items, start)]
#![no_std]
#[cfg(not(feature = "std"))]
extern crate alloc;

#[cfg(feature = "std")]
extern crate std;

pub mod ellie_types;
pub mod ellie_type_define;
pub mod ellie_function;