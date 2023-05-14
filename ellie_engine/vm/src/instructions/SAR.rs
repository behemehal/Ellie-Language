use ellie_core::defs::PlatformArchitecture;

use crate::{
    heap_memory::HeapMemory, instruction_utils::SAR, stack::Stack, stack_memory::StackMemory,
    utils::AddressingValues,
};

use super::StaticProgram;

impl super::InstructionExecuter for SAR {
    fn execute(
        &self,
        _heap_memory: &mut HeapMemory,
        _program: StaticProgram,
        _current_stack: &mut Stack,
        _stack_memory: &mut StackMemory,
        _addressing_value: &AddressingValues,
        _arch: PlatformArchitecture,
    ) -> Result<super::ExecuterResult, super::ExecuterPanic> {
        todo!()
    }
}
