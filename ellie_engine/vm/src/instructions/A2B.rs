use alloc::format;
use ellie_core::{defs::PlatformArchitecture, raw_type::StaticRawType};

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::A2B,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for A2B {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match addressing_value {
            AddressingValues::Implicit => {
                match current_stack.registers.A.type_id.id {
                    1 => {
                        let data = current_stack.registers.A.to_int();
                        if data < 255 {
                            current_stack.registers.A = StaticRawType::byte(data as u8);
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A =
                            StaticRawType::byte(if data.is_sign_negative() {
                                data.to_le_bytes()[0]
                            } else {
                                data.to_le_bytes()[0]
                            });
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A =
                            StaticRawType::byte(if data.is_sign_negative() {
                                data.to_le_bytes()[0]
                            } else {
                                data.to_le_bytes()[0]
                            });
                    }
                    4 => (),
                    5 => {
                        let data = current_stack.registers.A.to_bool();
                        current_stack.registers.A = StaticRawType::byte(if data { 1 } else { 0 });
                    }
                    e => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::CannotConvertToType(e, 7),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                };
                Ok(ExecuterResult::Continue)
            }
            _ => Err(ExecuterPanic {
                reason: ThreadPanicReason::IllegalAddressingValue,
                code_location: format!("{}:{}", file!(), line!()),
            }),
        }
    }
}
