use std::fmt::Display;

use ellie_core::raw_type::RawType;

use crate::thread::Stack;

pub trait Reader {
    fn read(&mut self) -> Option<u8>;
}

#[derive(Debug, Clone)]
pub enum ThreadPanicReason {
    IntegerOverflow,
    PlatformOverflow,
    FloatOverflow,
    DoubleOverflow,
    UnmergebleTypes,
    StackOverflow,
}

#[derive(Debug, Clone)]
pub struct ThreadPanic {
    pub reason: ThreadPanicReason,
    pub stack_trace: Vec<Stack>,
}

#[derive(Debug, Clone)]
pub enum ThreadExit {
    Panic(ThreadPanic),
    OutOfInstructions,
    ExitGracefully,
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
    //AbsoluteRef(usize, usize),
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
                addressing_mode: AddressingModes::Absolute,
            })),
            43 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            44 => Some(Instructions::STB(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            45 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            46 => Some(Instructions::STC(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            47 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            48 => Some(Instructions::STX(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            49 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            50 => Some(Instructions::STY(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            51 => Some(Instructions::EQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            52 => Some(Instructions::NE(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            53 => Some(Instructions::GT(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            54 => Some(Instructions::LT(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            55 => Some(Instructions::GQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            56 => Some(Instructions::LQ(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            57 => Some(Instructions::AND(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            58 => Some(Instructions::OR(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            59 => Some(Instructions::ADD(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            60 => Some(Instructions::SUB(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            61 => Some(Instructions::MUL(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            62 => Some(Instructions::EXP(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            63 => Some(Instructions::DIV(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            64 => Some(Instructions::MOD(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            65 => Some(Instructions::INC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            66 => Some(Instructions::DEC(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            67 => Some(Instructions::JMP(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            68 => Some(Instructions::CALL(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            69 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            70 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Immediate,
            })),
            71 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            72 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            73 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            74 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            75 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            76 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            77 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            78 => Some(Instructions::RET(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            79 => Some(Instructions::AOL(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            80 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            81 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectA,
            })),
            82 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectB,
            })),
            83 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectC,
            })),
            84 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectX,
            })),
            85 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::IndirectY,
            })),
            86 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::AbsoluteIndex,
            })),
            87 => Some(Instructions::PUSHA(Instruction {
                addressing_mode: AddressingModes::AbsoluteProperty,
            })),
            88 => Some(Instructions::LEN(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            89 => Some(Instructions::A2I(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            90 => Some(Instructions::A2F(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            91 => Some(Instructions::A2D(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            92 => Some(Instructions::A2B(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            93 => Some(Instructions::A2S(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            94 => Some(Instructions::A2C(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            95 => Some(Instructions::A2O(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            96 => Some(Instructions::JMPA(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            97 => Some(Instructions::POPS(Instruction {
                addressing_mode: AddressingModes::Implicit,
            })),
            98 => Some(Instructions::ACP(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            99 => Some(Instructions::BRK(Instruction {
                addressing_mode: AddressingModes::Absolute,
            })),
            100 => Some(Instructions::CALLN(Instruction {
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

pub struct ProgramReader<'a> {
    reader: &'a mut dyn Reader,
    op_code: u8,
    args: Vec<u8>,
}

impl ProgramReader<'_> {
    pub fn new<'a>(vreader: &'a mut dyn Reader) -> ProgramReader<'a> {
        ProgramReader {
            reader: vreader,
            op_code: 0,
            args: Vec::new(),
        }
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

pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_id = match self {
            Colors::Black => "[30m",
            Colors::Red => "[31m",
            Colors::Green => "[32m",
            Colors::Yellow => "[33m",
            Colors::Blue => "[34m",
            Colors::Magenta => "[35m",
            Colors::Cyan => "[36m",
            Colors::White => "[37m",
            Colors::Reset => "[0m",
        };
        write!(f, "{}{}", '\u{001b}', color_id)
    }
}

/*


 pub fn read(&self) -> u8 {
        match (self.reader)(Request::GetByte) {
            Response::SeekByte(_) => panic!("SeekByte not expected"),
            Response::GetByte(e) => e,
        }
    }

    pub fn seek(&self) -> bool {
        match (self.reader)(Request::SeekByte) {
            Response::SeekByte(e) => e,
            Response::GetByte(_) => panic!("GetByte not expected"),
        }
    }

*/
