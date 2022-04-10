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
    Absolute(usize),
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

impl Instruction {
    pub fn immediate() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Immediate,
        }
    }

    pub fn absolute(val: usize) -> Instruction {
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
