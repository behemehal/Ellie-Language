use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::DEA,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for DEA {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Absolute(location) => match stack_memory.get(&location) {
                Some(e) => {
                    if e.type_id.is_heap_reference() {
                        heap_memory.dea(&(location + current_stack.frame_pos));
                    }
                    stack_memory.dea(&(location + current_stack.frame_pos));
                    Ok(ExecuterResult::Continue)
                }
                None => {
                    return Err(ExecuterPanic {
                        reason: ThreadPanicReason::NullReference(*location),
                        code_location: format!("{}:{}", file!(), line!()),
                    })
                }
            },
            _ => {
                return Err(ExecuterPanic {
                    reason: ThreadPanicReason::IllegalAddressingValue,
                    code_location: format!("{}:{}", file!(), line!()),
                })
            }
        }
    }
}
