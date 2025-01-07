use std::string::String;
use crate::{heap_memory::HeapMemory, stack_memory::StackMemory};

#[derive(Clone)]
pub struct Isolate {
    pub heap_memory: HeapMemory,
    pub stack_memory: StackMemory,
}

impl Default for Isolate {
    fn default() -> Self {
        Self::new()
    }
}

impl Isolate {
    pub fn new() -> Self {
        Isolate {
            heap_memory: HeapMemory::new(),
            stack_memory: StackMemory::new(),
        }
    }

    pub fn heap_dump(&self) -> String {
        self.heap_memory.dump()
    }

    pub fn stack_dump(&self) -> String {
        self.stack_memory.dump()
    }
}
