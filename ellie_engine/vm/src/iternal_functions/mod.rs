use alloc::vec::Vec;

use crate::{isolate::Isolate, utils::{FunctionCallParameter, ThreadInfo, VmNativeAnswer}};

mod memory;
mod static_array_functions;

pub struct InternalFunction {
    pub name: &'static str,
    pub callback: fn(&mut Isolate, ThreadInfo, Vec<FunctionCallParameter>) -> VmNativeAnswer,
}

pub const INTERNAL_FUNCTIONS: [InternalFunction; 8] = [
    static_array_functions::ARRAY_LEN,
    memory::FRAME_POS,
    memory::CODE_POS,
    memory::ALLOC,
    memory::PLATFORM_SIZE,
    memory::REALLOC,
    memory::PTR,
    memory::INDEX_MUT,
];
