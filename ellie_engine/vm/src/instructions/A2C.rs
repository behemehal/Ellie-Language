use alloc::{format, string::String};
use ellie_core::{defs::PlatformArchitecture, raw_type::StaticRawType};

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::A2C,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for A2C {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match addressing_value {
            AddressingValues::Implicit => {
                match current_stack.registers.A.type_id.id {
                    7 => (),
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
                                let a_value = String::from_utf8(mref.data)
                                    .unwrap()
                                    .chars()
                                    .next()
                                    .unwrap();
                                current_stack.registers.A =
                                    StaticRawType::char((a_value as u32).to_le_bytes().to_vec());
                            }
                            e => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::CannotConvertToType(e, 7),
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