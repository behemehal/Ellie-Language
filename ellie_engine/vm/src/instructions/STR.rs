use alloc::{format, vec::Vec};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::STR,
    raw_type::{RawType, StaticRawType, TypeId},
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
                    StaticRawType::from_heap_reference(current_stack.get_pos()),
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
