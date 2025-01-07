use alloc::{format, vec::Vec};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::CALLN,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{
        resolve_parameter_data_from_raw_type, resolve_parameter_data_from_static_raw_type,
        resolve_reference, AddressingValues, FunctionCallParameter, RawFunctionData, ReferenceType,
        ResolvedReference, ThreadPanicReason, VmNativeCall,
    },
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for CALLN {
    fn execute(
        &self,
        heap_memory: &mut HeapMemory,
        program: StaticProgram,
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        addressing_value: &AddressingValues,
        arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic> {
        match addressing_value {
            AddressingValues::Absolute(start_location) => {
                let hash = match &program[*start_location].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };
                let return_heap_position = match &program[start_location + 1].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };
                let params_length = match &program[start_location + 2].addressing_value {
                    AddressingValues::Immediate(e) => e.to_int() as usize,
                    _ => {
                        return Err(ExecuterPanic {
                            reason: ThreadPanicReason::IllegalAddressingValue,
                            code_location: format!("{}:{}", file!(), line!()),
                        })
                    }
                };
                let mut params: Vec<FunctionCallParameter> = Vec::new();
                let _start_position_of_params = current_stack.get_pos() - 2;

                for i in 0..params_length {
                    let pos = current_stack.get_pos() - (params_length - i);
                    let parameter = match stack_memory.get(&pos) {
                        Some(raw_type) => {
                            if raw_type.type_id.is_stack_reference()
                                || raw_type.type_id.is_heap_reference()
                            {
                                match resolve_reference(
                                    if raw_type.type_id.is_stack_reference() {
                                        ReferenceType::Stack
                                    } else {
                                        ReferenceType::Heap
                                    },
                                    raw_type.to_uint(),
                                    heap_memory,
                                    stack_memory,
                                    arch,
                                )
                                .unwrap()
                                {
                                    ResolvedReference::StaticRawType(e) => FunctionCallParameter {
                                        data: resolve_parameter_data_from_static_raw_type(
                                            e.0,
                                            stack_memory,
                                            heap_memory,
                                            arch,
                                        ),
                                        raw_data: RawFunctionData::Static(e.0),
                                        memory_location: e.1,
                                    },
                                    ResolvedReference::RawType(e) => FunctionCallParameter {
                                        data: resolve_parameter_data_from_raw_type(
                                            e.0.clone(),
                                            stack_memory,
                                            heap_memory,
                                            arch,
                                        ),
                                        raw_data: RawFunctionData::Dynamic(e.0),
                                        memory_location: e.1,
                                    },
                                }
                            } else {
                                FunctionCallParameter {
                                    data: resolve_parameter_data_from_static_raw_type(
                                        raw_type,
                                        stack_memory,
                                        heap_memory,
                                        arch,
                                    ),
                                    raw_data: RawFunctionData::Static(raw_type),
                                    memory_location: pos,
                                }
                            }
                        }
                        None => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::NullReference(pos),
                                code_location: format!("{}:{}", file!(), line!()),
                            })
                        }
                    };
                    params.push(parameter);
                }
                Ok(ExecuterResult::CallNativeFunction(VmNativeCall {
                    hash,
                    params,
                    return_heap_position,
                }))
            }
            _ => Err(ExecuterPanic {
                reason: ThreadPanicReason::IllegalAddressingValue,
                code_location: format!("{}:{}", file!(), line!()),
            }),
        }
    }
}
