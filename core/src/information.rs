use alloc::vec::Vec;
use core::any::Any;
use core::clone::Clone;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Informations {
    pub warnings: Vec<crate::warning::Warning>,
    pub errors: Vec<crate::error::Error>,
}

impl Informations {
    pub fn new() -> Informations {
        Informations {
            warnings: Vec::new(),
            errors: Vec::new(),
        }
    }

    pub fn has_no_errors(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn has_no_warnings(&self) -> bool {
        self.warnings.is_empty()
    }

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
