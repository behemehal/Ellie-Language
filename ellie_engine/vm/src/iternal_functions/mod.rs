use alloc::vec::Vec;

use crate::{
    thread::Isolate,
    utils::{ThreadInfo, VmNativeAnswer, VmNativeCallParameters},
};

mod stack_memory_functions;
mod static_array_functions;

pub struct InternalFunction {
    pub name: &'static str,
    pub callback: fn(&mut Isolate, ThreadInfo, Vec<VmNativeCallParameters>) -> VmNativeAnswer,
}

pub const INTERNAL_FUNCTIONS: [InternalFunction; 3] = [
    static_array_functions::ARRAY_LEN,
    stack_memory_functions::FRAME_POS,
    stack_memory_functions::CODE_POS,
];
