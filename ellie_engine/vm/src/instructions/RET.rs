use crate::{
    heap_memory::HeapMemory,
    instruction_utils::RET,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};
use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for RET {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        _current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => Ok(ExecuterResult::DropStack),
            _ => {
                Err(ExecuterPanic {
                    reason: ThreadPanicReason::IllegalAddressingValue,
                    code_location: format!("{}:{}", file!(), line!()),
                })
            }
        }
    }
}
