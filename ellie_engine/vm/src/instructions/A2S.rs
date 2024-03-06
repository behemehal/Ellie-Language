use alloc::{format, string::ToString};
use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory,
    instruction_utils::A2S,
    raw_type::{RawType, StaticRawType},
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult, StaticProgram};

impl super::InstructionExecuter for A2S {
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
                    1 => {
                        let data = current_stack.registers.A.to_int().to_string();
                        heap_memory.set(&current_stack.get_pos(), RawType::generate_string(data));
                        current_stack.registers.A =
                            StaticRawType::from_heap_reference(current_stack.get_pos());
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float().to_string();
                        heap_memory.set(&current_stack.get_pos(), RawType::generate_string(data));
                        current_stack.registers.A =
                            StaticRawType::from_heap_reference(current_stack.get_pos());
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double().to_string();
                        heap_memory.set(&current_stack.get_pos(), RawType::generate_string(data));
                        current_stack.registers.A =
                            StaticRawType::from_heap_reference(current_stack.get_pos());
                    }
                    4 => {
                        let data = current_stack.registers.A.to_byte().to_string();
                        heap_memory.set(&current_stack.get_pos(), RawType::generate_string(data));
                        current_stack.registers.A =
                            StaticRawType::from_heap_reference(current_stack.get_pos());
                    }
                    7 => {
                        let data = current_stack.registers.A.to_char().to_string();
                        heap_memory.set(&current_stack.get_pos(), RawType::generate_string(data));
                        current_stack.registers.A =
                            StaticRawType::from_heap_reference(current_stack.get_pos());
                    }
                    11 => {

                        let class_ref = current_stack.registers.A.to_uint();
                        let class = heap_memory.get(&class_ref);

                        panic!("Conver: {:#?}, ", class);

                        //let data = current_stack.registers.A.to_short().to_string();
                        //heap_memory.set(&current_stack.get_pos(), RawType::generate_string(data));
                        //current_stack.registers.A =
                        //    StaticRawType::from_heap_reference(current_stack.get_pos());
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
