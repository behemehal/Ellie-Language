use core::mem;

use alloc::{string::String, vec::Vec};
use ellie_core::defs::CursorPosition;

use crate::{
    heap_memory,
    raw_type::{RawType, StaticRawType, TypeId},
    stack::Stack,
    stack_memory,
};

#[derive(Clone, Debug)]
pub enum VmNativeCallParameters {
    Static(StaticRawType),
    Dynamic(RawType),
}

#[derive(Clone, Debug)]
pub struct VmNativeCall {
    /// Native function's hash
    pub hash: usize,
    /// Parameter array
    pub params: Vec<VmNativeCallParameters>,
    /// Return heap position is location of the ret instruction
    /// If a non static value want to be returned, it will be stored in the heap,
    /// and Y register will be referencing to this position,
    /// so set the location of your dynamic value here
    pub return_heap_position: usize,
}

#[derive(Clone, Debug)]
pub enum VmNativeAnswer {
    Ok(VmNativeCallParameters),
    RuntimeError(String),
}

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: usize,
    pub stack_id: usize,
    pub frame_pos: usize,
    pub pos: usize,
    pub stack_caller: Option<usize>,
}

impl ThreadInfo {
    pub fn get_real_pos(&self) -> usize {
        self.frame_pos + self.pos
    }

    pub fn get_real_pos_with_location(&self, pos: usize) -> usize {
        self.frame_pos + pos
    }
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
    UnexpectedType(u8),
    /// This might be triggered if the program trying to access a heap location that is not allocated
    NullReference(usize),
    /// This panic will be triggered when there is no more instructions to read and stack did not drop properly
    OutOfInstructions,
    RuntimeError(String),
    InvalidRegisterAccess(u8),
    /// This panic triggered when the program trying to access a array index with wrong value
    IndexAccessViolation(u8),
    /// This panic triggered when the program trying to access a array index with out of bounds value
    IndexOutOfBounds(usize),
    /// This panic triggered when the program trying to write a value to array with unexpected size
    /// * first: expected size
    /// * second: given size
    WrongEntryLength(usize, usize),
    /// This panic triggered when the program trying to access a array index with negative value
    /// * first: index
    CannotIndexWithNegative(isize),
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
    CallToUnknown((String, usize)),
    /// This panic is triggered when a native call not matched with any module_manager item
    MissingModule(usize),
    /// This panic is triggered when a native call does not registered as trace
    MissingTrace(usize),
    /// Usally arrays are created with first index of it as it's entries size
    /// If array data doesnt have the entry_size or entry_size is zero or less this panic will be triggered
    ArraySizeCorruption,
    /// Reference error, this could be triggered when the program trying to access a reference that does not exists
    /// * location: Heap or Stack location of the data that is trying to be accessed
    ReferenceError(usize),
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
pub enum StepResult {
    Step,
    ThreadExit(ThreadExit),
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
    AbsoluteProperty,
    AbsoluteStatic,
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
    AbsoluteProperty(usize, usize),
    AbsoluteStatic(usize),
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
    pub fn new(vreader: &mut dyn Reader) -> ProgramReader<'_> {
        ProgramReader { reader: vreader }
    }

    pub fn read_usize(&mut self, arch_size: u8) -> Option<usize> {
        //Read usize in little endian
        let mut array = [0; mem::size_of::<usize>()];
        for i in 0..mem::size_of::<usize>() {
            if arch_size > i as u8 {
                match self.reader.read() {
                    Some(byte) => {
                        array[i] = byte;
                    }
                    None => return None,
                }
            } else {
                array[i] = 0
            }
        }
        Some(usize::from_le_bytes(array))
    }

    pub fn read_string(&mut self, string_length: usize) -> Option<String> {
        let mut string = String::new();
        for _ in 0..string_length {
            match self.reader.read() {
                Some(byte) => {
                    string.push(byte as char);
                }
                None => return None,
            }
        }
        Some(string)
    }

    pub fn read_isize(&mut self, arch_size: u8) -> Option<isize> {
        //Read isize in little endian
        let mut array = [0; mem::size_of::<isize>()];

        for i in 0..mem::size_of::<isize>() {
            if arch_size > i as u8 {
                match self.reader.read() {
                    Some(byte) => {
                        array[i] = byte;
                    }
                    None => return None,
                }
            } else {
                array[i] = 0
            }
        }
        Some(isize::from_le_bytes(array))
    }

    pub fn read_u8(&mut self) -> Option<u8> {
        self.reader.read()
    }
}

pub enum ResolvedReference {
    /// Static raw type
    /// * (StaticRawType, usize) = (StaticRawType, location)
    StaticRawType((StaticRawType, usize)),
    /// Raw type
    /// * (RawType, usize) = (RawType, location)
    RawType((RawType, usize)),
}

impl ResolvedReference {
    pub fn type_id(&self) -> TypeId {
        match self {
            ResolvedReference::StaticRawType(e) => e.0.type_id,
            ResolvedReference::RawType(e) => e.0.type_id,
        }
    }

    pub fn data(&self) -> Option<&Vec<u8>> {
        match self {
            ResolvedReference::RawType(e) => Some(&e.0.data),
            _ => None,
        }
    }

    pub fn as_static_raw_type(&self) -> Option<&StaticRawType> {
        match self {
            ResolvedReference::StaticRawType(e) => Some(&e.0),
            _ => None,
        }
    }

    pub fn as_raw_type(&self) -> Option<&RawType> {
        match self {
            ResolvedReference::RawType(e) => Some(&e.0),
            _ => None,
        }
    }
}

pub enum ReferenceType {
    Heap,
    Stack,
}

impl ReferenceType {
    pub fn from(ref_type: usize) -> ReferenceType {
        match ref_type {
            13 => ReferenceType::Stack,
            14 => ReferenceType::Heap,
            _ => panic!("Invalid reference type"),
        }
    }
}

pub fn resolve_reference(
    reference_type: ReferenceType,
    reference_data: usize,
    heap_memory: &heap_memory::HeapMemory,
    stack_memory: &stack_memory::StackMemory,
) -> Result<ResolvedReference, usize> {
    match reference_type {
        ReferenceType::Heap => match heap_memory.get(&reference_data) {
            Some(data) => {
                if data.type_id.id == 13 {
                    resolve_reference(
                        ReferenceType::Stack,
                        usize::from_le_bytes(data.data.try_into().unwrap()),
                        heap_memory,
                        stack_memory,
                    )
                } else if data.type_id.id == 14 {
                    resolve_reference(
                        ReferenceType::Heap,
                        usize::from_le_bytes(data.data.try_into().unwrap()),
                        heap_memory,
                        stack_memory,
                    )
                } else {
                    Ok(ResolvedReference::RawType((data, reference_data)))
                }
            }
            None => Err(reference_data),
        },
        ReferenceType::Stack => match stack_memory.get(&reference_data) {
            Some(data) => {
                if data.type_id.id == 13 {
                    resolve_reference(
                        ReferenceType::Stack,
                        data.to_int() as usize,
                        heap_memory,
                        stack_memory,
                    )
                } else if data.type_id.id == 14 {
                    resolve_reference(
                        ReferenceType::Heap,
                        data.to_int() as usize,
                        heap_memory,
                        stack_memory,
                    )
                } else {
                    Ok(ResolvedReference::StaticRawType((data, reference_data)))
                }
            }
            None => Err(reference_data),
        },
    }
}
