use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::SPUS,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for SPUS {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match addressing_value {
            AddressingValues::Absolute(absolute_address) => {
                match heap_memory.get_mut(absolute_address) {
                    Some(mut heap_value) => {
                        let mut type_id = heap_value.get_type_id();
                        if type_id.id != 6 {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::InvalidRegisterAccess(type_id.id),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                        type_id.size += 4;
                        heap_value.set_type_id(type_id);
                        let char_data = match stack_memory.get(&(current_stack.get_pos() - 1)) {
                            Some(e) => {
                                if e.type_id.is_char() {
                                    e.data.clone()
                                } else {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::InvalidType(7),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            }
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(
                                        current_stack.get_pos() - 1,
                                    ),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        };
                        heap_value.data.extend(char_data[0..4].to_vec());
                    }
                    None => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::NullReference(*absolute_address),
                            code_location: format!("{}:{}", file!(), line!()),
                        })
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
