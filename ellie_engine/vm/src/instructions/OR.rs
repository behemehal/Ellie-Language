use crate::{
    heap_memory::HeapMemory,
    instruction_utils::OR,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};
use alloc::format;
use ellie_core::{raw_type::StaticRawType, defs::PlatformArchitecture};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for OR {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => {
                match (
                    current_stack.registers.B.type_id.id,
                    current_stack.registers.C.type_id.id,
                ) {
                    (5, 5) => {
                        let b_value = current_stack.registers.B.to_bool();
                        let c_value = current_stack.registers.C.to_bool();
                        current_stack.registers.A = StaticRawType::bool(b_value || c_value);
                    }
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::UncomparableTypes(
                                current_stack.registers.B.type_id.id,
                                current_stack.registers.C.type_id.id,
                            ),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
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
