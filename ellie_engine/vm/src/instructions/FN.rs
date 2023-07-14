use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::FN,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for FN {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Immediate(e) => {
                let hash: usize = e.to_int() as usize;
                stack_memory.set(&current_stack.get_pos(), StaticRawType::from_function(hash));
                let end_point = match &program[current_stack.pos + 1].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };
                let parameter_count = match &program[current_stack.pos + 2].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };
                if hash != current_stack.id {
                    //Reduce by one to pass the stack_pos increase in thread
                    current_stack.pos = end_point;
                } else {
                    //skip the function len, the parameter count and paramaters
                    if parameter_count > 0 {
                        let previous_frame_pos = current_stack.frame_pos - current_stack.stack_len;
                        if !current_stack.registers.X.type_id.is_int() {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::IllegalAddressingValue,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                        let index_start =
                            current_stack.registers.X.to_int() as usize + previous_frame_pos;
                        for i in 0..parameter_count {
                            let pos = current_stack.get_pos() + 3 + i;
                            match stack_memory.get(&(index_start + i)) {
                                Some(e) => {
                                    if e.type_id.is_void() {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::NullReference(
                                                index_start + i,
                                            ),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    } else if e.type_id.is_heap_reference() {
                                        match heap_memory.get(&e.to_uint()) {
                                            Some(e) => {
                                                heap_memory.set(&pos, e.clone());
                                                stack_memory.set(
                                                    &pos,
                                                    StaticRawType::from_heap_reference(pos),
                                                );
                                            }
                                            None => {
                                                return Err(ExecuterPanic {
                                                    reason: ThreadPanicReason::NullReference(
                                                        index_start + i,
                                                    ),
                                                    code_location: format!(
                                                        "{}:{}",
                                                        file!(),
                                                        line!()
                                                    ),
                                                })
                                            }
                                        }
                                    } else {
                                        stack_memory.set(&pos, e);
                                    }
                                }
                                None => {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::NullReference(
                                            index_start + i,
                                        ),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    })
                                }
                            }
                        }
                    }
                    current_stack.pos = current_stack.pos + 2 + parameter_count;
                }
            }
            _ => {
                return Err(ExecuterPanic {
                    reason: ThreadPanicReason::IllegalAddressingValue,
                    code_location: format!("{}:{}", file!(), line!()),
                })
            }
        };
        Ok(ExecuterResult::Continue)
    }
}
