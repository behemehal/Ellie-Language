use alloc::{format, string::String};
use ellie_core::{raw_type::{RawType, StaticRawType}, defs::PlatformArchitecture};

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::ADD,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{
        resolve_reference, AddressingValues, ReferenceType, ResolvedReference, ThreadPanicReason,
    },
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for ADD {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => {
                let B = if current_stack.registers.B.type_id.is_stack_reference()
                    || current_stack.registers.B.type_id.is_heap_reference()
                {
                    let reference_type = if current_stack.registers.B.type_id.is_stack_reference() {
                        ReferenceType::Stack
                    } else {
                        ReferenceType::Heap
                    };
                    let reference_data = current_stack.registers.B.to_int() as usize;
                    match resolve_reference(
                        reference_type,
                        reference_data,
                        heap_memory,
                        stack_memory,
                    ) {
                        Ok(e) => e,
                        Err(e) => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::ReferenceError(e),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                } else {
                    ResolvedReference::StaticRawType((current_stack.registers.B, 0))
                };
                let C = if current_stack.registers.C.type_id.is_stack_reference()
                    || current_stack.registers.C.type_id.is_heap_reference()
                {
                    let reference_type = if current_stack.registers.C.type_id.is_stack_reference() {
                        ReferenceType::Stack
                    } else {
                        ReferenceType::Heap
                    };
                    let reference_data = current_stack.registers.C.to_int() as usize;
                    match resolve_reference(
                        reference_type,
                        reference_data,
                        heap_memory,
                        stack_memory,
                    ) {
                        Ok(e) => e,
                        Err(e) => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::ReferenceError(e),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                } else {
                    ResolvedReference::StaticRawType((current_stack.registers.C, 0))
                };
                match (B.type_id().id, C.type_id().id) {
                    (1, 1) => {
                        let b_value = isize::from_le_bytes(current_stack.registers.B.data);
                        let c_value = isize::from_le_bytes(current_stack.registers.C.data);
                        let result = match b_value.checked_add(c_value) {
                            Some(e) => e,
                            None => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        };
                        current_stack.registers.A =
                            StaticRawType::integer(result.to_le_bytes().to_vec());
                    }
                    (2, 2) => {
                        let b_value = f32::from_le_bytes(
                            current_stack.registers.B.data[0..4].try_into().unwrap(),
                        );
                        let c_value = f32::from_le_bytes(
                            current_stack.registers.C.data[0..4].try_into().unwrap(),
                        );
                        let result = b_value + c_value;
                        if result.is_finite() {
                            current_stack.registers.A =
                                StaticRawType::float(result.to_le_bytes().to_vec());
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::FloatOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    (3, 3) => {
                        let b_value = f64::from_le_bytes(current_stack.registers.B.data);
                        let c_value = f64::from_le_bytes(current_stack.registers.C.data);
                        let result = b_value + c_value;
                        if result.is_finite() {
                            current_stack.registers.A =
                                StaticRawType::double(result.to_le_bytes().to_vec());
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::DoubleOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    // Byte + Byte
                    (4, 4) => {
                        let b_value = isize::from_le_bytes(current_stack.registers.B.data);
                        let c_value = isize::from_le_bytes(current_stack.registers.C.data);
                        let result = b_value + c_value;
                        if result > -128 && result < 127 {
                            current_stack.registers.A = StaticRawType::byte(result as u8);
                        } else {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::ByteOverflow,
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    (6, 6) => {
                        let mut b_value = String::new();
                        for i in B.data().unwrap().chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            b_value.push(char::from_u32(char).unwrap());
                        }
                        let mut c_value = String::new();
                        for i in C.data().unwrap().chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            c_value.push(char::from_u32(char).unwrap());
                        }
                        let result = b_value + &c_value;
                        heap_memory
                            .set(&(current_stack.get_pos()), RawType::generate_string(result));
                        current_stack.registers.A =
                            StaticRawType::heap_reference(current_stack.get_pos().to_le_bytes());
                    }
                    (1, 6) => {
                        let b_value = B.as_static_raw_type().unwrap().to_int();
                        let mut c_value = String::new();
                        for i in C.data().unwrap().chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            c_value.push(char::from_u32(char).unwrap());
                        }
                        let result = format!("{}{}", &b_value, &c_value);
                        heap_memory
                            .set(&(current_stack.get_pos()), RawType::generate_string(result));
                        current_stack.registers.A =
                            StaticRawType::heap_reference(current_stack.get_pos().to_le_bytes());
                    }
                    (6, 1) => {
                        let mut b_value = String::new();
                        for i in B.data().unwrap().chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            b_value.push(char::from_u32(char).unwrap());
                        }
                        let c_value = C.as_static_raw_type().unwrap().to_int();
                        let result = format!("{}{}", &b_value, &c_value);
                        heap_memory
                            .set(&(current_stack.get_pos()), RawType::generate_string(result));
                        current_stack.registers.A =
                            StaticRawType::heap_reference(current_stack.get_pos().to_le_bytes());
                    }
                    (9, 9) => {
                        let mut b_value = String::new();
                        for i in B.data().unwrap().chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            b_value.push(char::from_u32(char).unwrap());
                        }
                        let mut c_value = String::new();
                        for i in C.data().unwrap().chunks(4) {
                            let char = u32::from_le_bytes(i.try_into().unwrap());
                            c_value.push(char::from_u32(char).unwrap());
                        }
                        let result = b_value + &c_value;
                        heap_memory
                            .set(&(current_stack.get_pos()), RawType::generate_string(result));
                        current_stack.registers.A =
                            StaticRawType::heap_reference(current_stack.get_pos().to_le_bytes());
                    }
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::UnmergebleTypes(
                                B.type_id().id,
                                C.type_id().id,
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
