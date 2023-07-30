use alloc::{format, string::String};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::A2D,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for A2D {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
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
                        current_stack.registers.A = StaticRawType::from_double(data as f32);
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A = StaticRawType::from_double(data as f32);
                    }
                    3 => (),
                    4 => {
                        let data = current_stack.registers.A.to_byte();
                        current_stack.registers.A = StaticRawType::from_double(data as f32);
                    }
                    13 => {
                        let pointer = current_stack.registers.B.to_int() as usize;
                        let mref = match heap_memory.get(&pointer) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(pointer),
                                    code_location: format!("{}:{}", file!(), line!()),
                                })
                            }
                        };
                        match mref.type_id.id {
                            6 => {
                                let a_value = String::from_utf8(mref.data).unwrap();
                                match a_value.parse::<f32>() {
                                    Ok(double_value) => {
                                        current_stack.registers.A =
                                            StaticRawType::from_double(double_value);
                                    }
                                    Err(_) => {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::CannotConvertToType(
                                                mref.type_id.id,
                                                3,
                                            ),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        })
                                    }
                                }
                            }
                            e => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::CannotConvertToType(e, 3),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        }
                    }
                    e => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::CannotConvertToType(e, 3),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                }
                Ok(ExecuterResult::Continue)
            }
            _ => Err(ExecuterPanic {
                reason: ThreadPanicReason::IllegalAddressingValue,
                code_location: format!("{}:{}", file!(), line!()),
            }),
        }
    }
}
