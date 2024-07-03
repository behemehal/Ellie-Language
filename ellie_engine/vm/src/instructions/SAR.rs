use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::SAR,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for SAR {
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
            AddressingValues::Immediate(static_raw_type) => {
                if static_raw_type.type_id.is_static_array() {
                    stack_memory.set(
                        &current_stack.get_pos(),
                        StaticRawType::from_static_array(
                            current_stack.calculate_frame_pos(static_raw_type.to_uint()),
                        ),
                    );
                } else {
                    return Err(ExecuterPanic {
                        reason: ThreadPanicReason::UnexpectedType(static_raw_type.type_id.id),
                        code_location: format!("{}:{}", file!(), line!()),
                    });
                }
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
