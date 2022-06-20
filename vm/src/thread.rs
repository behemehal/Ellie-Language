use std::sync::Mutex;

use crate::{
    heap,
    program::ReadInstruction,
    utils::{self, ExitCode, Instructions, ThreadExit, ThreadPanic, ThreadPanicReason, Types},
};

pub struct Registers {
    pub A: (Types, Vec<u8>),
    pub B: (Types, Vec<u8>),
    pub C: (Types, Vec<u8>),
    pub X: (Types, Vec<u8>),
    pub Y: (Types, Vec<u8>),
}

#[derive(Debug, Clone)]
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
    pub arch: u8,
}

impl<'a> Thread<'a> {
    pub fn new(
        id: u32,
        arch: u8,
        program: &'a Vec<ReadInstruction>,
        heap: &'a mut heap::Heap,
    ) -> Self {
        Thread {
            id,
            program,
            heap,
            arch,
            stack: StackController::new(),
            registers: Registers {
                A: (Types::Void, vec![]),
                B: (Types::Void, vec![]),
                C: (Types::Void, vec![]),
                X: (Types::Void, vec![]),
                Y: (Types::Void, vec![]),
            },
        }
    }

    pub fn run(&mut self) -> ThreadExit {
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

        let mut thread_exit = ThreadExit::Complete;
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
                return ThreadExit::Complete;
            }

            //Borrow self.stack with mutex

            let mut drop_current_stack = false;
            let current_stack = self.stack.last_mut().unwrap();

            if current_stack.pos >= self.program.len() {
                println!(
                    "{}[VM]{}: Thread {}'{}'{} halted: Out of instructions",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset
                );
                return ThreadExit::OutOfInstructions;
            }

            let current_instruction = &self.program[current_stack.pos];
            println!(
                "{}[VM]{}: Executing instruction {:?} : {}",
                utils::Colors::Yellow,
                utils::Colors::Reset,
                current_instruction,
                current_stack.pos
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
                        self.registers.A = (Types::Bool, vec![if b.1 == c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::NE(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Bool, vec![if b.1 != c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::GT(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Bool, vec![if b.1 > c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::LT(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Bool, vec![if b.1 < c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::GQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Bool, vec![if b.1 >= c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::LQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = (Types::Bool, vec![if b.1 <= c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::AND(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let and = b.0 == Types::Bool && c.0 == Types::Bool;
                        let and = if and {
                            let b: bool = b.1.first().unwrap().clone() == 1_u8;
                            let c: bool = c.1.first().unwrap().clone() == 1_u8;
                            b && c
                        } else {
                            false
                        };
                        self.registers.A = (Types::Bool, vec![if b.1 <= c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::OR(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let or = b.0 == Types::Bool && c.0 == Types::Bool;
                        let or = if or {
                            let b: bool = b.1.first().unwrap().clone() == 1_u8;
                            let c: bool = c.1.first().unwrap().clone() == 1_u8;
                            b || c
                        } else {
                            false
                        };
                        self.registers.A = (Types::Bool, vec![if b.1 <= c.1 { 1 } else { 0 }]);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::ADD(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let arch_size = self.arch / 8;

                        let result = match (b.0, c.0) {
                            (Types::Integer, Types::Integer) => {
                                let b_value =
                                    usize::from_le_bytes(b.1[0..b.1.len()].try_into().unwrap());
                                let c_value =
                                    usize::from_le_bytes(c.1[0..c.1.len()].try_into().unwrap());
                                let result = match b_value.checked_add(c_value) {
                                    Some(e) => e,
                                    None => {
                                        return ThreadExit::Panic(ThreadPanic {
                                            reason: ThreadPanicReason::IntegerOverflow,
                                            stack_trace: self.stack.stack.clone(),
                                        });
                                    }
                                };
                                //Check emulated platform overflow
                                if self.arch == 16 && result > 0xffff {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                    });
                                } else if self.arch == 32 && result > 0xffff_ffff {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                    });
                                }
                                self.registers.A = (Types::Integer, result.to_le_bytes().to_vec());
                            }
                            (Types::Float, Types::Integer) => todo!(),
                            (Types::Float, Types::Float) => todo!(),
                            (Types::Double, Types::Integer) => todo!(),
                            (Types::Double, Types::Double) => todo!(),
                            (Types::Byte, Types::Byte) => todo!(),
                            (Types::String, Types::Integer) => {
                                let b_value = String::from_utf8(b.1.clone()).unwrap();
                                let c_value =
                                    usize::from_le_bytes(c.1[0..c.1.len()].try_into().unwrap())
                                        .to_string();
                                let result = b_value + &c_value;
                                self.registers.A = (Types::String, result.bytes().collect());
                            }
                            (Types::String, Types::Float) => todo!(),
                            (Types::String, Types::Double) => todo!(),
                            (Types::String, Types::Byte) => todo!(),
                            (Types::String, Types::Bool) => todo!(),
                            (Types::String, Types::String) => todo!(),
                            (Types::String, Types::Char) => todo!(),
                            _ => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::UnmergebleTypes,
                                    stack_trace: self.stack.stack.clone(),
                                });
                            }
                        };
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::SUB(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();

                        let b_value = usize::from_le_bytes(b.1[0..8].try_into().unwrap());
                        let c_value = usize::from_le_bytes(c.1[0..8].try_into().unwrap());
                        let result = match b_value.checked_sub(c_value) {
                            Some(e) => e,
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                })
                            }
                        };

                        self.registers.A = (Types::Integer, result.to_le_bytes().to_vec());
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::MUL(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let b_value = usize::from_le_bytes(b.1[0..8].try_into().unwrap());
                        let c_value = usize::from_le_bytes(c.1[0..8].try_into().unwrap());

                        let result = match b_value.checked_mul(c_value) {
                            Some(e) => e,
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                });
                            }
                        };

                        //Check emulated platform overflow
                        if self.arch == 16 && result > 0xffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                            });
                        } else if self.arch == 32 && result > 0xffff_ffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                            });
                        }

                        self.registers.A = (Types::Integer, result.to_le_bytes().to_vec());
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::EXP(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let b_value = usize::from_le_bytes(b.1[0..8].try_into().unwrap());
                        let c_value = usize::from_le_bytes(c.1[0..8].try_into().unwrap());
                        //TODO
                        panic!("Exponentiation not implemented");
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::DIV(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let b_value = usize::from_le_bytes(b.1[0..8].try_into().unwrap());
                        let c_value = usize::from_le_bytes(c.1[0..8].try_into().unwrap());
                        let result = match b_value.checked_div(c_value) {
                            Some(e) => e,
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                });
                            }
                        };

                        //Check emulated platform overflow
                        if self.arch == 16 && result > 0xffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                            });
                        } else if self.arch == 32 && result > 0xffff_ffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                            });
                        }

                        self.registers.A = (Types::Integer, result.to_le_bytes().to_vec());
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::MOD(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        panic!("Modulo not implemented");
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::INC(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        panic!("Increment not implemented");
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::DEC(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        panic!("Decrement not implemented");
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
                                    println!(
                                        "{}[VM]{} Might be Stack overflow",
                                        utils::Colors::Red,
                                        utils::Colors::Reset
                                    );
                                    //return ThreadExit::StackOverflow;
                                }
                            };
                            continue;
                        }
                        _ => panic!("Illegal addressing value"),
                    }
                }
                Instructions::CALLN(e) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Absolute(stack_pos) => {
                        println!(
                            "{}[VM]{} Native Call: {}",
                            utils::Colors::Yellow,
                            utils::Colors::Reset,
                            stack_pos
                        );
                    }
                    _ => panic!("Illegal addressing value"),
                },
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
    }
}
