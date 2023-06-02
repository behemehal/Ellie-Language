use super::{ExecuterPanic, ExecuterResult, StaticProgram};
use crate::{
    heap_memory::HeapMemory,
    instruction_utils::LDX,
    raw_type::StaticRawType,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};
use alloc::{format, vec::Vec};
use ellie_core::defs::PlatformArchitecture;

impl super::InstructionExecuter for LDX {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Immediate(raw_type) => {
                current_stack.registers.X = if raw_type.type_id.is_stack_storable() {
                    *raw_type
                } else {
                    return Err(ExecuterPanic {
                        reason: ThreadPanicReason::ImmediateUseViolation(raw_type.type_id.id),
                        code_location: format!("{}:{}", file!(), line!()),
                    });
                }
            }
            AddressingValues::Absolute(e) => {
                current_stack.registers.X =
                    match stack_memory.get(&(current_stack.calculate_frame_pos(*e))) {
                        Some(raw_type) => {
                            if raw_type.type_id.is_void() {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(
                                        current_stack.calculate_frame_pos(*e),
                                    ),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            } else if raw_type.type_id.is_stack_storable() {
                                raw_type
                            } else {
                                StaticRawType::from_heap_reference(
                                    current_stack.calculate_frame_pos(*e),
                                )
                            }
                        }
                        None => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::MemoryAccessViolation(
                                    e.clone(),
                                    current_stack.pos,
                                ),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    };
            }
            AddressingValues::AbsoluteIndex(pointer, index) => {
                let index = match stack_memory.get(&current_stack.calculate_frame_pos(*index)) {
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
                match stack_memory.get(&(&current_stack.calculate_frame_pos(*pointer))) {
                    Some(stack_data) => {
                        if stack_data.type_id.is_heap_reference() {
                            match heap_memory.get(&(stack_data.to_uint())) {
                                Some(heap_data) => {
                                    if heap_data.type_id.is_array() {
                                        let array_entry_size = usize::from_le_bytes(
                                            heap_data.data[..arch.usize_len() as usize]
                                                .try_into()
                                                .unwrap(),
                                        );
                                        let array_data =
                                            &heap_data.data[arch.usize_len() as usize..];
                                        let array_entries =
                                            array_data.chunks(array_entry_size).collect::<Vec<_>>();
                                        if index > array_entries.len() {
                                            return Err(ExecuterPanic {
                                                reason: ThreadPanicReason::IndexOutOfBounds(index),
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        } else {
                                            let array_entry = array_entries[index];
                                            current_stack.registers.X =
                                                StaticRawType::from_bytes(array_entry);
                                            return Ok(ExecuterResult::Continue);
                                        }
                                    } else {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::UnexpectedType(
                                                heap_data.type_id.id,
                                            ),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    }
                                }
                                None => {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::NullReference(
                                            stack_data.to_uint(),
                                        ),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
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
                .get(&(current_stack.calculate_frame_pos(*pointer)))
            {
                Some(static_raw_type) => {
                    if static_raw_type.type_id.is_class()
                        || static_raw_type.type_id.is_heap_reference()
                    {
                        match heap_memory.get(&static_raw_type.to_uint()) {
                            Some(raw_type) => {
                                if raw_type.type_id.is_array() {
                                    // Increase size of array
                                    let array_size = raw_type.type_id.size;
                                    if *index < array_size && *index > array_size {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::IndexOutOfBounds(*index),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    } else {
                                        let platform_size = arch.usize_len() as usize;
                                        let array_entry_len = {
                                            if raw_type.data.len() < (platform_size + 1) {
                                                return Err(ExecuterPanic {
                                                    reason: ThreadPanicReason::ArraySizeCorruption,
                                                    code_location: format!(
                                                        "{}:{}",
                                                        file!(),
                                                        line!()
                                                    ),
                                                });
                                            } else {
                                                usize::from_le_bytes(
                                                    raw_type.data[0..platform_size]
                                                        .try_into()
                                                        .unwrap(),
                                                )
                                            }
                                        };

                                        if array_entry_len == 0 {
                                            return Err(ExecuterPanic {
                                                reason: ThreadPanicReason::ArraySizeCorruption,
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        }

                                        let array_size = if (raw_type.data.len() - platform_size)
                                            == 0
                                        {
                                            0
                                        } else {
                                            (raw_type.data.len() - platform_size) / array_entry_len
                                        };
                                        if index > &array_size {
                                            return Err(ExecuterPanic {
                                                reason: ThreadPanicReason::IndexOutOfBounds(*index),
                                                code_location: format!("{}:{}", file!(), line!()),
                                            });
                                        } else {
                                            let absolute_position_start = (arch.usize_len()
                                                as usize)
                                                + (array_entry_len * index);
                                            let absolute_position_end =
                                                absolute_position_start + array_entry_len;
                                            let array_entry = &raw_type.data
                                                [absolute_position_start..absolute_position_end];
                                            current_stack.registers.X =
                                                StaticRawType::from_bytes(array_entry);
                                        }
                                    }
                                } else if raw_type.type_id.is_core_type() {
                                    current_stack.registers.X = StaticRawType::from_heap_reference(
                                        static_raw_type.to_uint(),
                                    )
                                } else {
                                    return Err(ExecuterPanic {
                                        reason: ThreadPanicReason::UnexpectedType(
                                            static_raw_type.type_id.id,
                                        ),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            }
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(
                                        static_raw_type.to_uint(),
                                    ),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        }
                    } else if static_raw_type.type_id.is_core_type() {
                        current_stack.registers.B = static_raw_type;
                    } else {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::UnexpectedType(static_raw_type.type_id.id),
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
            AddressingValues::AbsoluteStatic(e) => {
                let instruction = program[*e];
                current_stack.registers.X = match instruction.addressing_value {
                    AddressingValues::Immediate(static_raw_type) => {
                        if static_raw_type.type_id.is_stack_storable() {
                            static_raw_type
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::ImmediateUseViolation(
                                    static_raw_type.type_id.id,
                                ),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                }
            }
            AddressingValues::IndirectA => {
                current_stack.registers.X = current_stack.registers.C;
            }
            AddressingValues::IndirectB => {
                current_stack.registers.X = current_stack.registers.B;
            }
            AddressingValues::IndirectC => {
                current_stack.registers.X = current_stack.registers.C;
            }
            AddressingValues::IndirectY => {
                current_stack.registers.X = current_stack.registers.Y;
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
