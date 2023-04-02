use alloc::{format, vec::Vec};
use ellie_core::{
    defs::PlatformArchitecture,
    raw_type::{RawType, StaticRawType, TypeId},
};

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::STR,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for STR {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match addressing_value {
            AddressingValues::Implicit => {
                heap_memory.set(
                    &current_stack.get_pos(),
                    RawType {
                        type_id: TypeId::string(0),
                        data: Vec::new(),
                    },
                );
                stack_memory.set(
                    &current_stack.get_pos(),
                    StaticRawType::heap_reference(current_stack.get_pos().to_le_bytes()),
                );
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
