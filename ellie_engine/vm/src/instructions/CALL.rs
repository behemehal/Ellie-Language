use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::CALL,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{CallFunction, ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for CALL {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        program: StaticProgram,
        _current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Absolute(start_location) => {
                let hash = match &program[*start_location].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };
                let escape_pos = match &program[start_location + 1].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };

                Ok(ExecuterResult::CallFunction(CallFunction {
                    hash,
                    stack_len: escape_pos - start_location,
                    escape_pos,
                    pos: *start_location,
                }))
            }
            _ => {
                Err(ExecuterPanic {
                    reason: ThreadPanicReason::IllegalAddressingValue,
                    code_location: format!("{}:{}", file!(), line!()),
                })
            }
        }
    }
}
