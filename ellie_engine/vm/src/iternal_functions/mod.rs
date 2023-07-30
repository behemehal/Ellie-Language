use alloc::vec::Vec;

use crate::{
    thread::Isolate,
    utils::{VmNativeAnswer, VmNativeCallParameters},
};

mod array_len;

pub struct InternalFunction {
    pub name: &'static str,
    pub callback: fn(&mut Isolate, Vec<VmNativeCallParameters>) -> VmNativeAnswer,
}

pub const INTERNAL_FUNCTIONS: [InternalFunction; 1] = [array_len::ARRAY_LEN];
