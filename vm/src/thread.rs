use crate::{
    heap,
    program::ReadInstruction,
    utils::{self, Instructions, Types},
};

pub struct Registers {
    pub A: (Types, isize),
    pub B: (Types, isize),
    pub C: (Types, isize),
    pub X: (Types, isize),
    pub Y: (Types, isize),
}

pub struct Thread<'a> {
    pub id: u32,
    pub stack: &'a Vec<ReadInstruction>,
    pub heap: &'a mut heap::Heap,
    pub registers: Registers,
    pub stack_pointer: usize,
}

impl<'a> Thread<'a> {
    pub fn new(id: u32, stack: &'a Vec<ReadInstruction>, heap: &'a mut heap::Heap) -> Self {
        Thread {
            id,
            stack,
            heap,
            stack_pointer: 0,
            registers: Registers {
                A: (Types::Void, 0),
                B: (Types::Void, 0),
                C: (Types::Void, 0),
                X: (Types::Void, 0),
                Y: (Types::Void, 0),
            },
        }
    }

    pub fn run(&mut self) -> u8 {
        println!(
            "{}[VM]{}: Running thread {}'{}'{}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            utils::Colors::Cyan,
            self.id,
            utils::Colors::Reset,
        );

        let mut exit_code = 0;
        loop {
            if self.stack_pointer == self.stack.len() {
                exit_code = 0;
                break;
            }
            let current_instruction = &self.stack[self.stack_pointer];
            println!(
                "{}[VM]{}: Executing instruction {:?}",
                utils::Colors::Yellow,
                utils::Colors::Reset,
                current_instruction
            );
            match &current_instruction.instruction {
                Instructions::LDA(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => todo!(),
                    utils::AddressingValues::Immediate(rtype, value) => {
                        self.registers.A = (rtype.clone(), value.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.A = self.heap.get(e).unwrap().clone()
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => panic!("Illigal addressing value"),
                    utils::AddressingValues::IndirectB => {
                        self.registers.A = self.registers.B.clone();
                    }
                    utils::AddressingValues::IndirectC => {
                        self.registers.A = self.registers.C.clone();
                    }
                    utils::AddressingValues::IndirectX => {
                        self.registers.A = self.registers.X.clone();
                    }
                    utils::AddressingValues::IndirectY => {
                        self.registers.A = self.registers.Y.clone();
                    }
                },
                Instructions::LDB(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                    utils::AddressingValues::Immediate(rtype, value) => {
                        self.registers.B = (rtype.clone(), value.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.B = self.heap.get(e).unwrap().clone();
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => {
                        self.registers.B = self.registers.A.clone();
                    }
                    utils::AddressingValues::IndirectB => panic!("Illegal addressing value"),
                    utils::AddressingValues::IndirectC => {
                        self.registers.B = self.registers.C.clone();
                    }
                    utils::AddressingValues::IndirectX => {
                        self.registers.B = self.registers.X.clone();
                    }
                    utils::AddressingValues::IndirectY => {
                        self.registers.B = self.registers.Y.clone();
                    }
                },
                Instructions::LDC(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                    utils::AddressingValues::Immediate(rtype, value) => {
                        self.registers.C = (rtype.clone(), value.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.C = self.heap.get(e).unwrap().clone();
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => {
                        self.registers.C = self.registers.A.clone();
                    }
                    utils::AddressingValues::IndirectB => {
                        self.registers.C = self.registers.B.clone();
                    }
                    utils::AddressingValues::IndirectC => panic!("Illegal addressing value"),
                    utils::AddressingValues::IndirectX => {
                        self.registers.C = self.registers.X.clone();
                    }
                    utils::AddressingValues::IndirectY => {
                        self.registers.C = self.registers.Y.clone();
                    }
                },
                Instructions::LDX(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                    utils::AddressingValues::Immediate(rtype, value) => {
                        self.registers.X = (rtype.clone(), value.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.X = self.heap.get(e).unwrap().clone();
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => {
                        self.registers.X = self.registers.A.clone();
                    }
                    utils::AddressingValues::IndirectB => {
                        self.registers.X = self.registers.B.clone();
                    }
                    utils::AddressingValues::IndirectC => {
                        self.registers.X = self.registers.C.clone();
                    }
                    utils::AddressingValues::IndirectX => panic!("Illegal addressing value"),
                    utils::AddressingValues::IndirectY => {
                        self.registers.X = self.registers.Y.clone();
                    }
                },
                Instructions::LDY(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                    utils::AddressingValues::Immediate(rtype, value) => {
                        self.registers.Y = (rtype.clone(), value.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.Y = self.heap.get(e).unwrap().clone();
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => {
                        self.registers.Y = self.registers.A.clone();
                    }
                    utils::AddressingValues::IndirectB => {
                        self.registers.Y = self.registers.B.clone();
                    }
                    utils::AddressingValues::IndirectC => {
                        self.registers.Y = self.registers.C.clone();
                    }
                    utils::AddressingValues::IndirectX => {
                        self.registers.X = self.registers.X.clone();
                    }
                    utils::AddressingValues::IndirectY => panic!("Illegal addressing value"),
                },
                Instructions::STA(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.heap.set(&self.stack_pointer, self.registers.A.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.heap.set(&e, self.registers.A.clone());
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::STB(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.heap.set(&self.stack_pointer, self.registers.B.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.heap.set(&e, self.registers.B.clone());
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::STC(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.heap.set(&self.stack_pointer, self.registers.C.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.heap.set(&e, self.registers.C.clone());
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::STX(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.heap.set(&self.stack_pointer, self.registers.X.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.heap.set(&e, self.registers.X.clone());
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::STY(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.heap.set(&self.stack_pointer, self.registers.Y.clone());
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.heap.set(&e, self.registers.Y.clone());
                    }
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::EQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, if b.1 == c.1 { 1 } else { 0 });
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::NE(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, if b.1 != c.1 { 1 } else { 0 });
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::GT(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, if b.1 > c.1 { 1 } else { 0 });
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::LT(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, if b.1 < c.1 { 1 } else { 0 });
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::GQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, if b.1 >= c.1 { 1 } else { 0 });
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::LQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, if b.1 <= c.1 { 1 } else { 0 });
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::AND(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 & c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::OR(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 | c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::ADD(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 + c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::SUB(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 - c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::MUL(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 * c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::EXP(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        panic!("Kernel panic: {} ^ {} not implemented", b.1, c.1);
                        //self.registers.A = (Types::Integer, isize::pow(b.1, c.1));
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::DIV(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 / c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::MOD(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Integer, b.1 % c.1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::INC(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.registers.A = (Types::Integer, self.registers.A.1 + 1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::DEC(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        self.registers.A = (Types::Integer, self.registers.A.1 - 1);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::JMP(_) => todo!(),
                Instructions::CALL(_) => todo!(),
                Instructions::RET(_) => todo!(),
                Instructions::AOL(_) => todo!(),
                Instructions::PUSHA(_) => todo!(),
                Instructions::LEN(_) => todo!(),
                Instructions::A2I(_) => todo!(),
                Instructions::A2F(_) => todo!(),
                Instructions::A2D(_) => todo!(),
                Instructions::A2B(_) => todo!(),
                Instructions::A2S(_) => todo!(),
                Instructions::A2C(_) => todo!(),
                Instructions::A2O(_) => todo!(),
                Instructions::JMPA(_) => todo!(),
                Instructions::POPS(_) => todo!(),
                Instructions::ACP(_) => todo!(),
            }
            self.stack_pointer += 1;
        }
        println!(
            "{}[VM]{}: Thread {}'{}'{} finished with exit code {}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            utils::Colors::Cyan,
            self.id,
            utils::Colors::Reset,
            exit_code,
        );
        exit_code
    }
}
