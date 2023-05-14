#[cfg(feature = "cli-utils")]
extern crate std;

extern crate alloc;

#[cfg(feature = "cli-utils")]
/// CLI arg options
pub mod options;

#[cfg(feature = "cli-utils")]
/// CLI outputs for JSON
pub mod outputs;

/// Utilities
pub mod utils;
