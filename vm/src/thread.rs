use std::sync::Mutex;

use crate::{
    heap,
    program::ReadInstruction,
    utils::{self, ExitCode, Instructions, Types},
};

pub struct Registers {
    pub A: (Types, isize),
    pub B: (Types, isize),
    pub C: (Types, isize),
    pub X: (Types, isize),
    pub Y: (Types, isize),
}

pub struct Stack {
    pub id: usize,
    pub name: String,
    pub caller: Option<usize>,
    pub pos: usize,
}

pub struct StackController {
    pub stack: Vec<Stack>,
}

impl StackController {
    pub fn new() -> StackController {
        StackController { stack: Vec::new() }
    }

    pub fn len(&self) -> usize {
        self.stack.len()
    }

    pub fn first(&self) -> Option<&Stack> {
        self.stack.first()
    }

    pub fn last(&self) -> Option<&Stack> {
        self.stack.last()
    }

    pub fn last_mut(&mut self) -> Option<&mut Stack> {
        self.stack.last_mut()
    }

    pub fn push(&mut self, stack: Stack) -> Result<(), u8> {
        if self.stack.len() > 0 && self.stack.last().unwrap().name == stack.name {
            Err(1)
        } else {
            self.stack.push(stack);
            Ok(())
        }
    }

    pub fn pop(&mut self) -> Stack {
        self.stack.pop().unwrap()
    }
}

pub struct Thread<'a> {
    pub id: u32,
    pub program: &'a Vec<ReadInstruction>,
    pub heap: &'a mut heap::Heap,
    pub registers: Registers,
    pub stack: StackController,
}

impl<'a> Thread<'a> {
    pub fn new(id: u32, program: &'a Vec<ReadInstruction>, heap: &'a mut heap::Heap) -> Self {
        Thread {
            id,
            program,
            heap,
            stack: StackController::new(),
            registers: Registers {
                A: (Types::Void, 0),
                B: (Types::Void, 0),
                C: (Types::Void, 0),
                X: (Types::Void, 0),
                Y: (Types::Void, 0),
            },
        }
    }

    pub fn run(&mut self) -> ExitCode {
        println!(
            "{}[VM]{}: Running thread {}'{}'{}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            utils::Colors::Cyan,
            self.id,
            utils::Colors::Reset,
        );
        println!(
            "{}[VM]{}: Thread start at {}: {}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            self.stack.last().unwrap().name,
            self.stack.last().unwrap().pos
        );

        let mut exit_code = ExitCode::Success;
        let last_stack = self.stack.last_mut().unwrap();

        loop {
            if self.stack.len() == 0 {
                println!(
                    "{}[VM]{}: Thread {}'{}'{} finished",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset
                );
                break;
            }

            //Borrow self.stack with mutex

            let mut drop_current_stack = false;
            let current_stack = self.stack.last_mut().unwrap();

            if current_stack.pos >= self.program.len() {
                exit_code = ExitCode::OutOfInstructions;
                println!(
                    "{}[VM]{}: Thread {}'{}'{} halted: Out of instructions",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset
                );
                break;
            }

            let current_instruction = &self.program[current_stack.pos];
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
                        self.heap.set(&current_stack.pos, self.registers.A.clone());
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
                        self.heap.set(&current_stack.pos, self.registers.B.clone());
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
                        self.heap.set(&current_stack.pos, self.registers.C.clone());
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
                        self.heap.set(&current_stack.pos, self.registers.X.clone());
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
                        self.heap.set(&current_stack.pos, self.registers.Y.clone());
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
                Instructions::CALL(e) => {
                    match &current_instruction.addressing_value {
                        utils::AddressingValues::Absolute(stack_pos) => {
                            println!(
                                "{}[VM]{} Push stack: {}",
                                utils::Colors::Yellow,
                                utils::Colors::Reset,
                                stack_pos
                            );
                            current_stack.pos += 1;
                            let current_stack_id = current_stack.id.clone();
                            match self.stack.push(Stack {
                                id: *stack_pos,
                                name: format!("fn<{}>", stack_pos),
                                caller: Some(current_stack_id),
                                pos: *stack_pos,
                            }) {
                                Ok(_) => (),
                                Err(_) => {
                                    exit_code = ExitCode::StackOverflow;
                                    break;
                                }
                            };
                            continue;
                        }
                        _ => panic!("Illegal addressing value"),
                    }

                    panic!("CALL : {:?}", current_instruction);
                }
                Instructions::RET(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        drop_current_stack = true;
                    }
                    utils::AddressingValues::Immediate(_, _) => todo!(),
                    utils::AddressingValues::Absolute(_) => todo!(),
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => todo!(),
                    utils::AddressingValues::IndirectB => todo!(),
                    utils::AddressingValues::IndirectC => todo!(),
                    utils::AddressingValues::IndirectX => todo!(),
                    utils::AddressingValues::IndirectY => todo!(),
                },
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
            if drop_current_stack {
                println!(
                    "{}[VM]{}: Dropping stack '{}'",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    current_stack.name
                );
                self.stack.pop();
            } else {
                current_stack.pos += 1;
            }
        }
        match exit_code {
            ExitCode::Success => {
                println!(
                    "{}[VM]{}: Thread {}'{}'{} exited gracefully",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset,
                )
            }
            ExitCode::OutOfInstructions => {
                println!(
                    "{}[VM]{}: Thread {}'{}'{} exited because of out of instructions",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset,
                )
            }
            ExitCode::StackOverflow => {
                println!(
                    "{}[VM]{}: Thread {}'{}'{} exited because of stack overflow",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset,
                )
            }
        }
        exit_code
    }
}
