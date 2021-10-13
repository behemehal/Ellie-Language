use crate::definite::types;
use alloc::boxed::Box;
use ellie_core::definite::types::negative;

#[repr(C)]
pub struct Negative {
    pub value: Box<types::Types>,
}

pub unsafe fn build_negative_from(target: negative::Negative) -> Negative {
    Negative {
        value: Box::new(types::build_collecting_from(*target.value)),
    }
}
