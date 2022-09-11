use alloc::{
    string::{String, ToString},
    vec::Vec,
};
use ellie_core::{
    defs::{PlatformArchitecture, VmNativeAnswer, VmNativeCall},
    raw_type::RawType,
};

use crate::{
    heap::Heap,
    program::{Program, ReadInstruction, MainProgram},
    thread::{Registers, Stack, Thread},
    utils::{ThreadExit, ThreadStepInfo},
};

pub struct VM<T> {
    pub stack: Vec<ReadInstruction>,
    pub threads: Vec<Thread<T>>,
    pub heap: Heap,
    native_call_channel: T,
    target_arch: PlatformArchitecture,
}

impl<T> VM<T>
where
    T: Fn(crate::thread::ThreadInfo, VmNativeCall) -> VmNativeAnswer + Clone + Copy + Sized,
{
    pub fn new(target_arch: PlatformArchitecture, native_call_channel: T) -> Self {
        VM {
            stack: Vec::new(),
            threads: Vec::new(),
            heap: Heap::new(),
            target_arch,
            native_call_channel,
        }
    }

    pub fn load(&mut self, program: &Program) -> Result<(), u8> {
        if self.target_arch != program.arch {
            return Err(1);
        }
        self.stack = program.instructions.clone();
        Ok(())
    }

    pub fn build_thread(&mut self, heap: &mut Heap, main: usize) {
        let thread = Thread::new(
            main,
            self.target_arch,
            self.stack.clone(),
            self.native_call_channel,
        );
        self.threads.push(thread);
    }

    pub fn build_main_thread(&mut self, main: MainProgram) {
        let mut thread = Thread::new(
            main.hash,
            self.target_arch,
            self.stack.clone(),
            self.native_call_channel,
        );
        thread
            .stack
            .push(Stack {
                id: main.hash,
                name: "<ellie_main>".to_string(),
                registers: Registers {
                    A: RawType::void(),
                    B: RawType::void(),
                    C: RawType::void(),
                    X: RawType::void(),
                    Y: RawType::void(),
                },
                stack_len: main.length,
                caller: None,
                stack_pos: main.start,
                frame_pos: main.start,
            })
            .unwrap();
        self.threads.push(thread);
    }

    pub fn heap_dump(&mut self) -> String {
        self.heap.dump()
    }
}
