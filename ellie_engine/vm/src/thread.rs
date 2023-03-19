#![allow(non_snake_case)]
use alloc::format;
use ellie_core::{defs::PlatformArchitecture, raw_type::StaticRawType};

use crate::{
    channel::ModuleManager,
    heap_memory::HeapMemory,
    program::Program,
    stack::{Stack, StackArray},
    stack_memory::StackMemory,
    utils::{ThreadExit, ThreadInfo, ThreadPanic, ThreadPanicReason},
};

#[derive(Debug, Copy, Clone)]
pub struct Registers {
    pub A: StaticRawType,
    pub B: StaticRawType,
    pub C: StaticRawType,
    pub X: StaticRawType,
    pub Y: StaticRawType,
}

#[derive(Debug, Clone)]
pub struct Thread {
    // Thread ID
    pub id: usize,
    // Platform architecture
    pub arch: PlatformArchitecture,
    // Stack of the thread
    pub stack: StackArray,
    // Frame position of the thread changes over stack changes
    pub frame_pos: usize,
}

impl Thread {
    pub fn new(id: usize, arch: PlatformArchitecture) -> Self {
        Thread {
            id,
            arch,
            stack: StackArray::new(),
            frame_pos: 0,
        }
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
        heap_memory: &mut HeapMemory,
        stack_memory: &mut StackMemory,
        module_manager: &mut ModuleManager,
        loaded_program: &Program,
    ) -> Option<ThreadExit> {
        loop {
            if self.stack.len() == 0 {
                return Some(ThreadExit::ExitGracefully);
            }
            let current_stack = self.stack.last_mut().unwrap();
            if current_stack.pos >= loaded_program.program_len {
                return Some(ThreadExit::Panic(ThreadPanic {
                    reason: ThreadPanicReason::OutOfInstructions,
                    stack_trace: self.stack.clone(),
                    code_location: format!("{}:{}", file!(), line!()),
                }));
            }
            let current_instruction = &loaded_program.instructions[current_stack.pos];
            let execute_result = current_instruction.instruction.execute(
                heap_memory,
                &loaded_program.instructions,
                current_stack,
                stack_memory,
                module_manager,
                &current_instruction.addressing_value,
            );
            match execute_result {
                Ok(result) => match result {
                    crate::instructions::ExecuterResult::Continue => {
                        current_stack.pos += 1;
                    }
                    crate::instructions::ExecuterResult::DropStack => {
                        let current_y = current_stack.registers.Y.clone();
                        match current_stack.caller {
                            Some(_) => {
                                //let last_scope = self.stack.get(self.stack.idx - 1);
                                self.stack.pop();
                                self.stack.last_mut().unwrap().registers.Y = current_y;
                            }
                            None => {
                                self.stack.pop();
                            }
                        }
                        if self.stack.len() == 0 {
                            return Some(ThreadExit::ExitGracefully);
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
                                A: StaticRawType::void(),
                                B: StaticRawType::void(),
                                C: StaticRawType::void(),
                                X: current_x,
                                Y: StaticRawType::void(),
                            },
                            caller,
                        });
                    }
                    crate::instructions::ExecuterResult::CallNativeFunction(native_call) => {
                        match module_manager.find_module_by_item_hash(native_call.hash) {
                            Some(module) => match module.get_emiter(native_call.hash) {
                                Some(item) => match item {
                                    crate::channel::ModuleElements::Function(native_function) => {
                                        let response = (native_function.callback)(
                                            ThreadInfo {
                                                id: self.id,
                                                stack_id: current_stack.id,
                                                stack_caller: current_stack.caller,
                                            },
                                            native_call.params,
                                        );
                                        match response {
                                            ellie_core::defs::VmNativeAnswer::Ok(return_value) => {
                                                match return_value {
                                                    ellie_core::defs::VmNativeCallParameters::Static(static_value) => {
                                                        current_stack.registers.Y = static_value;
                                                    }
                                                    ellie_core::defs::VmNativeCallParameters::Dynamic(dynamic_value) => {
                                                        heap_memory.set(&native_call.return_heap_position, dynamic_value);
                                                        current_stack.registers.Y = StaticRawType::reference(native_call.return_heap_position.to_le_bytes())
                                                    }
                                                }
                                                current_stack.pos += 1;
                                            }
                                            ellie_core::defs::VmNativeAnswer::RuntimeError(e) => {
                                                return Some(ThreadExit::Panic(ThreadPanic {
                                                    reason: ThreadPanicReason::RuntimeError(e),
                                                    stack_trace: self.stack.clone(),
                                                    code_location: format!(
                                                        "{}:{}",
                                                        file!(),
                                                        line!()
                                                    ),
                                                }))
                                            }
                                        }
                                    }
                                },
                                None => {
                                    return Some(ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::CallToUnknown(native_call.hash),
                                        stack_trace: self.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    }));
                                }
                            },
                            None => {
                                return Some(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MissingModule,
                                    stack_trace: self.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                    }
                },
                Err(panic) => {
                    return Some(ThreadExit::Panic(ThreadPanic {
                        reason: panic.reason,
                        stack_trace: self.stack.clone(),
                        code_location: panic.code_location,
                    }))
                }
            }
        }
    }
}
