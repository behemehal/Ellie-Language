#![allow(non_snake_case)]

mod A2B;
mod A2C;
mod A2D;
mod A2F;
mod A2I;
mod A2O;
mod A2S;
mod ADD;
mod AND;
mod ARR;
mod BRK;
mod CALL;
mod CALLN;
mod CO;
mod DEA;
mod DIV;
mod EQ;
mod EXP;
mod FN;
mod GQ;
mod GT;
mod JMP;
mod JMPA;
mod LDA;
mod LDB;
mod LDC;
mod LDX;
mod LDY;
mod LEN;
mod LQ;
mod LT;
mod MUL;
mod NE;
mod OR;
mod POPS;
mod PUSH;
mod RET;
mod SAR;
mod SPUS;
mod STA;
mod STB;
mod STC;
mod STR;
mod STX;
mod STY;
mod SUB;
mod _MOD;

use alloc::string::String;
use ellie_core::defs::{PlatformArchitecture, VmNativeCall};

use crate::{
    config::PROGRAM_MAX_SIZE,
    heap_memory::HeapMemory,
    program::ReadInstruction,
    stack::Stack,
    stack_memory::StackMemory,
    utils::{AddressingValues, ThreadPanicReason},
};

#[derive(Debug, Clone)]
pub struct ExecuterPanic {
    pub reason: ThreadPanicReason,
    pub code_location: String,
}

#[derive(Debug)]
pub struct CallFunction {
    pub hash: usize,
    pub stack_len: usize,
    pub escape_pos: usize,
    pub pos: usize,
}

pub enum ExecuterResult {
    Continue,
    DropStack,
    CallFunction(CallFunction),
    CallNativeFunction(VmNativeCall),
}

pub type StaticProgram<'a> = &'a [ReadInstruction; PROGRAM_MAX_SIZE];

pub trait InstructionExecuter {
    fn execute(
        &self,
        // Heap Memory
        heap_memory: &mut HeapMemory,
        program: StaticProgram,
        // Current stack
        current_stack: &mut Stack,
        // Stack Memory
        stack_memory: &mut StackMemory,
        // Instruction's Addressing Value
        addressing_value: &AddressingValues,
        // Platform architecture
        arch: PlatformArchitecture,
    ) -> Result<ExecuterResult, ExecuterPanic>;
}
