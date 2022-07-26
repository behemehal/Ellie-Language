use crate::thread::Stack;
use ellie_core::{defs::CursorPosition, raw_type::RawType};

#[derive(Debug, Clone)]
pub enum ThreadPanicReason {
    IntegerOverflow,
    PlatformOverflow,
    FloatOverflow,
    DoubleOverflow,
    UnmergebleTypes,
    StackOverflow,
    UnexpectedType,
    OutOfInstructions,
    RuntimeError(String),
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
    AOL(Instruction),
    PUSHA(Instruction),
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
}

impl Instructions {
    pub fn from(op_code: &u8) -> Option<Instructions> {
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
            9 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            10 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            11 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            12 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            13 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            14 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            15 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            16 => Some(Instructions::LDB(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            17 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            18 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            19 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            20 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            21 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            22 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            23 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            24 => Some(Instructions::LDC(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            25 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            26 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            27 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            28 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            29 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            30 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            31 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            32 => Some(Instructions::LDX(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            33 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            34 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            35 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            36 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            38 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            37 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            39 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            40 => Some(Instructions::LDY(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            41 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            42 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            43 => Some(Instructions::STA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            44 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            45 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            46 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            47 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            48 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            49 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            50 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            51 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            52 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            53 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            54 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            55 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            56 => Some(Instructions::EQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            57 => Some(Instructions::NE(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            58 => Some(Instructions::GT(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            59 => Some(Instructions::LT(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            60 => Some(Instructions::GQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            61 => Some(Instructions::LQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            62 => Some(Instructions::AND(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            63 => Some(Instructions::OR(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            64 => Some(Instructions::ADD(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            65 => Some(Instructions::SUB(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            66 => Some(Instructions::MUL(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            67 => Some(Instructions::EXP(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            68 => Some(Instructions::DIV(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            69 => Some(Instructions::MOD(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            70 => Some(Instructions::INC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            71 => Some(Instructions::DEC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            72 => Some(Instructions::JMP(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            73 => Some(Instructions::CALL(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            74 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            75 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            76 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            77 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            78 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            79 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            80 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            81 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            82 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            83 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            84 => Some(Instructions::AOL(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            85 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            86 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            87 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            88 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            89 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            90 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            91 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            92 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            93 => Some(Instructions::LEN(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            94 => Some(Instructions::A2I(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            95 => Some(Instructions::A2F(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            96 => Some(Instructions::A2D(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            97 => Some(Instructions::A2B(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            98 => Some(Instructions::A2S(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            99 => Some(Instructions::A2C(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            100 => Some(Instructions::A2O(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            101 => Some(Instructions::JMPA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            102 => Some(Instructions::POPS(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            103 => Some(Instructions::ACP(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            104 => Some(Instructions::BRK(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            105 => Some(Instructions::CALLN(Instruction {
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
            Instructions::AOL(e) => e.addressing_mode.clone(),
            Instructions::PUSHA(e) => e.addressing_mode.clone(),
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
