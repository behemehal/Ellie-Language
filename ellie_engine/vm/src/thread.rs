#![allow(non_snake_case)]
use core::sync::atomic::{AtomicBool, Ordering};

use alloc::{boxed::Box, format, string::String};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    channel::ModuleManager,
    heap_memory::HeapMemory,
    isolate::Isolate,
    iternal_functions::INTERNAL_FUNCTIONS,
    program::{MainProgram, VmProgram},
    raw_type::{RawType, StaticRawType},
    stack::{Caller, Stack, StackArray},
    stack_memory::StackMemory,
    utils::{
        ellie_data_to_static_raw_type, EllieData, RawFunctionData, StepResult, ThreadExit,
        ThreadInfo, ThreadPanic, ThreadPanicReason, VmNativeAnswer,
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

pub struct Thread {
    // Thread ID
    pub id: usize,
    // Platform architecture
    pub arch: PlatformArchitecture,
    // Stack of the thread
    pub stack: StackArray,
    pub isolate: Isolate,
}

impl Thread {
    pub fn new(id: usize, arch: PlatformArchitecture, isolate: Isolate) -> Self {
        Thread {
            id,
            arch,
            stack: StackArray::new(),
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
            frame_pos: main.start + main.length,
        });
    }

    pub fn call(&mut self) {
        todo!()
    }

    pub fn step(
        &mut self,
        module_manager: &mut ModuleManager,
        loaded_program: &VmProgram,
    ) -> StepResult {
        if self.stack.len() == 0 {
            return StepResult::ThreadExit(ThreadExit::ExitGracefully);
        }

        let current_stack = self.stack.last_mut().unwrap();
        if current_stack.pos > loaded_program.length {
            return StepResult::ThreadExit(ThreadExit::Panic(ThreadPanic {
                reason: ThreadPanicReason::OutOfInstructions,
                stack_trace: self.stack.clone(),
                code_location: format!("{}:{}", file!(), line!()),
            }));
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
                    StepResult::Step
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
                        return StepResult::ThreadExit(ThreadExit::ExitGracefully);
                    }
                    StepResult::Step
                }
                crate::instructions::ExecuterResult::CallFunction(e) => {
                    let caller = Some(Caller {
                        id: current_stack.id,
                        frame_pos: current_stack.frame_pos,
                    });
                    let current_x = current_stack.registers.X;
                    let frame_pos = current_stack.get_pos() + e.stack_len;
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
                    StepResult::Step
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
                                    ThreadInfo {
                                        id: self.id,
                                        stack_id: current_stack.id,
                                        frame_pos: current_stack.frame_pos,
                                        pos: current_stack.pos,
                                        stack_caller: current_stack.caller.map(|c| c.id),
                                        arch: self.arch,
                                    },
                                    native_call.params,
                                );
                                match response {
                                    VmNativeAnswer::Ok(return_value) => {
                                        match return_value {
                                            EllieData::String(string) => {
                                                self.isolate.heap_memory.set(
                                                    &native_call.return_heap_position,
                                                    RawType::generate_string(string),
                                                );
                                                current_stack.registers.Y =
                                                    StaticRawType::from_heap_reference(
                                                        native_call.return_heap_position,
                                                    );
                                            }
                                            EllieData::Array(vec) => {
                                                todo!("Array return not yet implemented")
                                            }
                                            EllieData::Class(vec) => {
                                                todo!("Class return not yet implemented")
                                            }
                                            value => {
                                                current_stack.registers.Y =
                                                    ellie_data_to_static_raw_type(value);
                                            }
                                        }

                                        current_stack.pos += 1;
                                        StepResult::Step
                                    }
                                    VmNativeAnswer::RuntimeError(e) => {
                                        StepResult::ThreadExit(ThreadExit::Panic(ThreadPanic {
                                            reason: ThreadPanicReason::RuntimeError(e),
                                            stack_trace: self.stack.clone(),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        }))
                                    }
                                }
                            } else {
                                match module_manager
                                    .find_module_by_item_name(&found_trace.function_name)
                                {
                                    Some(module) => {
                                        match module.get_emiter_by_name(&found_trace.function_name)
                                        {
                                            Some(item) => match item {
                                                crate::channel::ModuleElements::Function(
                                                    native_function,
                                                ) => {
                                                    let response = (native_function.callback)(
                                                        ThreadInfo {
                                                            id: self.id,
                                                            stack_id: current_stack.id,
                                                            frame_pos: current_stack.frame_pos,
                                                            pos: current_stack.pos,
                                                            stack_caller: current_stack
                                                                .caller
                                                                .map(|c| c.id),
                                                            arch: self.arch,
                                                        },
                                                        native_call.params,
                                                    );
                                                    match response {
                                                        VmNativeAnswer::Ok(return_value) => {
                                                            match return_value {
                                                                EllieData::String(string) => {
                                                                    self.isolate.heap_memory.set(
                                                                        &native_call
                                                                            .return_heap_position,
                                                                            RawType::generate_string(string),
                                                                    );
                                                                    current_stack.registers.Y =
                                                                StaticRawType::from_heap_reference(
                                                                    native_call.return_heap_position,
                                                                );
                                                                },
                                                                EllieData::Array(vec) => todo!("Array return not yet implemented"),
                                                                EllieData::Class(vec) => todo!("Class return not yet implemented"),
                                                                value => {
                                                                    current_stack.registers.Y = ellie_data_to_static_raw_type(value);
                                                                },
                                                            };

                                                            current_stack.pos += 1;
                                                            StepResult::Step
                                                        }
                                                        VmNativeAnswer::RuntimeError(e) => {
                                                            StepResult::ThreadExit(ThreadExit::Panic(ThreadPanic {
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
                                            None => StepResult::ThreadExit(ThreadExit::Panic(
                                                ThreadPanic {
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
                                                },
                                            )),
                                        }
                                    }
                                    None => {
                                        StepResult::ThreadExit(ThreadExit::Panic(ThreadPanic {
                                            reason: ThreadPanicReason::MissingModule(
                                                native_call.hash,
                                            ),
                                            stack_trace: self.stack.clone(),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        }))
                                    }
                                }
                            }
                        }
                        None => StepResult::ThreadExit(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::MissingTrace(native_call.hash),
                            stack_trace: self.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        })),
                    }
                }
            },
            Err(panic) => StepResult::ThreadExit(ThreadExit::Panic(ThreadPanic {
                reason: panic.reason,
                stack_trace: self.stack.clone(),
                code_location: panic.code_location,
            })),
        }
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

            static STACK_OVERFLOW: AtomicBool = AtomicBool::new(false);
            static HEAP_OUT_OF_MEMORY: AtomicBool = AtomicBool::new(false);

            self.isolate
                .heap_memory
                .set_on_heap_out_of_memory(Box::new(|| {
                    HEAP_OUT_OF_MEMORY.store(true, Ordering::Relaxed);
                }));

            /* self.isolate
            .stack_memory
            .set_on_stack_overflow(Box::new(|| {
                STACK_OVERFLOW.store(true, Ordering::Relaxed);
            })); */

            if STACK_OVERFLOW.load(Ordering::SeqCst) {
                return ThreadExit::Panic(ThreadPanic {
                    reason: ThreadPanicReason::StackOverflow,
                    stack_trace: self.stack.clone(),
                    code_location: format!("{}:{}", file!(), line!()),
                });
            }

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
                        let caller = Some(Caller {
                            id: current_stack.id,
                            frame_pos: current_stack.frame_pos,
                        });
                        let current_x = current_stack.registers.X;
                        let frame_pos = current_stack.get_pos() + e.stack_len;
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
                                        ThreadInfo {
                                            id: self.id,
                                            stack_id: current_stack.id,
                                            frame_pos: current_stack.frame_pos,
                                            pos: current_stack.pos,
                                            stack_caller: current_stack.caller.map(|c| c.id),
                                            arch: self.arch,
                                        },
                                        native_call.params,
                                    );

                                    match response {
                                        VmNativeAnswer::Ok(return_value) => {
                                            match return_value {
                                                EllieData::String(string) => {
                                                    self.isolate.heap_memory.set(
                                                        &native_call.return_heap_position,
                                                        RawType::generate_string(string),
                                                    );
                                                    current_stack.registers.Y =
                                                        StaticRawType::from_heap_reference(
                                                            native_call.return_heap_position,
                                                        );
                                                }
                                                EllieData::Array(vec) => {
                                                    todo!("Array return not yet implemented")
                                                }
                                                EllieData::Class(vec) => {
                                                    todo!("Class return not yet implemented")
                                                }
                                                value => {
                                                    current_stack.registers.Y =
                                                        ellie_data_to_static_raw_type(value);
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
                                                                frame_pos: current_stack.frame_pos,
                                                                pos: current_stack.pos,
                                                                stack_caller: current_stack
                                                                    .caller
                                                                    .map(|c| c.id),
                                                                arch: self.arch,
                                                            },
                                                            native_call.params,
                                                        );
                                                        match response {
                                                            VmNativeAnswer::Ok(return_value) => {
                                                                match return_value {
                                                                    EllieData::String(string) => {
                                                                        self.isolate.heap_memory.set(
                                                                            &native_call
                                                                                .return_heap_position,
                                                                            RawType::generate_string(string),
                                                                        );
                                                                        current_stack.registers.Y =
                                                                    StaticRawType::from_heap_reference(
                                                                        native_call.return_heap_position,
                                                                    )
                                                                    }
                                                                    EllieData::Array(vec) => {
                                                                        todo!("Array return not yet implemented")
                                                                    }
                                                                    EllieData::Class(vec) => {
                                                                        todo!("Class return not yet implemented")
                                                                    }
                                                                    value => {
                                                                        current_stack.registers.Y =
                                                                    ellie_data_to_static_raw_type(value)
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
