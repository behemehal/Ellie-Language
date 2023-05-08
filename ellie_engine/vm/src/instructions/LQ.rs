use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::LQ,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason}, raw_type::StaticRawType,
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for LQ {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => {
                match (
                    current_stack.registers.B.type_id.id,
                    current_stack.registers.C.type_id.id,
                ) {
                    (1, 1) => {
                        let b_value = current_stack.registers.B.to_int();
                        let c_value = current_stack.registers.C.to_int();
                        current_stack.registers.A = StaticRawType::from_bool(b_value <= c_value);
                    }
                    (2, 2) => {
                        let b_value = current_stack.registers.B.to_float();
                        let c_value = current_stack.registers.C.to_float();
                        current_stack.registers.A = StaticRawType::from_bool(b_value <= c_value);
                    }
                    (3, 3) => {
                        let b_value = current_stack.registers.B.to_double();
                        let c_value = current_stack.registers.C.to_double();
                        current_stack.registers.A = StaticRawType::from_bool(b_value <= c_value);
                    }
                    (2, 3) | (3, 2) => {
                        let b_value = current_stack.registers.B.to_double();
                        let c_value = current_stack.registers.C.to_double();
                        current_stack.registers.A = StaticRawType::from_bool(b_value <= c_value);
                    }
                    (4, 4) => {
                        let b_value = current_stack.registers.B.to_byte();
                        let c_value = current_stack.registers.C.to_byte();
                        current_stack.registers.A = StaticRawType::from_bool(b_value <= c_value);
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
