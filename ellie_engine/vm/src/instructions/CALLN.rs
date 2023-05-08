use alloc::{format, vec::Vec};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::CALLN,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason, VmNativeCall, VmNativeCallParameters},
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
        _arch: PlatformArchitecture,
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
                let mut params = Vec::new();
                let _start_position_of_params = current_stack.get_pos() - 2;

                for i in 0..params_length {
                    let pos =
                        current_stack.get_pos() - (params_length - ((i as isize * -1) as usize));
                    let paramater = match stack_memory.get(&pos) {
                        Some(raw_type) => match raw_type.type_id.id {
                            0..=5 | 7 | 10 => VmNativeCallParameters::Static(raw_type),
                            14 => {
                                let location = raw_type.to_int() as usize;
                                match heap_memory.get(&location) {
                                    Some(raw_type) => VmNativeCallParameters::Dynamic(raw_type),
                                    None => {
                                        return Err(ExecuterPanic {
                                            reason: ThreadPanicReason::NullReference(location),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    }
                                }
                            }
                            8 => {
                                return Err(ExecuterPanic {
                                    reason: ThreadPanicReason::NullReference(pos),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                            _ => unreachable!("Wrong typeid"),
                        },
                        None => {
                            return Err(ExecuterPanic {
                                reason: ThreadPanicReason::NullReference(pos),
                                code_location: format!("{}:{}", file!(), line!()),
                            })
                        }
                    };
                    params.push(paramater);
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
