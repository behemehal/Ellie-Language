#![allow(non_snake_case)]
use alloc::{format, string::String};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    channel::ModuleManager,
    heap_memory::HeapMemory,
    iternal_functions::INTERNAL_FUNCTIONS,
    program::{MainProgram, VmProgram},
    raw_type::StaticRawType,
    stack::{Stack, StackArray},
    stack_memory::StackMemory,
    utils::{
        ThreadExit, ThreadInfo, ThreadPanic, ThreadPanicReason, VmNativeAnswer,
        VmNativeCallParameters,
    },
};

#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub A: StaticRawType,
    pub B: StaticRawType,
    pub C: StaticRawType,
    pub X: StaticRawType,
    pub Y: StaticRawType,
}

pub struct Isolate {
    pub heap_memory: HeapMemory,
    pub stack_memory: StackMemory,
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

pub struct Thread {
    // Thread ID
    pub id: usize,
    // Platform architecture
    pub arch: PlatformArchitecture,
    // Stack of the thread
    pub stack: StackArray,
    // Frame position of the thread changes over stack changes
    pub frame_pos: usize,
    pub isolate: Isolate,
}

impl Thread {
    pub fn new(id: usize, arch: PlatformArchitecture, isolate: Isolate) -> Self {
        Thread {
            id,
            arch,
            stack: StackArray::new(),
            frame_pos: 0,
            isolate,
        }
    }

    pub fn build_thread(&mut self, main: MainProgram) {
        self.stack.push(Stack {
            id: main.hash,
            registers: Registers {
                A: StaticRawType::from_void(),
                B: StaticRawType::from_void(),
                C: StaticRawType::from_void(),
                X: StaticRawType::from_void(),
                Y: StaticRawType::from_void(),
            },
            stack_len: main.length,
            caller: None,
            pos: main.start,
            frame_pos: self.frame_pos,
        });
    }

    pub fn call(&mut self) {
        todo!()
    }

    pub fn step(
        &mut self,
        _heap_memory: &mut HeapMemory,
        _stack_memory: &mut StackMemory,
    ) -> Option<ThreadExit> {
        //let current_instruction = &self.program[self.stack_pos];
        /* let execute_output =
        &current_instruction
            .instruction
            .execute(&mut self, heap_memory, stack_memory); */

        todo!()
    }

    pub fn run(
        &mut self,
        module_manager: &mut ModuleManager,
        loaded_program: &VmProgram,
    ) -> ThreadExit {
        loop {
            if self.stack.len() == 0 {
                return ThreadExit::ExitGracefully;
            }
            let current_stack = self.stack.last_mut().unwrap();
            if current_stack.pos > loaded_program.length {
                return ThreadExit::Panic(ThreadPanic {
                    reason: ThreadPanicReason::OutOfInstructions,
                    stack_trace: self.stack.clone(),
                    code_location: format!("{}:{}", file!(), line!()),
                });
            }
            let current_instruction = &loaded_program.instructions[current_stack.pos];
            let execute_result = current_instruction.instruction.execute(
                &mut self.isolate.heap_memory,
                &loaded_program.instructions,
                current_stack,
                &mut self.isolate.stack_memory,
                &current_instruction.addressing_value,
                self.arch,
            );
            match execute_result {
                Ok(result) => match result {
                    crate::instructions::ExecuterResult::Continue => {
                        current_stack.pos += 1;
                    }
                    crate::instructions::ExecuterResult::DropStack => {
                        let current_y = current_stack.registers.Y;
                        match current_stack.caller {
                            Some(_) => {
                                self.stack.pop();
                                self.stack.last_mut().unwrap().registers.Y = current_y;
                            }
                            None => {
                                self.stack.pop();
                            }
                        }
                        if self.stack.len() == 0 {
                            return ThreadExit::ExitGracefully;
                        }
                    }
                    crate::instructions::ExecuterResult::CallFunction(e) => {
                        let caller = Some(current_stack.id);
                        let current_x = current_stack.registers.X;
                        let frame_pos = current_stack.frame_pos + e.stack_len;
                        current_stack.pos += 1;
                        self.stack.push(Stack {
                            pos: e.pos,
                            frame_pos,
                            id: e.hash,
                            stack_len: e.stack_len,
                            registers: Registers {
                                A: StaticRawType::from_void(),
                                B: StaticRawType::from_void(),
                                C: StaticRawType::from_void(),
                                X: current_x,
                                Y: StaticRawType::from_void(),
                            },
                            caller,
                        });
                    }
                    crate::instructions::ExecuterResult::CallNativeFunction(native_call) => {
                        let found_trace = loaded_program
                            .traces
                            .iter()
                            .find(|x| x.function_hash == native_call.hash);
                        match found_trace {
                            Some(found_trace) => {
                                if let Some(internal_function) = INTERNAL_FUNCTIONS
                                    .iter()
                                    .find(|x| x.name == found_trace.function_name)
                                {
                                    let response = (internal_function.callback)(
                                        &mut self.isolate,
                                        native_call.params,
                                    );
                                    match response {
                                        VmNativeAnswer::Ok(return_value) => {
                                            match return_value {
                                                VmNativeCallParameters::Static(static_value) => {
                                                    current_stack.registers.Y = static_value;
                                                }
                                                VmNativeCallParameters::Dynamic(dynamic_value) => {
                                                    self.isolate.heap_memory.set(
                                                        &native_call.return_heap_position,
                                                        dynamic_value,
                                                    );
                                                    current_stack.registers.Y =
                                                        StaticRawType::from_heap_reference(
                                                            native_call.return_heap_position,
                                                        )
                                                }
                                            }
                                            current_stack.pos += 1;
                                        }
                                        VmNativeAnswer::RuntimeError(e) => {
                                            return ThreadExit::Panic(ThreadPanic {
                                                reason: ThreadPanicReason::RuntimeError(e),
                                                stack_trace: self.stack.clone(),
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        }
                                    }
                                } else {
                                    match module_manager
                                        .find_module_by_item_name(&found_trace.function_name)
                                    {
                                        Some(module) => {
                                            match module
                                                .get_emiter_by_name(&found_trace.function_name)
                                            {
                                                Some(item) => match item {
                                                    crate::channel::ModuleElements::Function(
                                                        native_function,
                                                    ) => {
                                                        let response = (native_function.callback)(
                                                            ThreadInfo {
                                                                id: self.id,
                                                                stack_id: current_stack.id,
                                                                stack_caller: current_stack.caller,
                                                            },
                                                            native_call.params,
                                                        );
                                                        match response {
                                                            VmNativeAnswer::Ok(return_value) => {
                                                                match return_value {
                                                                VmNativeCallParameters::Static(
                                                                    static_value,
                                                                ) => {
                                                                    current_stack.registers.Y =
                                                                        static_value;
                                                                }
                                                                VmNativeCallParameters::Dynamic(
                                                                    dynamic_value,
                                                                ) => {
                                                                    self.isolate.heap_memory.set(
                                                                        &native_call
                                                                            .return_heap_position,
                                                                        dynamic_value,
                                                                    );
                                                                    current_stack.registers.Y =
                                                                    StaticRawType::from_heap_reference(
                                                                        native_call.return_heap_position,
                                                                    )
                                                                }
                                                            }
                                                                current_stack.pos += 1;
                                                            }
                                                            VmNativeAnswer::RuntimeError(e) => {
                                                                return ThreadExit::Panic(ThreadPanic {
                                                            reason: ThreadPanicReason::RuntimeError(e),
                                                            stack_trace: self.stack.clone(),
                                                            code_location: format!(
                                                                "{}:{}",
                                                                file!(),
                                                                line!()
                                                            ),
                                                        });
                                                            }
                                                        }
                                                    }
                                                },
                                                None => {
                                                    return ThreadExit::Panic(ThreadPanic {
                                                        reason: ThreadPanicReason::CallToUnknown((
                                                            found_trace.function_name.clone(),
                                                            native_call.hash,
                                                        )),
                                                        stack_trace: self.stack.clone(),
                                                        code_location: format!(
                                                            "{}:{}",
                                                            file!(),
                                                            line!()
                                                        ),
                                                    });
                                                }
                                            }
                                        }
                                        None => {
                                            return ThreadExit::Panic(ThreadPanic {
                                                reason: ThreadPanicReason::MissingModule(
                                                    native_call.hash,
                                                ),
                                                stack_trace: self.stack.clone(),
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        }
                                    }
                                }
                            }
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MissingTrace(native_call.hash),
                                    stack_trace: self.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                })
                            }
                        }
                    }
                },
                Err(panic) => {
                    return ThreadExit::Panic(ThreadPanic {
                        reason: panic.reason,
                        stack_trace: self.stack.clone(),
                        code_location: panic.code_location,
                    });
                }
            }
        }
    }
}
