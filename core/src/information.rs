use alloc::vec::Vec;
use core::any::Any;
use core::clone::Clone;
use serde::{Deserialize, Serialize};

/// Parser's output colider, it contains a list of warnings and errors and implements some of the [`Vec`] methods
/// ## Fields
/// * `warnings` - [`Vec`] of [`Warning`]
/// * `errors` - [`Vec`] of [`Error`]
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Informations {
    pub warnings: Vec<crate::warning::Warning>,
    pub errors: Vec<crate::error::Error>,
}

impl Informations {
    /// Create a new [`Informations`] instance
    pub fn new() -> Informations {
        Informations {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    /// Check if there is any error
    pub fn has_no_errors(&self) -> bool {
        self.errors.is_empty()
    }

    /// Check if there is any warning
    pub fn has_no_warnings(&self) -> bool {
        self.warnings.is_empty()
    }

    /// Implement [`Vec`] like interface for [`Warning`] and [`Error`] in one method
    pub fn extend(&mut self, item: &dyn Any) {
        if item.is::<Vec<crate::warning::Warning>>() {
            self.warnings.extend(
                item.downcast_ref::<Vec<crate::warning::Warning>>()
                    .unwrap()
                    .clone(),
            );
        } else if item.is::<Vec<crate::error::Error>>() {
            self.errors.extend(
                item.downcast_ref::<Vec<crate::error::Error>>()
                    .unwrap()
                    .clone(),
            );
        }
    }

    /// Implement [`Vec`] like interface for [`Warning`] and [`Error`] in one method
    pub fn push(&mut self, item: &dyn Any) {
        if item.is::<crate::warning::Warning>() {
            self.warnings.push(
                item.downcast_ref::<crate::warning::Warning>()
                    .unwrap()
                    .clone(),
            );
        } else if item.is::<crate::error::Error>() {
            self.errors
                .push(item.downcast_ref::<crate::error::Error>().unwrap().clone());
        }
    }
}
