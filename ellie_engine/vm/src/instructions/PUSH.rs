use crate::{
    heap_memory::HeapMemory,
    instruction_utils::PUSH,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};
use alloc::{format, vec};
use ellie_core::defs::PlatformArchitecture;

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for PUSH {
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
                match stack_memory.get(&(current_stack.get_pos() - 1)) {
                    Some(stack_data) => match heap_memory.get_mut(&(absolute_address + current_stack.frame_pos)) {
                        Some(mut heap_value) => {
                            let mut type_id = heap_value.get_type_id();
                            if type_id.id != 9 {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::InvalidRegisterAccess(type_id.id),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }

                            let data = stack_data.to_bytes();
                            let entry_len = {
                                if heap_value.data.len() == (arch.usize_len() as usize + 1) {
                                    //platform safety:
                                    //If current platform is 64 bit, but targeted arch is 64
                                    //we can use 64 bit to store the length of the array
                                    //----------------
                                    //If current platform is 64 bit, but targeted arch is 32
                                    //we can use 4 bytes of the usize's 64bit 8 bytes to store the length of the array
                                    //----------------
                                    //If current platform is 64 bit, but targeted arch is 16
                                    //we can use 2 bytes of the usize's 64bit 8 bytes to store the length of the array
                                    let len = data.len().to_le_bytes().to_vec();
                                    heap_value.data.extend(match arch {
                                        ellie_core::defs::PlatformArchitecture::B16 => {
                                            vec![len[0], len[1]]
                                        }
                                        ellie_core::defs::PlatformArchitecture::B32 => {
                                            vec![len[0], len[1], len[2], len[3]]
                                        }
                                        ellie_core::defs::PlatformArchitecture::B64 => len,
                                    });
                                    data.len()
                                } else {
                                    let array_len_range = {
                                        let type_id_size = arch.type_id_size() as usize;
                                        let usize_len = arch.usize_len() as usize;
                                        type_id_size..(type_id_size + usize_len)
                                    };
                                    usize::from_le_bytes(
                                        heap_value.data[array_len_range].try_into().unwrap(),
                                    )
                                }
                            };

                            if entry_len != data.len() {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::WrongEntryLength(
                                        entry_len,
                                        data.len(),
                                    ),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }

                            type_id.size += heap_value.data.len() + data.len();
                            heap_value.set_type_id(type_id);
                            heap_value.data.extend(data);
                        }
                        None => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::NullReference(*absolute_address),
                                code_location: format!("{}:{}", file!(), line!()),
                            })
                        }
                    },
                    None => todo!(),
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
