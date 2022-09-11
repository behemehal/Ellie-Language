use crate::thread::Stack;
use alloc::{string::String, vec::Vec};
use ellie_core::{defs::CursorPosition, raw_type::RawType};

#[derive(Debug, Clone)]
pub enum ThreadPanicReason {
    IntegerOverflow,
    ByteOverflow,
    PlatformOverflow,
    FloatOverflow,
    DoubleOverflow,
    UnmergebleTypes(u8, u8),
    StackOverflow,
    BrokenStackTree(u8),
    UnexpectedType,
    OutOfInstructions,
    /// This might be triggered if the program trying to access a heap location that is not allocated
    InvalidMemoryAccess(usize),
    RuntimeError(String),
    InvalidRegisterAccess(u8),
    IndexAccessViolation(u8),
    IndexOutOfBounds(usize),
    CannotIndexWithNegative,
    ParemeterMemoryAccessViolation(usize),
    MemoryAccessViolation(usize, usize),
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

#[derive(Debug, Clone)]
pub enum ThreadStepInfo {
    /// The thread requires a step.
    StepNext,
    /// Thread called a function and pushed a new stack
    CALL(usize),
    /// Thread jumped to a position
    JMP(usize),
    /// Thread has no more stack to execute
    EndOfStacks,
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
    Vector,
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
            Types::Vector => String::from("Vector"),
            Types::Void => String::from("Void"),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum AddressingModes {
    Implicit,
    Immediate,
    Absolute,
    AbsoluteIndex,
    AbsoluteProperty,
    Parameter,
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

#[derive(Clone, Debug)]
pub struct Instruction {
    addressing_mode: AddressingModes,
}

#[derive(Clone, Debug)]
pub enum AddressingValues {
    Implicit,
    Immediate(RawType),
    Absolute(usize),
    AbsoluteIndex(usize, usize),
    AbsoluteProperty(usize, usize),
    Parameter(usize),
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

#[derive(Clone, Debug)]
pub enum Instructions {
    LDA(Instruction),
    LDB(Instruction),
    LDC(Instruction),
    LDX(Instruction),
    LDY(Instruction),
    STA(Instruction),
    STB(Instruction),
    STC(Instruction),
    STX(Instruction),
    STY(Instruction),
    EQ(Instruction),
    NE(Instruction),
    GT(Instruction),
    LT(Instruction),
    GQ(Instruction),
    LQ(Instruction),
    AND(Instruction),
    OR(Instruction),
    ADD(Instruction),
    SUB(Instruction),
    MUL(Instruction),
    EXP(Instruction),
    DIV(Instruction),
    MOD(Instruction),
    INC(Instruction),
    DEC(Instruction),
    JMP(Instruction),
    CALL(Instruction),
    RET(Instruction),
    UGR(Instruction),
    ULR(Instruction),
    PUSH(Instruction),
    LEN(Instruction),
    A2I(Instruction),
    A2F(Instruction),
    A2D(Instruction),
    A2B(Instruction),
    A2S(Instruction),
    A2C(Instruction),
    A2O(Instruction),
    JMPA(Instruction),
    POPS(Instruction),
    ACP(Instruction),
    BRK(Instruction),
    CALLN(Instruction),
    CO(Instruction),
    FN(Instruction),
}

impl Instructions {
    pub fn from(op_code: &u8) -> Option<Instructions> {
        let c = vec![1, 2, 3];
        let q = c.iter().map(|x| x.to_string());
        match op_code {
            1 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            2 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            3 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),

            4 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),

            5 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),

            6 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),

            7 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            8 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            9 => Some(Instructions::LDA(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            10 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            11 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            12 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),

            13 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),

            14 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),

            15 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),

            16 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            17 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            18 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            19 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            20 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            21 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),

            22 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),

            23 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),

            24 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),

            25 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            26 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            27 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            28 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            29 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            30 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),

            31 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),

            32 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),

            33 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),

            34 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            35 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            36 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            37 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            38 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            39 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),

            40 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),

            41 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),

            42 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),

            43 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            44 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            45 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            46 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            47 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            48 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            49 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            50 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            51 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            52 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            53 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            54 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            55 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            56 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            57 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            58 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            59 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            60 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            61 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            62 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            63 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            64 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            65 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            66 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            67 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            68 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            69 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            70 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            71 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            72 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            73 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            74 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Parameter,
            })),

            75 => Some(Instructions::EQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            76 => Some(Instructions::NE(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            77 => Some(Instructions::GT(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            78 => Some(Instructions::LT(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            79 => Some(Instructions::GQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            80 => Some(Instructions::LQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            81 => Some(Instructions::AND(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            82 => Some(Instructions::OR(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            83 => Some(Instructions::ADD(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            84 => Some(Instructions::SUB(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            85 => Some(Instructions::MUL(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            86 => Some(Instructions::EXP(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            87 => Some(Instructions::DIV(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            88 => Some(Instructions::MOD(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            89 => Some(Instructions::INC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            90 => Some(Instructions::DEC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            91 => Some(Instructions::JMP(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            92 => Some(Instructions::CALL(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            93 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            94 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            95 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            96 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),

            97 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),

            98 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),

            99 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),

            100 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),

            101 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            102 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            103 => Some(Instructions::UGR(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            104 => Some(Instructions::ULR(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            105 => Some(Instructions::ULR(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            106 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            107 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),

            108 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),

            109 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),

            110 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),

            111 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),

            112 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),

            113 => Some(Instructions::PUSH(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),

            114 => Some(Instructions::LEN(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            115 => Some(Instructions::A2I(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            116 => Some(Instructions::A2F(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            117 => Some(Instructions::A2D(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            118 => Some(Instructions::A2B(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            119 => Some(Instructions::A2S(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            120 => Some(Instructions::A2C(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            121 => Some(Instructions::A2O(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            122 => Some(Instructions::JMPA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            123 => Some(Instructions::POPS(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            124 => Some(Instructions::ACP(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            125 => Some(Instructions::BRK(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),

            126 => Some(Instructions::CALLN(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),

            127 => Some(Instructions::CO(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),

            128 => Some(Instructions::FN(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            _ => None,
        }
    }

    pub fn addressing_mode(&self) -> AddressingModes {
        match self {
            Instructions::LDA(e) => e.addressing_mode.clone(),
            Instructions::LDB(e) => e.addressing_mode.clone(),
            Instructions::LDC(e) => e.addressing_mode.clone(),
            Instructions::LDX(e) => e.addressing_mode.clone(),
            Instructions::LDY(e) => e.addressing_mode.clone(),
            Instructions::STA(e) => e.addressing_mode.clone(),
            Instructions::STB(e) => e.addressing_mode.clone(),
            Instructions::STC(e) => e.addressing_mode.clone(),
            Instructions::STX(e) => e.addressing_mode.clone(),
            Instructions::STY(e) => e.addressing_mode.clone(),
            Instructions::EQ(e) => e.addressing_mode.clone(),
            Instructions::NE(e) => e.addressing_mode.clone(),
            Instructions::GT(e) => e.addressing_mode.clone(),
            Instructions::LT(e) => e.addressing_mode.clone(),
            Instructions::GQ(e) => e.addressing_mode.clone(),
            Instructions::LQ(e) => e.addressing_mode.clone(),
            Instructions::AND(e) => e.addressing_mode.clone(),
            Instructions::OR(e) => e.addressing_mode.clone(),
            Instructions::ADD(e) => e.addressing_mode.clone(),
            Instructions::SUB(e) => e.addressing_mode.clone(),
            Instructions::MUL(e) => e.addressing_mode.clone(),
            Instructions::EXP(e) => e.addressing_mode.clone(),
            Instructions::DIV(e) => e.addressing_mode.clone(),
            Instructions::MOD(e) => e.addressing_mode.clone(),
            Instructions::INC(e) => e.addressing_mode.clone(),
            Instructions::DEC(e) => e.addressing_mode.clone(),
            Instructions::JMP(e) => e.addressing_mode.clone(),
            Instructions::CALL(e) => e.addressing_mode.clone(),
            Instructions::RET(e) => e.addressing_mode.clone(),
            Instructions::PUSH(e) => e.addressing_mode.clone(),
            Instructions::LEN(e) => e.addressing_mode.clone(),
            Instructions::A2I(e) => e.addressing_mode.clone(),
            Instructions::A2F(e) => e.addressing_mode.clone(),
            Instructions::A2D(e) => e.addressing_mode.clone(),
            Instructions::A2B(e) => e.addressing_mode.clone(),
            Instructions::A2S(e) => e.addressing_mode.clone(),
            Instructions::A2C(e) => e.addressing_mode.clone(),
            Instructions::A2O(e) => e.addressing_mode.clone(),
            Instructions::JMPA(e) => e.addressing_mode.clone(),
            Instructions::POPS(e) => e.addressing_mode.clone(),
            Instructions::ACP(e) => e.addressing_mode.clone(),
            Instructions::BRK(e) => e.addressing_mode.clone(),
            Instructions::CALLN(e) => e.addressing_mode.clone(),
            Instructions::CO(e) => e.addressing_mode.clone(),
            Instructions::FN(e) => e.addressing_mode.clone(),
            Instructions::UGR(e) => e.addressing_mode.clone(),
            Instructions::ULR(e) => e.addressing_mode.clone(),
        }
    }
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
