use crate::definite::types;
use alloc::boxed::Box;
use ellie_core::definite::types::null_resolver;

#[repr(C)]
pub struct NullResolver {
    pub value: Box<types::Types>,
}

pub unsafe fn build_null_resolver_from(target: null_resolver::NullResolver) -> NullResolver {
    NullResolver {
        value: Box::new(types::build_collecting_from(*target.value)),
    }
}
