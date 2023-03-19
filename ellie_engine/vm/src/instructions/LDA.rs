use alloc::format;
use ellie_core::raw_type::{StaticRawType, TypeId};

use crate::{
    channel::ModuleManager,
    config::PROGRAM_MAX_SIZE,
    heap_memory::HeapMemory,
    instruction_utils::LDA,
    program::ReadInstruction,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult};

impl super::InstructionExecuter for LDA {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: &[ReadInstruction; PROGRAM_MAX_SIZE],
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        _module_manager: &ModuleManager,
        addressing_value: &AddressingValues,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Immediate(raw_type) => {
                current_stack.registers.A = match raw_type.type_id.id {
                    0..=5 | 7 | 8 | 10 | 13 => *raw_type,
                    id => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::ImmediateUseViolation(id),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                };
            }
            AddressingValues::Absolute(e) => {
                current_stack.registers.A = match stack_memory.get(&(e + current_stack.frame_pos)) {
                    Some(raw_type) => match raw_type.type_id.id {
                        0..=5 | 7 | 10 | 13 => raw_type,
                        8 => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::NullReference(
                                    e + current_stack.frame_pos,
                                ),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                        _ => StaticRawType {
                            type_id: TypeId {
                                id: 13,
                                // ! TODO: Platform size should be used here
                                size: 8,
                            },
                            data: (e + current_stack.frame_pos).to_le_bytes(),
                        },
                    },
                    None => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::MemoryAccessViolation(
                                e.clone(),
                                current_stack.frame_pos,
                            ),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                };
            }
            AddressingValues::AbsoluteIndex(_, _) => {
                todo!("Implementation is missing")
            }
            AddressingValues::IndirectB => {
                current_stack.registers.A = current_stack.registers.B;
            }
            AddressingValues::IndirectC => {
                current_stack.registers.A = current_stack.registers.C;
            }
            AddressingValues::IndirectX => {
                current_stack.registers.A = current_stack.registers.X;
            }
            AddressingValues::IndirectY => {
                current_stack.registers.A = current_stack.registers.Y;
            }
            _ => {
                return Err(ExecuterPanic {
                    reason: ThreadPanicReason::IllegalAddressingValue,
                    code_location: format!("{}:{}", file!(), line!()),
                })
            }
        }
        Ok(ExecuterResult::Continue)
    }
}
