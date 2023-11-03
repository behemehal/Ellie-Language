use alloc::format;
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::STB,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for STB {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => {
                stack_memory.set(&current_stack.get_pos(), current_stack.registers.B);
            }
            AddressingValues::Immediate(raw_type) => {
                stack_memory.set(&current_stack.get_pos(), *raw_type);
            }
            AddressingValues::Absolute(e) => {
                stack_memory.set(&(e + current_stack.frame_pos), current_stack.registers.B);
            }
            AddressingValues::AbsoluteIndex(pointer, index) => {
                let index = match stack_memory.get(&(index + current_stack.frame_pos)) {
                    Some(stack_data) => {
                        if stack_data.type_id.is_int() {
                            let data = stack_data.to_int();
                            if data < 0 {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::CannotIndexWithNegative(data),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            } else {
                                data as usize
                            }
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::UnexpectedType(stack_data.type_id.id),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    None => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::NullReference(*index),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                };
                match stack_memory.get(&(pointer + current_stack.frame_pos)) {
                    Some(stack_data) => {
                        if stack_data.type_id.is_heap_reference() {
                            match heap_memory.get_mut(&(stack_data.to_int() as usize)) {
                                Some(heap_data) => {
                                    let type_id = heap_data.get_type_id();
                                    if type_id.is_array() {
                                        let usize_len = arch.usize_len() as usize;
                                        let type_id_len = arch.type_id_size() as usize;
                                        let entry_size = {
                                            if heap_data.data.is_empty() {
                                                0
                                            } else {
                                                usize::from_le_bytes(
                                                    heap_data.data
                                                        [type_id_len..(usize_len + type_id_len)]
                                                        .try_into()
                                                        .unwrap(),
                                                )
                                            }
                                        };

                                        let register_bytes = current_stack.registers.B.to_bytes();

                                        if entry_size != register_bytes.len() {
                                            return Err(ExecuterPanic {
                                                reason: ThreadPanicReason::WrongEntryLength(
                                                    entry_size,
                                                    register_bytes.len(),
                                                ),
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        }

                                        let array_size = {
                                            if entry_size == 0 {
                                                0
                                            } else {
                                                (heap_data.data.len() - (usize_len + (type_id_len)))
                                                    / entry_size
                                            }
                                        };

                                        if index >= array_size {
                                            return Err(ExecuterPanic {
                                                reason: ThreadPanicReason::IndexOutOfBounds(
                                                    index, array_size,
                                                ),
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        } else {
                                            let data_start_idx = usize_len + type_id_len;
                                            let index_range = {
                                                let start = (entry_size * index) + data_start_idx;
                                                start..start + entry_size
                                            };
                                            heap_data.data[index_range.clone()]
                                                .copy_from_slice(&register_bytes);
                                            return Ok(ExecuterResult::Continue);
                                        }
                                    } else {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::UnexpectedType(type_id.id),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    }
                                }
                                None => {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::NullReference(
                                            stack_data.to_int() as usize,
                                        ),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            }
                        } else if stack_data.type_id.is_static_array() {
                            let array_location = stack_data.to_uint();
                            let array_size = match stack_memory.get(&(array_location + 1)) {
                                Some(e) => e.to_uint(),
                                None => {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::NullReference(array_location),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            };

                            if index >= array_size {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::IndexOutOfBounds(index, array_size),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            } else {
                                let entry = array_location + index;
                                stack_memory.set(&(entry + 2), current_stack.registers.B);
                                return Ok(ExecuterResult::Continue);
                            }
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::UnexpectedType(stack_data.type_id.id),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    None => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::NullReference(*pointer),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                }
            }
            AddressingValues::AbsoluteProperty(pointer, index) => match stack_memory
                .get(&(pointer + current_stack.frame_pos))
            {
                Some(e) => {
                    if e.type_id.is_class() {
                        match heap_memory.get_mut(&e.to_uint()) {
                            Some(heap_data) => {
                                let type_id = heap_data.get_type_id();
                                if type_id.is_array() {
                                    let usize_len = arch.usize_len() as usize;
                                    let type_id_len = arch.type_id_size() as usize;
                                    let entry_size = {
                                        if heap_data.data.is_empty() {
                                            0
                                        } else {
                                            usize::from_le_bytes(
                                                heap_data.data
                                                    [type_id_len..(usize_len + type_id_len)]
                                                    .try_into()
                                                    .unwrap(),
                                            )
                                        }
                                    };

                                    let register_bytes = current_stack.registers.B.to_bytes();

                                    if entry_size != register_bytes.len() {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::WrongEntryLength(
                                                entry_size,
                                                register_bytes.len(),
                                            ),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    }

                                    let array_size = {
                                        if entry_size == 0 {
                                            0
                                        } else {
                                            (heap_data.data.len() - (usize_len + (type_id_len)))
                                                / entry_size
                                        }
                                    };

                                    if *index >= array_size {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::IndexOutOfBounds(
                                                *index, array_size,
                                            ),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    } else {
                                        let data_start_idx = usize_len + type_id_len;
                                        let index_range = {
                                            let start = (entry_size * index) + data_start_idx;
                                            start..start + entry_size
                                        };
                                        heap_data.data[index_range.clone()]
                                            .copy_from_slice(&register_bytes);
                                        return Ok(ExecuterResult::Continue);
                                    }
                                } else {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::UnexpectedType(type_id.id),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            }
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(e.to_int() as usize),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        }
                    } else {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::UnexpectedType(e.type_id.id),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
                    }
                }
                None => {
                    return Err(ExecuterPanic {
                        reason: ThreadPanicReason::IllegalAddressingValue,
                        code_location: format!("{}:{}", file!(), line!()),
                    })
                }
            },
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
