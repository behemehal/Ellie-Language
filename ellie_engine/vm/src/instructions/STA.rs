use alloc::format;

use crate::{
    channel::ModuleManager,
    config::PROGRAM_MAX_SIZE,
    heap_memory::HeapMemory,
    instruction_utils::STA,
    program::ReadInstruction,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

use super::{ExecuterPanic, ExecuterResult};

impl super::InstructionExecuter for STA {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: &[ReadInstruction; PROGRAM_MAX_SIZE],
        current_stack: &mut Stack,
        stack_memory: &mut StackMemory,
        _module_manager: &ModuleManager,
        addressing_value: &AddressingValues,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        match &addressing_value {
            AddressingValues::Implicit => {
                stack_memory.set(&current_stack.get_pos(), current_stack.registers.A);
            }
            AddressingValues::Immediate(raw_type) => {
                stack_memory.set(&current_stack.get_pos(), raw_type.clone());
            }
            AddressingValues::Absolute(e) => {
                stack_memory.set(&(e + current_stack.frame_pos), current_stack.registers.A);
            }
            AddressingValues::AbsoluteIndex(_, _) => {
                todo!("Implementation is missing")
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
