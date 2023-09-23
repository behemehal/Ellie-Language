use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::CO,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for CO {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match addressing_value {
            AddressingValues::Absolute(pos) => stack_memory.set(
                &current_stack.get_pos(),
                StaticRawType::from_class(current_stack.calculate_frame_pos(*pos)),
            ),
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
