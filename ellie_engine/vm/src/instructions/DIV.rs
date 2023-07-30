use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::DIV,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for DIV {
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
                        let result = match b_value.checked_div(c_value) {
                            Some(e) => e,
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        };
                        current_stack.registers.A = StaticRawType::from_int(result);
                    }
                    (2, 2) => {
                        let b_value = current_stack.registers.B.to_float();
                        let c_value = current_stack.registers.C.to_float();
                        let result = b_value / c_value;
                        if result.is_finite() {
                            current_stack.registers.A = StaticRawType::from_float(result);
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::FloatOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    (3, 3) => {
                        let b_value = current_stack.registers.B.to_double();
                        let c_value = current_stack.registers.C.to_double();
                        let result = b_value / c_value;
                        if result.is_finite() {
                            current_stack.registers.A = StaticRawType::from_double(result);
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::DoubleOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    // Byte + Byte
                    (4, 4) => {
                        let b_value = current_stack.registers.B.to_int();
                        let c_value = current_stack.registers.C.to_int();
                        let result = b_value / c_value;
                        if result > -128 && result < 127 {
                            current_stack.registers.A = StaticRawType::from_byte(result as u8);
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::ByteOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::UnmergebleTypes(
                                current_stack.registers.B.type_id.id,
                                current_stack.registers.C.type_id.id,
                            ),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                };
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
