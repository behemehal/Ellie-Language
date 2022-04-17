use alloc::vec;
use alloc::vec::Vec;

#[derive(Clone, Debug)]
pub struct Immediate;

#[derive(Clone, Debug)]
pub struct Absolute(usize);

#[derive(Clone, Debug)]
pub struct IndirectA;

#[derive(Clone, Debug)]
pub struct IndirectB;

#[derive(Clone, Debug)]
pub struct IndirectC;

#[derive(Clone, Debug)]
pub struct IndirectX;

#[derive(Clone, Debug)]
pub struct IndirectY;

#[derive(Clone, Debug)]
pub struct Reference;

#[derive(Clone, Debug)]
pub enum AddressingModes {
    Immediate,
    Absolute(isize),
    IndirectA,
    IndirectB,
    IndirectC,
    IndirectX,
    IndirectY,
}

impl AddressingModes {
    pub fn mode(&self) -> u8 {
        match self {
            AddressingModes::Immediate => 0,
            AddressingModes::Absolute(_) => 1,
            AddressingModes::IndirectA => 2,
            AddressingModes::IndirectB => 3,
            AddressingModes::IndirectC => 4,
            AddressingModes::IndirectX => 5,
            AddressingModes::IndirectY => 6,
        }
    }

    pub fn arg(&self) -> Vec<u8> {
        match self {
            AddressingModes::Absolute(x) => x.to_le_bytes().to_vec(),
            _ => vec![],
        }
    }
}

impl core::fmt::Display for AddressingModes {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            AddressingModes::Absolute(value) => write!(f, "{}", value),
            _ => write!(f, ""),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Instruction {
    addressing_mode: AddressingModes,
}

impl Instruction {
    pub fn immediate() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Immediate,
        }
    }

    pub fn absolute(val: isize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Absolute(val),
        }
    }

    pub fn indirect_a() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectA,
        }
    }

    pub fn indirect_b() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectB,
        }
    }

    pub fn indirect_c() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectC,
        }
    }

    pub fn indirect_x() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectX,
        }
    }

    pub fn indirect_y() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::IndirectY,
        }
    }
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
}

impl Instructions {
    pub fn op_code(&self) -> Vec<u8> {
        match self {
            Instructions::LDA(instruction) => {
                let base = 0x01_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::LDB(instruction) => {
                let base = 0x07_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::LDC(instruction) => {
                let base = 0x0D_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::LDX(instruction) => {
                let base = 0x13_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::LDY(instruction) => {
                let base = 0x19_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::STA(instruction) => {
                let base = 0x1F_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::STB(instruction) => {
                let base = 0x25_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::STC(instruction) => {
                let base = 0x2B_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::STX(instruction) => {
                let base = 0x31_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::STY(instruction) => {
                let base = 0x37_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::EQ(instruction) => {
                let base = 0x3D_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::NE(instruction) => {
                let base = 0x45_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::GT(instruction) => {
                let base = 0x4C_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::LT(instruction) => {
                let base = 0x53_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::GQ(instruction) => {
                let base = 0x5A_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::LQ(instruction) => {
                let base = 0x61_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::AND(instruction) => {
                let base = 0x68_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::OR(instruction) => {
                let base = 0x6E_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::ADD(instruction) => {
                let base = 0x75_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::SUB(instruction) => {
                let base = 0x7C_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::MUL(instruction) => {
                let base = 0x83_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::EXP(instruction) => {
                let base = 0x8A_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::DIV(instruction) => {
                let base = 0x91_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
            Instructions::MOD(instruction) => {
                let base = 0x98_u8;
                let mode = instruction.addressing_mode.mode();
                let mut op_code = vec![base + mode];
                op_code.extend(instruction.addressing_mode.arg());
                op_code
            }
        }
    }
}

impl core::fmt::Display for Instructions {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
        match self {
            Instructions::LDA(instruction) => write!(f, "LDA {}", instruction.addressing_mode),
            Instructions::LDB(instruction) => write!(f, "LDB {}", instruction.addressing_mode),
            Instructions::LDC(instruction) => write!(f, "LDC {}", instruction.addressing_mode),
            Instructions::LDX(instruction) => write!(f, "LDX {}", instruction.addressing_mode),
            Instructions::LDY(instruction) => write!(f, "LDY {}", instruction.addressing_mode),
            Instructions::STA(instruction) => write!(f, "STA {}", instruction.addressing_mode),
            Instructions::STB(instruction) => write!(f, "STB {}", instruction.addressing_mode),
            Instructions::STC(instruction) => write!(f, "STC {}", instruction.addressing_mode),
            Instructions::STX(instruction) => write!(f, "STX {}", instruction.addressing_mode),
            Instructions::STY(instruction) => write!(f, "STY {}", instruction.addressing_mode),
            Instructions::EQ(instruction) => write!(f, "EQ {}", instruction.addressing_mode),
            Instructions::NE(instruction) => write!(f, "NE {}", instruction.addressing_mode),
            Instructions::GT(instruction) => write!(f, "GT {}", instruction.addressing_mode),
            Instructions::LT(instruction) => write!(f, "LT {}", instruction.addressing_mode),
            Instructions::GQ(instruction) => write!(f, "GQ {}", instruction.addressing_mode),
            Instructions::LQ(instruction) => write!(f, "LQ {}", instruction.addressing_mode),
            Instructions::AND(instruction) => write!(f, "AND {}", instruction.addressing_mode),
            Instructions::OR(instruction) => write!(f, "OR {}", instruction.addressing_mode),
            Instructions::ADD(instruction) => write!(f, "ADD {}", instruction.addressing_mode),
            Instructions::SUB(instruction) => write!(f, "SUB {}", instruction.addressing_mode),
            Instructions::MUL(instruction) => write!(f, "MUL {}", instruction.addressing_mode),
            Instructions::EXP(instruction) => write!(f, "EXP {}", instruction.addressing_mode),
            Instructions::DIV(instruction) => write!(f, "DIV {}", instruction.addressing_mode),
            Instructions::MOD(instruction) => write!(f, "MOD {}", instruction.addressing_mode),
        }
    }
}
