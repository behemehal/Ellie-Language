use alloc::{string::String, vec::Vec};
use core::mem;
use ellie_core::defs::{CursorPosition, PlatformArchitecture};
use enum_as_inner::EnumAsInner;
use std::string::ToString;

use crate::{
    heap_memory::{self, HeapMemory},
    raw_type::{RawType, StaticRawType, TypeId},
    stack::Stack,
    stack_memory::{self, StackMemory},
};

#[derive(Clone, Debug)]
pub enum RawFunctionData {
    /// Static raw type
    Static(StaticRawType),
    /// Raw type
    Dynamic(RawType),
}

impl RawFunctionData {
    pub fn as_static(self) -> Option<StaticRawType> {
        match self {
            RawFunctionData::Static(e) => Some(e),
            _ => None,
        }
    }

    pub fn as_dynamic(self) -> Option<RawType> {
        match self {
            RawFunctionData::Dynamic(e) => Some(e),
            _ => None,
        }
    }
}

#[derive(Clone, Debug)]
pub struct EllieInteger {
    pub as_isize: isize,
    pub as_usize: usize,
}

impl From<isize> for EllieInteger {
    fn from(value: isize) -> Self {
        EllieInteger {
            as_isize: value,
            as_usize: value as usize,
        }
    }
}

impl From<usize> for EllieInteger {
    fn from(value: usize) -> Self {
        EllieInteger {
            as_isize: value as isize,
            as_usize: value,
        }
    }
}

impl From<&EllieInteger> for isize {
    fn from(value: &EllieInteger) -> Self {
        value.as_isize
    }
}

impl From<&EllieInteger> for usize {
    fn from(value: &EllieInteger) -> Self {
        value.as_usize
    }
}

#[derive(Clone, Debug, EnumAsInner)]
pub enum EllieData {
    Integer(EllieInteger),
    Float(f32),
    Double(f64),
    Byte(u8),
    Bool(bool),
    String(String),
    Char(char),
    Void,
    Null,
    Array(Vec<EllieData>),
    Class(Vec<EllieData>),
}

impl Into<EllieData> for &str {
    fn into(self) -> EllieData {
        EllieData::String(self.to_string())
    }
}

impl Into<EllieData> for String {
    fn into(self) -> EllieData {
        EllieData::String(self)
    }
}

impl Into<EllieData> for char {
    fn into(self) -> EllieData {
        EllieData::Char(self)
    }
}

impl Into<EllieData> for u8 {
    fn into(self) -> EllieData {
        EllieData::Byte(self)
    }
}

impl Into<EllieData> for f32 {
    fn into(self) -> EllieData {
        EllieData::Float(self)
    }
}

impl Into<EllieData> for f64 {
    fn into(self) -> EllieData {
        EllieData::Double(self)
    }
}

impl Into<EllieData> for isize {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for usize {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self,
        })
    }
}

#[cfg(target_pointer_width = "64")]
impl Into<EllieData> for i128 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

#[cfg(target_pointer_width = "64")]
impl Into<EllieData> for u128 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for u64 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for i64 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for u32 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for i32 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for u16 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for i16 {
    fn into(self) -> EllieData {
        EllieData::Integer(EllieInteger {
            as_isize: self as isize,
            as_usize: self as usize,
        })
    }
}

impl Into<EllieData> for bool {
    fn into(self) -> EllieData {
        EllieData::Bool(self)
    }
}

impl Into<EllieData> for () {
    fn into(self) -> EllieData {
        EllieData::Void
    }
}

impl Into<EllieData> for Vec<EllieData> {
    fn into(self) -> EllieData {
        EllieData::Array(self)
    }
}

#[derive(Clone, Debug)]
pub struct FunctionCallParameter {
    pub data: EllieData,
    pub raw_data: RawFunctionData,
    pub memory_location: usize,
}

#[derive(Clone, Debug)]
pub struct VmNativeCall {
    /// Native function's hash
    pub hash: usize,
    /// Parameter array
    pub params: Vec<FunctionCallParameter>,
    /// Return heap position is location of the ret instruction
    /// If a non static value want to be returned, it will be stored in the heap,
    /// and Y register will be referencing to this position,
    /// so set the location of your dynamic value here
    pub return_heap_position: usize,
}

#[derive(Clone, Debug)]
// ! Todo: implement Into for VmNativeAnswer
pub enum VmNativeAnswer {
    Ok(EllieData),
    RuntimeError(String),
}

impl Into<VmNativeAnswer> for EllieData {
    fn into(self) -> VmNativeAnswer {
        VmNativeAnswer::Ok(self)
    }
}

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: usize,
    pub stack_id: usize,
    pub frame_pos: usize,
    pub pos: usize,
    pub stack_caller: Option<usize>,
    pub arch: PlatformArchitecture,
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
    UnmergebleTypes(String, String),
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
    /// This panic triggered when the program trying to access a array index with out of bounds value (index, size)
    IndexOutOfBounds(usize, usize),
    /// This panic triggered when the program trying to write a value to array with unexpected size
    /// * first: expected size
    /// * second: given size
    WrongEntryLength(usize, usize),
    /// This panic triggered when the program trying to access a array index with negative value
    /// * first: index
    CannotIndexWithNegative(isize),
    ParameterMemoryAccessViolation(usize),
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
    arch: PlatformArchitecture,
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
                        arch,
                    )
                } else if data.type_id.id == 14 {
                    resolve_reference(
                        ReferenceType::Heap,
                        usize::from_le_bytes(data.data.try_into().unwrap()),
                        heap_memory,
                        stack_memory,
                        arch,
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
                        arch,
                    )
                } else if data.type_id.id == 14 {
                    resolve_reference(
                        ReferenceType::Heap,
                        data.to_int() as usize,
                        heap_memory,
                        stack_memory,
                        arch,
                    )
                } else {
                    Ok(ResolvedReference::StaticRawType((data, reference_data)))
                }
            }
            None => Err(reference_data),
        },
    }
}

pub fn resolve_parameter_data_from_static_raw_type(
    raw_data: StaticRawType,
    stack_memory: &mut StackMemory,
    heap_memory: &mut HeapMemory,
    arch: PlatformArchitecture,
) -> EllieData {
    match raw_data.type_id.id {
        1 => {
            let as_isize = raw_data.to_int();
            let as_usize = raw_data.to_uint();
            EllieData::Integer(EllieInteger { as_isize, as_usize })
        }
        2 => EllieData::Float(raw_data.to_float()),
        3 => EllieData::Double(raw_data.to_double()),
        4 => EllieData::Byte(raw_data.to_byte()),
        5 => EllieData::Bool(raw_data.to_bool()),
        6 => unreachable!("String type is not supported in static raw type"),
        7 => EllieData::Char(raw_data.to_char()),
        8 => EllieData::Void,
        9 => {
            todo!()
        }
        10 => EllieData::Null,
        11 => {
            std::println!("class origin: {:#?}", raw_data);
            let array_origin = raw_data.to_uint();
            std::println!("class origin: {:#?}", array_origin);

            let class_data = heap_memory.get(&array_origin).unwrap();

            std::println!("class_data: {:#?}", class_data);

            let array_entry_size = usize::from_le_bytes(
                class_data.data[..arch.usize_len() as usize]
                    .try_into()
                    .unwrap(),
            );
            let array_data = &class_data.data[arch.usize_len() as usize..];
            let array_entries = array_data.chunks(array_entry_size).collect::<Vec<_>>();

            let class_variables_raw = array_entries
                .iter()
                .map(|entry| RawType::from_bytes(&entry))
                .collect::<Vec<_>>();

            EllieData::Class(
                class_variables_raw
                    .iter()
                    .map(|raw_type| {
                        resolve_parameter_data_from_raw_type(
                            raw_type.clone(),
                            stack_memory,
                            heap_memory,
                            arch,
                        )
                    })
                    .collect::<Vec<_>>(),
            )
        }
        13 => {
            let reference_data = raw_data.to_uint();
            let resolved_reference = resolve_reference(
                ReferenceType::Stack,
                reference_data,
                heap_memory,
                stack_memory,
                arch,
            )
            .expect("Memory corruption");

            match resolved_reference {
                ResolvedReference::StaticRawType(static_raw_type) => {
                    resolve_parameter_data_from_static_raw_type(
                        static_raw_type.0,
                        stack_memory,
                        heap_memory,
                        arch,
                    )
                }
                ResolvedReference::RawType(raw_type) => resolve_parameter_data_from_raw_type(
                    raw_type.0,
                    stack_memory,
                    heap_memory,
                    arch,
                ),
            }
        }
        14 => {
            let reference_data = raw_data.to_uint();
            let resolved_reference = resolve_reference(
                ReferenceType::Heap,
                reference_data,
                heap_memory,
                stack_memory,
                arch,
            )
            .expect("Memory corruption");

            match resolved_reference {
                ResolvedReference::StaticRawType(static_raw_type) => {
                    resolve_parameter_data_from_static_raw_type(
                        static_raw_type.0,
                        stack_memory,
                        heap_memory,
                        arch,
                    )
                }
                ResolvedReference::RawType(raw_type) => resolve_parameter_data_from_raw_type(
                    raw_type.0,
                    stack_memory,
                    heap_memory,
                    arch,
                ),
            }
        }
        15 => {
            let array_origin = raw_data.to_uint();
            let array_size = stack_memory.get(&(array_origin + 1)).unwrap().to_uint();
            let index_on_stack = array_origin + 2;
            let mut array = Vec::new();

            for i in 0..array_size {
                let pos = index_on_stack + i;
                let static_raw_type = stack_memory.get(&pos).unwrap();

                let definite_element = if static_raw_type.type_id.is_heap_reference()
                    || static_raw_type.type_id.is_stack_reference()
                {
                    resolve_reference(
                        if static_raw_type.type_id.is_heap_reference() {
                            ReferenceType::Heap
                        } else {
                            ReferenceType::Stack
                        },
                        static_raw_type.to_uint(),
                        heap_memory,
                        stack_memory,
                        arch,
                    )
                    .expect("Memory corruption")
                } else {
                    ResolvedReference::StaticRawType((static_raw_type, pos))
                };

                match definite_element {
                    ResolvedReference::StaticRawType(static_raw_type) => {
                        array.push(resolve_parameter_data_from_static_raw_type(
                            static_raw_type.0,
                            stack_memory,
                            heap_memory,
                            arch,
                        ));
                    }
                    ResolvedReference::RawType(raw_type) => {
                        array.push(resolve_parameter_data_from_raw_type(
                            raw_type.0,
                            stack_memory,
                            heap_memory,
                            arch,
                        ));
                    }
                }
            }

            EllieData::Array(array)
        }
        _ => unreachable!("Unknown type id"),
    }
}

pub fn resolve_parameter_data_from_raw_type(
    raw_data: RawType,
    stack_memory: &mut StackMemory,
    heap_memory: &mut HeapMemory,
    arch: PlatformArchitecture,
) -> EllieData {
    match raw_data.type_id.id {
        1 => {
            let as_isize = raw_data.to_int();
            let as_usize = raw_data.to_uint();
            EllieData::Integer(EllieInteger { as_isize, as_usize })
        }
        2 => EllieData::Float(raw_data.to_float()),
        3 => EllieData::Double(raw_data.to_double()),
        4 => EllieData::Byte(raw_data.to_byte()),
        5 => EllieData::Bool(raw_data.to_bool()),
        6 => EllieData::String(raw_data.to_string()),
        7 => EllieData::Char(raw_data.to_char()),
        8 => EllieData::Void,
        9 => {
            todo!()
        }
        10 => EllieData::Null,
        13 => {
            let reference_data = raw_data.to_uint();
            let resolved_reference = resolve_reference(
                ReferenceType::Stack,
                reference_data,
                heap_memory,
                stack_memory,
                arch,
            )
            .expect("Memory corruption");

            match resolved_reference {
                ResolvedReference::StaticRawType(static_raw_type) => {
                    resolve_parameter_data_from_static_raw_type(
                        static_raw_type.0,
                        stack_memory,
                        heap_memory,
                        arch,
                    )
                }
                ResolvedReference::RawType(raw_type) => resolve_parameter_data_from_raw_type(
                    raw_type.0,
                    stack_memory,
                    heap_memory,
                    arch,
                ),
            }
        }
        14 => {
            let reference_data = raw_data.to_uint();
            let resolved_reference = resolve_reference(
                ReferenceType::Heap,
                reference_data,
                heap_memory,
                stack_memory,
                arch,
            )
            .expect("Memory corruption");

            match resolved_reference {
                ResolvedReference::StaticRawType(static_raw_type) => {
                    resolve_parameter_data_from_static_raw_type(
                        static_raw_type.0,
                        stack_memory,
                        heap_memory,
                        arch,
                    )
                }
                ResolvedReference::RawType(raw_type) => resolve_parameter_data_from_raw_type(
                    raw_type.0,
                    stack_memory,
                    heap_memory,
                    arch,
                ),
            }
        }
        15 => {
            let array_origin = raw_data.to_uint();
            let array_size = stack_memory.get(&(array_origin + 1)).unwrap().to_uint();

            let index_on_stack = array_origin + 2;
            let mut array = Vec::new();

            for i in 0..array_size {
                let pos = index_on_stack + i;
                let raw_type = stack_memory.get(&pos).unwrap();
                match resolve_reference(
                    ReferenceType::Stack,
                    raw_type.to_uint(),
                    heap_memory,
                    stack_memory,
                    arch,
                )
                .expect("Memory corruption")
                {
                    ResolvedReference::StaticRawType(static_raw_type) => {
                        array.push(resolve_parameter_data_from_static_raw_type(
                            static_raw_type.0,
                            stack_memory,
                            heap_memory,
                            arch,
                        ));
                    }
                    ResolvedReference::RawType(raw_type) => {
                        array.push(resolve_parameter_data_from_raw_type(
                            raw_type.0,
                            stack_memory,
                            heap_memory,
                            arch,
                        ))
                    }
                }
            }

            EllieData::Array(array)
        }
        _ => unreachable!("Unknown type id"),
    }
}

pub fn ellie_data_to_static_raw_type(data: EllieData) -> StaticRawType {
    match data {
        EllieData::Integer(integer) => StaticRawType::from_int(integer.as_isize),
        EllieData::Float(float) => StaticRawType::from_float(float),
        EllieData::Double(double) => StaticRawType::from_double(double),
        EllieData::Byte(byte) => StaticRawType::from_byte(byte),
        EllieData::Bool(bool) => StaticRawType::from_bool(bool),
        EllieData::Char(char) => StaticRawType::from_char(char),
        EllieData::Void => StaticRawType::from_void(),
        EllieData::Null => StaticRawType::from_null(),
        _ => unreachable!(),
    }
}

#[macro_export]
/// Asserts the argument size
/// ## Parameters
/// * `$args` - Arguments
/// * `$expected_size` - Expected size
/// ## Returns
/// * `VmNativeAnswer::RuntimeError` if the argument size is not equal to the expected size
/// * `VmNativeAnswer::Ok` if the argument size is equal to the expected size
/// ## Example
/// ```rust
/// use ellie_vm::{assert_arg_size, channel::{EllieModule, FunctionElement, ModuleElements, VmNativeAnswer}};
/// let mut ellie_core_module = EllieModule::new("ellieCore".to_string());
///
/// ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
///     "println",
///     Box::new(|_, args| {
///         assert_arg_size!(args, 1);
///         VmNativeAnswer::Ok(().into())
///     }),
/// )));
///```
macro_rules! assert_arg_size {
    ($args:expr, $expected_size:expr) => {
        if $args.len() != $expected_size {
            return VmNativeAnswer::RuntimeError(format!(
                "Signature mismatch, expected {} argument(s)",
                $expected_size
            ));
        }
    };
}
