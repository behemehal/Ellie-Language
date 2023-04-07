#[cfg(feature = "std")]
use alloc::sync::Arc;
#[cfg(feature = "std")]
use std::sync::Mutex;

use alloc::boxed::Box;
use ellie_core::defs::VmNativeAnswer;
use ellie_core::defs::VmNativeCall;

use alloc::{string::String, vec::Vec};
use ellie_core::{defs::PlatformArchitecture, raw_type::StaticRawType};

use crate::{
    channel::ModuleManager,
    config::PROGRAM_MAX_SIZE,
    heap_memory::HeapMemory,
    instruction_utils::Instructions,
    program::{MainProgram, Program, ReadInstruction},
    stack::Stack,
    stack_memory::StackMemory,
    thread::{Registers, Thread},
    utils::{AddressingValues, ThreadExit},
};

pub struct VmProgram {
    pub instructions: [ReadInstruction; PROGRAM_MAX_SIZE],
    pub length: usize,
}

impl VmProgram {
    pub fn new() -> Self {
        VmProgram {
            instructions: [ReadInstruction::default(); PROGRAM_MAX_SIZE],
            length: 0,
        }
    }

    pub fn fill_from_vector(&mut self, program: Vec<ReadInstruction>) {
        for (idx, instruction) in program.iter().enumerate() {
            self.instructions[idx] = *instruction;
        }
        self.length = program.len();
    }
}

pub struct VM {
    pub program: VmProgram,
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
    target_arch: PlatformArchitecture,
}

impl VM {
    #[cfg(feature = "std")]
    pub fn new(target_arch: PlatformArchitecture) -> Self {
        VM {
            program: VmProgram::new(),
            threads: Vec::new(),
            heap_memory: Arc::new(Mutex::new(HeapMemory::new())),
            stack_memory: Arc::new(Mutex::new(StackMemory::new())),
            target_arch,
        }
    }

    pub fn load_program(&mut self, program: Vec<ReadInstruction>) {
        self.program.fill_from_vector(program);
    }

    #[cfg(not(feature = "std"))]
    pub fn new(target_arch: PlatformArchitecture, program: &'a Program) -> Self {
        VM {
            program,
            threads: Vec::new(),
            heap_memory: HeapMemory::new(),
            stack_memory: StackMemory::new(),
            target_arch,
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
    pub fn run_thread(
        &mut self,
        thread_id: usize,
        module_manager: &mut ModuleManager,
    ) -> Option<ThreadExit> {
        let thread_idx = self
            .threads
            .iter()
            .position(|thread| thread.id == thread_id)
            .unwrap();
        let thread = &mut self.threads[thread_idx];
        let mut heap_memory = self.heap_memory.lock().unwrap();
        let mut stack_memory = self.stack_memory.lock().unwrap();
        let thread_exit = thread.run(
            &mut heap_memory,
            &mut stack_memory,
            module_manager,
            &mut self.program,
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

    pub fn generate_main_from_function(&self, target_hash: usize) -> Result<MainProgram, u8> {
        let mut i = 0;
        while i < self.program.length {
            let instruction = self.program.instructions[i];
            match instruction.instruction {
                Instructions::FN(_) => match instruction.addressing_value {
                    AddressingValues::Immediate(static_raw_type) => {
                        if static_raw_type.type_id.id == 1 {
                            let hash = static_raw_type.to_int();
                            if static_raw_type.to_int() as usize == target_hash {
                                let program_len_instruction = self.program.instructions[i + 1];
                                let program_len = match program_len_instruction.instruction {
                                    Instructions::STA(_) => {
                                        match program_len_instruction.addressing_value {
                                            AddressingValues::Immediate(e) => e.to_int(),
                                            _ => return Err(2),
                                        }
                                    }
                                    _ => return Err(1),
                                };
                                return Ok(MainProgram {
                                    hash: hash as usize,
                                    start: i,
                                    length: program_len as usize,
                                });
                            }
                        } else {
                            return Err(2);
                        }
                    }
                    _ => return Err(1),
                },
                _ => (),
            }
            i += 1;
        }
        Err(3)
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
