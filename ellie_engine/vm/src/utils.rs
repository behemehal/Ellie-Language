use alloc::{string::String, vec::Vec};
use ellie_core::{defs::CursorPosition, raw_type::{StaticRawType, RawType}};

use crate::{heap_memory, stack::Stack};

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: usize,
    pub stack_id: usize,
    pub stack_caller: Option<usize>,
}

#[derive(Debug, Clone)]
pub enum ThreadPanicReason {
    IntegerOverflow,
    ByteOverflow,
    PlatformOverflow,
    FloatOverflow,
    DoubleOverflow,
    /// This panic triggered when the types are not mergeble with each other MOD, DIV, MUL, EXP, SUB AND ADD instructions can trigger this panic
    UnmergebleTypes(u8, u8),
    /// This panic triggered when the types are not comparable with each other
    UncomparableTypes(u8, u8),
    /// This panic triggered when stack exceeded the maximum size
    StackOverflow,
    BrokenStackTree(u8),
    /// This panic triggered when the value is not expected type
    UnexpectedType,
    /// This might be triggered if the program trying to access a heap location that is not allocated
    NullReference(usize),
    /// This panic will be triggered when there is no more instructions to read and stack did not drop properly
    OutOfInstructions,
    RuntimeError(String),
    InvalidRegisterAccess(u8),
    IndexAccessViolation(u8),
    IndexOutOfBounds(usize),
    CannotIndexWithNegative,
    ParemeterMemoryAccessViolation(usize),
    MemoryAccessViolation(usize, usize),
    /// This triggered when types like string, array, class tried to be kept in immediate mode
    ImmediateUseViolation(u8),
    InvalidType(u8),
    /// This panic triggered when the instruction's addressing value is not supported by the instruction
    IllegalAddressingValue,
    // This panic triggered from A2(n) instructions, when instruction does not support conversion between types
    CannotConvertToType(u8, u8),
    /// This panic is triggered when a native call not matched with any module_manager item
    CallToUnknown(usize),
    /// This panic is triggered when a native call not matched with any module_manager item
    MissingModule,
}

#[derive(Debug, Clone)]
pub struct StackNode {
    pub stack_name: String,
    pub location: CursorPosition,
    pub program_counter: usize,
}

#[derive(Debug, Clone)]
pub struct ThreadPanic {
    pub reason: ThreadPanicReason,
    pub stack_trace: Vec<Stack>,
    pub code_location: String,
}

#[derive(Debug, Clone)]
pub enum ThreadExit {
    Panic(ThreadPanic),
    ExitGracefully,
}

#[derive(Debug, Clone)]
pub struct ThreadStep {
    pub instruction: crate::program::ReadInstruction,
    pub stack_pos: usize,
    pub stack_id: usize,
    pub info: ThreadStepInfo,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ThreadStepInfo {
    /// The thread requires a step.
    StepNext,
    /// Thread called a function and pushed a new stack
    CALL(usize),
    /// Thread jumped to a position
    JMP(usize),
    /// Thread has no more stack to execute
    EndOfStacks,
    // Droping a stack
    DropStack,
}

pub enum ExitCode {
    Success,
    StackOverflow,
}

#[derive(Clone, Debug, PartialEq)]
pub enum Types {
    Integer,
    Float,
    Double,
    Byte,
    Bool,
    String,
    Char,
    Array,
    Void,
}

impl Types {
    pub fn display(&self) -> String {
        match self {
            Types::Integer => String::from("Integer"),
            Types::Float => String::from("Float"),
            Types::Double => String::from("Double"),
            Types::Byte => String::from("Byte"),
            Types::Bool => String::from("Bool"),
            Types::String => String::from("String"),
            Types::Char => String::from("Char"),
            Types::Array => String::from("Array"),
            Types::Void => String::from("Void"),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum AddressingModes {
    Implicit,
    Immediate,
    Absolute,
    AbsoluteIndex,
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

#[derive(Clone, Copy, Debug)]
pub enum AddressingValues {
    Implicit,
    Immediate(StaticRawType),
    Absolute(usize),
    AbsoluteIndex(usize, usize),
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

pub trait Reader {
    fn read(&mut self) -> Option<u8>;
}

pub struct ProgramReader<'a> {
    reader: &'a mut dyn Reader,
}

impl ProgramReader<'_> {
    pub fn new<'a>(vreader: &'a mut dyn Reader) -> ProgramReader<'a> {
        ProgramReader { reader: vreader }
    }

    pub fn read_usize(&mut self, arch_size: u8) -> Option<usize> {
        //Read usize in little endian
        let mut bytes = Vec::new();
        for _ in 0..arch_size {
            match self.reader.read() {
                Some(byte) => {
                    bytes.push(byte);
                }
                None => return None,
            }
        }
        Some(usize::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_isize(&mut self, arch_size: u8) -> Option<isize> {
        //Read usize in little endian
        let mut bytes = Vec::new();
        for _ in 0..arch_size {
            match self.reader.read() {
                Some(byte) => {
                    bytes.push(byte);
                }
                None => return None,
            }
        }
        Some(isize::from_le_bytes(bytes.try_into().unwrap()))
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.reader.read()
    }
}

pub fn resolve_reference(reference_data: usize, heap_memory: &heap_memory::HeapMemory) -> Result<RawType, usize> {
    match heap_memory.get(&reference_data) {
        Some(data) => {
            if data.type_id.id == 13 {
                resolve_reference(
                    usize::from_le_bytes(data.data.try_into().unwrap()),
                    heap_memory,
                )
            } else {
                Ok(data)
            }
        }
        None => Err(reference_data),
    }
}
