use alloc::vec::Vec;

use crate::{
    thread::Isolate,
    utils::{ThreadInfo, VmNativeAnswer, VmNativeCallParameters},
};

mod memory;
mod static_array_functions;

pub struct InternalFunction {
    pub name: &'static str,
    pub callback: fn(&mut Isolate, ThreadInfo, Vec<VmNativeCallParameters>) -> VmNativeAnswer,
}

pub const INTERNAL_FUNCTIONS: [InternalFunction; 3] = [
    static_array_functions::ARRAY_LEN,
    memory::FRAME_POS,
    memory::CODE_POS,
];
