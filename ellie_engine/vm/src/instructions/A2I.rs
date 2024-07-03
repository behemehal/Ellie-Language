use alloc::{format, string::String};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::A2I,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
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
                        let data = current_stack.registers.A.to_float() as isize;
                        current_stack.registers.A = StaticRawType::from_int(data);
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double() as isize;
                        current_stack.registers.A = StaticRawType::from_int(data);
                    }
                    4 => {
                        let data = current_stack.registers.A.to_byte() as isize;
                        current_stack.registers.A = StaticRawType::from_int(data);
                    }
                    5 => {
                        let data = current_stack.registers.A.to_bool();
                        current_stack.registers.A =
                            StaticRawType::from_int(if data { 1_isize } else { 0_isize });
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
                                let integer_value = a_value.parse::<isize>().unwrap();
                                current_stack.registers.A = StaticRawType::from_int(integer_value);
                            }
                            e => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::CannotConvertToType(e, 3),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        }
                    }
                    15 => {
                        let data = current_stack.registers.A.to_uint() as isize;
                        current_stack.registers.A = StaticRawType::from_int(data);
                    }
                    e => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::CannotConvertToType(e, 1),
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
