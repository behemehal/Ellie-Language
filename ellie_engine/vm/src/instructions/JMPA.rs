use crate::{
    heap_memory::HeapMemory,
    instruction_utils::JMPA,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};
use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for JMPA {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match addressing_value {
            AddressingValues::Absolute(e) => {
                if current_stack.registers.A.type_id.is_bool() {
                    if current_stack.registers.A.data[0] == 1 {
                        current_stack.pos = e - 1;
                    }
                    Ok(ExecuterResult::Continue)
                } else {
                    return Err(ExecuterPanic {
                        reason: ThreadPanicReason::UnexpectedType(
                            current_stack.registers.A.type_id.id,
                        ),
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
    }
}
