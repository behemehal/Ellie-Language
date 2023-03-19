#[cfg(feature = "std")]
use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::Mutex;

use alloc::{string::String, vec::Vec};
use ellie_core::{defs::PlatformArchitecture, raw_type::StaticRawType};

use crate::{
    channel::ModuleManager,
    heap_memory::HeapMemory,
    program::{MainProgram, Program},
    stack::Stack,
    stack_memory::StackMemory,
    thread::{Registers, Thread},
    utils::ThreadExit,
};

pub struct VM<'a> {
    pub program: &'a Program,
    pub threads: Vec<Thread>,
    #[cfg(feature = "std")]
    pub heap_memory: Arc<Mutex<HeapMemory>>,
    #[cfg(feature = "std")]
    pub stack_memory: Arc<Mutex<StackMemory>>,
    //Unified memory
    #[cfg(not(feature = "std"))]
    pub heap_memory: HeapMemory,
    #[cfg(not(feature = "std"))]
    pub stack_memory: StackMemory,
    module_manager: ModuleManager,
    target_arch: PlatformArchitecture,
}

impl<'a> VM<'a> {
    #[cfg(feature = "std")]
    pub fn new(
        target_arch: PlatformArchitecture,
        module_manager: ModuleManager,
        program: &'a Program,
    ) -> Self {
        VM {
            program,
            threads: Vec::new(),
            heap_memory: Arc::new(Mutex::new(HeapMemory::new())),
            stack_memory: Arc::new(Mutex::new(StackMemory::new())),
            module_manager,
            target_arch,
        }
    }

    #[cfg(not(feature = "std"))]
    pub fn new(
        target_arch: PlatformArchitecture,
        module_manager: ModuleManager,
        program: &'a Program,
    ) -> Self {
        VM {
            program,
            threads: Vec::new(),
            heap_memory: HeapMemory::new(),
            stack_memory: StackMemory::new(),
            target_arch,
            module_manager,
        }
    }

    pub fn create_thread(&mut self, main: MainProgram) -> Result<(), u8> {
        let mut thread = Thread::new(main.hash, self.target_arch);
        thread.stack.push(Stack {
            id: main.hash,
            registers: Registers {
                A: StaticRawType::void(),
                B: StaticRawType::void(),
                C: StaticRawType::void(),
                X: StaticRawType::void(),
                Y: StaticRawType::void(),
            },
            stack_len: main.length,
            caller: None,
            pos: main.start,
            frame_pos: thread.frame_pos,
        });
        self.threads.push(thread);
        Ok(())
    }

    #[cfg(feature = "std")]
    pub fn run_thread(&mut self, thread_id: usize) -> Option<ThreadExit> {
        let thread_idx = self
            .threads
            .iter()
            .position(|thread| thread.id == thread_id)
            .unwrap();
        let thread = &mut self.threads[thread_idx];
        let mut heap_memory = self.heap_memory.lock().unwrap();
        let mut stack_memory = self.stack_memory.lock().unwrap();
        let mut module_manager = &mut self.module_manager;
        let thread_exit = thread.run(
            &mut heap_memory,
            &mut stack_memory,
            &mut module_manager,
            &self.program,
        );
        self.threads.remove(thread_idx);
        thread_exit
    }

    #[cfg(not(feature = "std"))]
    pub fn run_thread(&mut self, thread_id: usize) -> Option<ThreadExit> {
        let thread_idx = self
            .threads
            .iter()
            .position(|thread| thread.id == thread_id)
            .unwrap();
        let thread = &mut self.threads[thread_idx];
        let thread_exit = thread.run(
            &mut self.heap_memory,
            &mut self.stack_memory,
            &mut self.module_manager,
            &self.program,
        );
        self.threads.remove(thread_idx);
        thread_exit
    }

    #[cfg(not(feature = "std"))]
    pub fn stack_dump(&mut self) -> String {
        self.stack_memory.dump()
    }

    #[cfg(not(feature = "std"))]
    pub fn heap_dump(&mut self) -> String {
        self.heap_memory.dump()
    }

    #[cfg(feature = "std")]
    pub fn stack_dump(&mut self) -> String {
        self.stack_memory.lock().unwrap().dump()
    }

    #[cfg(feature = "std")]
    pub fn heap_dump(&mut self) -> String {
        self.heap_memory.lock().unwrap().dump()
    }
}
