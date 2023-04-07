use alloc::format;
use ellie_core::{defs::PlatformArchitecture, raw_type::StaticRawType};

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::NE,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for NE {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => {
                match (
                    current_stack.registers.B.type_id.id,
                    current_stack.registers.C.type_id.id,
                ) {
                    (1, 1) | (1, 4) => {
                        let b_value = current_stack.registers.B.to_int();
                        let c_value = current_stack.registers.C.to_int();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (2, 2) => {
                        let b_value = current_stack.registers.B.to_float();
                        let c_value = current_stack.registers.C.to_float();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (3, 3) => {
                        let b_value = current_stack.registers.B.to_double();
                        let c_value = current_stack.registers.C.to_double();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (2, 3) | (3, 2) => {
                        let b_value = current_stack.registers.B.to_double();
                        let c_value = current_stack.registers.C.to_double();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (4, 4) | (4, 1) => {
                        let b_value = current_stack.registers.B.to_byte();
                        let c_value = current_stack.registers.C.to_byte();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (5, 5) => {
                        let b_value = current_stack.registers.B.to_bool();
                        let c_value = current_stack.registers.C.to_bool();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (7, 7) => {
                        let b_value = current_stack.registers.B.to_char();
                        let c_value = current_stack.registers.C.to_char();
                        current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                    }
                    (13, 13) => {
                        let b_pointer = usize::from_le_bytes(current_stack.registers.B.data);
                        let b_ref = match heap_memory.get(&b_pointer) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(b_pointer),
                                    code_location: format!("{}:{}", file!(), line!()),
                                })
                            }
                        };
                        let c_pointer = usize::from_le_bytes(current_stack.registers.C.data);
                        let c_ref = match heap_memory.get(&c_pointer) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(c_pointer),
                                    code_location: format!("{}:{}", file!(), line!()),
                                })
                            }
                        };
                        match (b_ref.type_id.id, c_ref.type_id.id) {
                            (6, 6) => {
                                let b_value = b_ref.to_string();
                                let c_value = c_ref.to_string();
                                current_stack.registers.A = StaticRawType::bool(b_value != c_value);
                            }
                            e => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::UncomparableTypes(e.0, e.1),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        }
                    }
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::UncomparableTypes(
                                current_stack.registers.B.type_id.id,
                                current_stack.registers.C.type_id.id,
                            ),
                            code_location: format!("{}:{}", file!(), line!()),
                        });
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
