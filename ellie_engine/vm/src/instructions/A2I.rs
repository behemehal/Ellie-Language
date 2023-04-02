use alloc::{format, string::String};
use ellie_core::{raw_type::StaticRawType, defs::PlatformArchitecture};

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::A2I,
    stack_memory::StackMemory,
    stack::Stack,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for A2I {
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
                    1 => (),
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A =
                            StaticRawType::integer(data.to_le_bytes().to_vec());
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A =
                            StaticRawType::integer(data.to_le_bytes().to_vec());
                    }
                    4 => {
                        let data = current_stack.registers.A.to_byte();
                        current_stack.registers.A = StaticRawType::byte(data);
                    }
                    5 => {
                        let data = current_stack.registers.A.to_bool();
                        current_stack.registers.A = StaticRawType::integer(if data {
                            1_isize.to_le_bytes().to_vec()
                        } else {
                            0_isize.to_le_bytes().to_vec()
                        });
                    }
                    13 => {
                        let pointer = usize::from_le_bytes(current_stack.registers.B.data);
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
                                let integer_value = a_value.parse::<isize>().unwrap();
                                current_stack.registers.A =
                                    StaticRawType::integer(integer_value.to_le_bytes().to_vec());
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
