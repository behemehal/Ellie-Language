use crate::{addressing_modes::AddressingModes, types::Types};

#[derive(Clone, Debug, PartialEq)]
pub struct Instruction {
    pub addressing_mode: AddressingModes,
}

impl Instruction {
    pub fn implicit() -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Implicit,
        }
    }

    pub fn immediate(rtype: Types, val: [u8; 8]) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Immediate(rtype, val),
        }
    }

    pub fn absolute(pointer: usize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::Absolute(pointer),
        }
    }

    pub fn absolute_index(pointer: usize, index_pointer: usize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::AbsoluteIndex(pointer, index_pointer),
        }
    }

    pub fn absolute_property(pointer: usize, index_pointer: usize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::AbsoluteProperty(pointer, index_pointer),
        }
    }

    pub fn absolute_static(pointer: usize) -> Instruction {
        Instruction {
            addressing_mode: AddressingModes::AbsoluteStatic(pointer),
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

pub enum Registers {
    A,
    B,
    C,
    X,
    Y,
}
