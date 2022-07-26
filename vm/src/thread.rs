#![allow(non_snake_case)]
use ellie_core::{
    defs::{PlatformArchitecture, VmNativeAnswer, VmNativeCall},
    raw_type::RawType,
};

use crate::{
    heap,
    program::ReadInstruction,
    utils::{self, Instructions, ThreadExit, ThreadPanic, ThreadPanicReason, ThreadStepInfo},
};

pub struct Registers {
    pub A: RawType,
    pub B: RawType,
    pub C: RawType,
    pub X: RawType,
    pub Y: RawType,
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
        if self.stack.len() > 0
            && self
                .stack
                .iter()
                .filter(|x| x.name == stack.name)
                .collect::<Vec<_>>()
                .len()
                > 7
        {
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

#[derive(Debug, Clone)]
pub struct ThreadInfo {
    pub id: usize,
    pub stack_id: usize,
    pub stack_name: String,
    pub stack_caller: Option<usize>,
    pub stack_pos: usize,
}

pub struct Thread<'a, T> {
    pub id: usize,
    pub program: &'a Vec<ReadInstruction>,
    pub heap: &'a mut heap::Heap,
    pub registers: Registers,
    pub stack: StackController,
    pub arch: PlatformArchitecture,
    pub(crate) native_call_channel: T,
}

impl<'a, T> Thread<'a, T>
where
    T: Fn(ThreadInfo, VmNativeCall) -> VmNativeAnswer + Clone + Sized,
{
    pub fn new(
        id: usize,
        arch: PlatformArchitecture,
        program: &'a Vec<ReadInstruction>,
        heap: &'a mut heap::Heap,
        native_call_channel: T,
    ) -> Self {
        Thread {
            id,
            program,
            heap,
            arch,
            stack: StackController::new(),
            registers: Registers {
                A: RawType::void(),
                B: RawType::void(),
                C: RawType::void(),
                X: RawType::void(),
                Y: RawType::void(),
            },
            native_call_channel,
        }
    }

    pub fn step(&mut self) -> Result<ThreadStepInfo, ThreadExit> {
        if self.stack.len() == 0 {
            return Err(ThreadExit::ExitGracefully);
        }

        let mut drop_current_stack = false;
        let current_stack = self.stack.last_mut().unwrap();

        if current_stack.pos >= self.program.len() {
            println!("{}-{}", current_stack.pos, self.program.len(),);
            return Err(ThreadExit::Panic(ThreadPanic {
                reason: ThreadPanicReason::OutOfInstructions,
                stack_trace: self.stack.stack.clone(),
                code_location: format!("{}:{}", file!(), line!()),
            }));
        }

        let current_instruction = &self.program[current_stack.pos];
        match &current_instruction.instruction {
            Instructions::LDA(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => todo!(),
                utils::AddressingValues::Immediate(raw_type) => {
                    self.registers.A = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    self.registers.A = match self.heap.get(e) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
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
            Instructions::LDB(e) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => unreachable!(
                    "Illegal addressing value: {:?} {:?}",
                    current_instruction, e
                ),
                utils::AddressingValues::Immediate(raw_type) => {
                    self.registers.B = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    self.registers.B = match self.heap.get(e) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
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
                utils::AddressingValues::Immediate(raw_type) => {
                    self.registers.C = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    self.registers.C = match self.heap.get(e) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
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
                utils::AddressingValues::Immediate(raw_type) => {
                    self.registers.X = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    self.registers.X = match self.heap.get(e) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
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
                utils::AddressingValues::Immediate(raw_type) => {
                    self.registers.Y = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    self.registers.Y = match self.heap.get(e) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
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
                    self.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::NE(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    self.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::GT(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    self.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::LT(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    self.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::GQ(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    self.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::LQ(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    self.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::AND(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    let and = if c.is_bool() && b.is_bool() {
                        let b: bool = b.data.first().unwrap() == &1_u8;
                        let c: bool = c.data.first().unwrap() == &1_u8;
                        b && c
                    } else {
                        false
                    };
                    self.registers.A = RawType::bool(and);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::OR(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    let and = if c.is_bool() && b.is_bool() {
                        let b: bool = b.data.first().unwrap() == &1_u8;
                        let c: bool = c.data.first().unwrap() == &1_u8;
                        b || c
                    } else {
                        false
                    };
                    self.registers.A = RawType::bool(and);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::ADD(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let result = match b_value.checked_add(c_value) {
                                Some(e) => e,
                                None => {
                                    return Err(ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    }));
                                }
                            };
                            //Check emulated platform overflow
                            if self.arch.is_16() && result > 0xffff {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            } else if self.arch.is_32() && result > 0xffff_ffff {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                            self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                        }
                        //Float + int
                        (2, 1) => todo!(),
                        //Float + float
                        (2, 2) => todo!(),
                        // Double + integer
                        (3, 1) => todo!(),
                        // Double + Double
                        (3, 3) => todo!(),
                        // Byte + Byte
                        (4, 4) => todo!(),
                        // String + Integer
                        (6, 1) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value =
                                isize::from_le_bytes(c.data.try_into().unwrap()).to_string();
                            let result = b_value + &c_value;
                            self.registers.A = RawType::string(result.bytes().collect());
                        }
                        //String + Float
                        (6, 2) => todo!(),
                        //String + Double
                        (6, 3) => todo!(),
                        // String + Byte
                        (6, 4) => todo!(),
                        // String + Bool
                        (6, 5) => todo!(),
                        // String + String
                        (6, 6) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = String::from_utf8(c.data).unwrap();
                            let result = b_value + &c_value;
                            self.registers.A = RawType::string(result.bytes().collect());
                        }
                        // String + Char
                        (6, 7) => todo!(),
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::SUB(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();

                    let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                    let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                    let result = match b_value.checked_sub(c_value) {
                        Some(e) => e,
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };

                    self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::MUL(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                    let c_value = isize::from_le_bytes(c.data.try_into().unwrap());

                    let result = match b_value.checked_mul(c_value) {
                        Some(e) => e,
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };

                    //Check emulated platform overflow
                    if self.arch.is_16() && result > 0xffff {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IntegerOverflow,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    } else if self.arch.is_32() && result > 0xffff_ffff {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IntegerOverflow,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::EXP(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    let _b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                    let _c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                    //TODO
                    panic!("Exponentiation not implemented");
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::DIV(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = self.registers.B.clone();
                    let c = self.registers.C.clone();
                    let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                    let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                    let result = match b_value.checked_div(c_value) {
                        Some(e) => e,
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };

                    //Check emulated platform overflow
                    if self.arch.is_16() && result > 0xffff {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IntegerOverflow,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    } else if self.arch.is_32() && result > 0xffff_ffff {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IntegerOverflow,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::MOD(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let _b = self.registers.B.clone();
                    let _c = self.registers.C.clone();
                    panic!("Mod not implemented");
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
            Instructions::JMP(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Absolute(e) => {
                    current_stack.pos = e;
                    return Ok(ThreadStepInfo::JMP(e));
                }
                _ => unreachable!("Illegal addressing value"),
            },
            Instructions::CALL(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Absolute(stack_pos) => {
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
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::StackOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                    return Ok(ThreadStepInfo::CALL(*stack_pos));
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::CALLN(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Immediate(target) => {
                    let location = String::from_utf8(target.data.clone()).unwrap();
                    let module = location.split(">").collect::<Vec<_>>()[0];
                    let fn_name = location.split(">").collect::<Vec<_>>()[1]
                        .split(":")
                        .collect::<Vec<_>>()[0];
                    let number_of_params = location.split(":").collect::<Vec<_>>()[1]
                        .parse::<usize>()
                        .unwrap();
                    let raw_params = {
                        let mut params = Vec::new();
                        for i in 0..number_of_params {
                            params.push(match self.heap.get(&(current_stack.pos - (i + 1))) {
                                Some(e) => e.clone(),
                                None => panic!(
                                    "[VM] Parameter {} not found",
                                    &(current_stack.pos - (i + 1))
                                ),
                            });
                        }
                        params
                    };
                    let native_call = VmNativeCall {
                        module: module.to_string(),
                        name: fn_name.to_string(),
                        params: raw_params,
                    };

                    self.registers.A = match (self.native_call_channel)(
                        ThreadInfo {
                            id: self.id,
                            stack_id: current_stack.id,
                            stack_name: current_stack.name.clone(),
                            stack_caller: current_stack.caller.clone(),
                            stack_pos: current_stack.pos,
                        },
                        native_call,
                    ) {
                        VmNativeAnswer::Ok(e) => e,
                        VmNativeAnswer::RuntimeError(e) => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::RuntimeError(e),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                    drop_current_stack = true
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::RET(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    drop_current_stack = true;
                }
                utils::AddressingValues::Immediate(_) => todo!(),
                utils::AddressingValues::Absolute(_) => todo!(),
                utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::IndirectA => todo!(),
                utils::AddressingValues::IndirectB => todo!(),
                utils::AddressingValues::IndirectC => todo!(),
                utils::AddressingValues::IndirectX => todo!(),
                utils::AddressingValues::IndirectY => todo!(),
            },
            Instructions::AOL(_) => {
                #[cfg(feature = "debug")]
                println!(
                    "{}[VM]{} Ignore aol: {}",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    match current_instruction.addressing_value {
                        utils::AddressingValues::Absolute(e) => e,
                        _ => unreachable!("Wrong op-code"),
                    }
                );
            }
            Instructions::PUSHA(_) => todo!(),
            Instructions::LEN(_) => todo!(),
            Instructions::A2I(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    1 => (),
                    2 => {
                        let data = self.registers.A.to_float();
                        self.registers.A = RawType::integer((data as isize).to_le_bytes().to_vec());
                    }
                    3 => {
                        let data = self.registers.A.to_double();
                        self.registers.A = RawType::integer((data as isize).to_le_bytes().to_vec());
                    }
                    4 => {
                        let data = self.registers.A.to_byte();
                        self.registers.A = RawType::integer((data as isize).to_le_bytes().to_vec());
                    }
                    5 => {
                        let data = if self.registers.A.to_bool() {
                            1_isize
                        } else {
                            0_isize
                        };
                        self.registers.A = RawType::integer(data.to_le_bytes().to_vec());
                    }
                    _ => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                _ => panic!("Illegal addressing value"),
            },
            Instructions::A2F(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    1 => {
                        let data = self.registers.A.to_int();
                        self.registers.A = RawType::float((data as f32).to_le_bytes().to_vec());
                    }
                    2 => (),
                    3 => {
                        let data = self.registers.A.to_double();
                        self.registers.A = RawType::float((data as f32).to_le_bytes().to_vec());
                    }
                    4 => {
                        let data = self.registers.A.to_byte();
                        self.registers.A = RawType::float((data as f32).to_le_bytes().to_vec());
                    }
                    _ => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                _ => panic!("Illegal addressing value"),
            },
            Instructions::A2D(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    1 => {
                        let data = self.registers.A.to_int();
                        self.registers.A = RawType::double((data as f64).to_le_bytes().to_vec());
                    }
                    2 => {
                        let data = self.registers.A.to_float();
                        self.registers.A = RawType::double((data as f64).to_le_bytes().to_vec());
                    }
                    3 => (),
                    4 => {
                        let data = self.registers.A.to_byte();
                        self.registers.A = RawType::double((data as f64).to_le_bytes().to_vec());
                    }
                    _ => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                _ => panic!("Illegal addressing value"),
            },
            Instructions::A2B(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    1 => {
                        let data = self.registers.A.to_int();
                        if data < 255 {
                            self.registers.A = RawType::byte(data as u8);
                        } else {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                    2 => {
                        let data = self.registers.A.to_float();
                        self.registers.A = RawType::byte(if data.is_sign_negative() {
                            data.to_be_bytes()[0]
                        } else {
                            data.to_le_bytes()[0]
                        });
                    }
                    3 => {
                        let data = self.registers.A.to_double();
                        self.registers.A = RawType::byte(if data.is_sign_negative() {
                            data.to_be_bytes()[0]
                        } else {
                            data.to_le_bytes()[0]
                        });
                    }
                    4 => (),
                    5 => {
                        let data = self.registers.A.to_bool();
                        self.registers.A = RawType::byte(if data { 1 } else { 0 });
                    }
                    _ => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                _ => panic!("Illegal addressing value"),
            },
            Instructions::A2S(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    1 => {
                        let data = self.registers.A.to_int();
                        self.registers.A = RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    2 => {
                        let data = self.registers.A.to_float();
                        self.registers.A = RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    3 => {
                        let data = self.registers.A.to_double();
                        self.registers.A = RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    4 => {
                        let data = self.registers.A.to_byte();
                        self.registers.A = RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    5 => {
                        let data = self.registers.A.to_bool();
                        self.registers.A = RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    6 => (),
                    7 => {
                        let data = self.registers.A.to_char();
                        self.registers.A = RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    8 => {
                        self.registers.A = RawType::string("void".as_bytes().to_vec());
                    }
                    10 => {
                        self.registers.A = RawType::string("null".as_bytes().to_vec());
                    }
                    _ => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                _ => panic!("Illegal addressing value"),
            },
            Instructions::A2C(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    7 => (),
                    6 => {
                        let data = self.registers.A.to_string().chars().collect::<Vec<_>>()[0];
                        self.registers.A = RawType::char((data as u32).to_le_bytes().to_vec());
                    }
                    _ => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                _ => panic!("Illegal addressing value"),
            },
            Instructions::A2O(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                    1 => {
                        let data = self.registers.A.to_int();
                        self.registers.A = RawType::bool(data.is_positive());
                    }
                    2 => {
                        let data = self.registers.A.to_float();
                        self.registers.A = RawType::bool(data.is_sign_positive());
                    }
                    3 => {
                        let data = self.registers.A.to_double();
                        self.registers.A = RawType::bool(data.is_sign_negative());
                    }
                    4 => {
                        self.registers.A = RawType::bool(true);
                    }
                    5 => (),
                    6 => {
                        let data = self.registers.A.to_string();
                        self.registers.A = RawType::bool(data.len() > 0);
                    }
                    7 => {
                        let data = self.registers.A.to_char();
                        self.registers.A = RawType::bool(data != '\0');
                    }
                    8 => {
                        self.registers.A = RawType::bool(false);
                    }
                    10 => {
                        self.registers.A = RawType::bool(false);
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!("Illegal addressing value"),
            },
            Instructions::JMPA(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Absolute(e) => {
                    if self.registers.A.is_bool() {
                        if self.registers.A.data[0] == 1 {
                            current_stack.pos = e;
                            return Ok(ThreadStepInfo::JMP(e));
                        }
                    } else {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnexpectedType,
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                }
                _ => unreachable!("Illegal addressing value"),
            },
            Instructions::POPS(_) => todo!(),
            Instructions::ACP(_) => todo!(),
            Instructions::BRK(_) => todo!(),
        }
        if drop_current_stack {
            #[cfg(feature = "debug")]
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

        Ok(ThreadStepInfo::StepNext)
    }

    pub fn run(&mut self) -> ThreadExit {
        #[cfg(feature = "debug")]
        println!(
            "{}[VM]{}: Running thread {}'{}'{}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            utils::Colors::Cyan,
            self.id,
            utils::Colors::Reset,
        );
        #[cfg(feature = "debug")]
        println!(
            "{}[VM]{}: Thread start at {}: {}",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            self.stack.last().unwrap().name,
            self.stack.last().unwrap().pos
        );

        loop {
            if self.stack.len() == 0 {
                #[cfg(feature = "debug")]
                println!(
                    "{}[VM]{}: Thread {}'{}'{} finished",
                    utils::Colors::Yellow,
                    utils::Colors::Reset,
                    utils::Colors::Cyan,
                    self.id,
                    utils::Colors::Reset
                );
                return ThreadExit::ExitGracefully;
            }

            let mut drop_current_stack = false;
            let current_stack = self.stack.last_mut().unwrap();

            if current_stack.pos >= self.program.len() {
                println!("{}-{}", current_stack.pos, self.program.len(),);
                return ThreadExit::Panic(ThreadPanic {
                    reason: ThreadPanicReason::OutOfInstructions,
                    stack_trace: self.stack.stack.clone(),
                    code_location: format!("{}:{}", file!(), line!()),
                });
            }

            let current_instruction = &self.program[current_stack.pos];
            #[cfg(feature = "debug")]
            println!(
                "{}[VM]{} {}({}){} : Exec '{:?}' at {}",
                utils::Colors::Yellow,
                utils::Colors::Reset,
                utils::Colors::Cyan,
                self.id,
                utils::Colors::Reset,
                current_instruction.instruction,
                current_stack.pos
            );
            match &current_instruction.instruction {
                Instructions::LDA(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => todo!(),
                    utils::AddressingValues::Immediate(raw_type) => {
                        self.registers.A = raw_type.clone();
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.A = match self.heap.get(e) {
                            Some(e) => e.clone(),
                            None => match &self.program[*e].addressing_value {
                                utils::AddressingValues::Immediate(e) => e.clone(),
                                _ => {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::MemoryAccessViolation(
                                            e.clone(),
                                            current_stack.pos,
                                        ),
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            },
                        };
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
                Instructions::LDB(e) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => unreachable!(
                        "Illegal addressing value: {:?} {:?}",
                        current_instruction, e
                    ),
                    utils::AddressingValues::Immediate(raw_type) => {
                        self.registers.B = raw_type.clone();
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.B = match self.heap.get(e) {
                            Some(e) => e.clone(),
                            None => match &self.program[*e].addressing_value {
                                utils::AddressingValues::Immediate(e) => e.clone(),
                                _ => {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::MemoryAccessViolation(
                                            e.clone(),
                                            current_stack.pos,
                                        ),
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            },
                        };
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
                    utils::AddressingValues::Immediate(raw_type) => {
                        self.registers.C = raw_type.clone();
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.C = match self.heap.get(e) {
                            Some(e) => e.clone(),
                            None => match &self.program[*e].addressing_value {
                                utils::AddressingValues::Immediate(e) => e.clone(),
                                _ => {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::MemoryAccessViolation(
                                            e.clone(),
                                            current_stack.pos,
                                        ),
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            },
                        };
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
                    utils::AddressingValues::Immediate(raw_type) => {
                        self.registers.X = raw_type.clone();
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.X = match self.heap.get(e) {
                            Some(e) => e.clone(),
                            None => match &self.program[*e].addressing_value {
                                utils::AddressingValues::Immediate(e) => e.clone(),
                                _ => {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::MemoryAccessViolation(
                                            e.clone(),
                                            current_stack.pos,
                                        ),
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            },
                        };
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
                    utils::AddressingValues::Immediate(raw_type) => {
                        self.registers.Y = raw_type.clone();
                    }
                    utils::AddressingValues::Absolute(e) => {
                        self.registers.Y = match self.heap.get(e) {
                            Some(e) => e.clone(),
                            None => match &self.program[*e].addressing_value {
                                utils::AddressingValues::Immediate(e) => e.clone(),
                                _ => {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::MemoryAccessViolation(
                                            e.clone(),
                                            current_stack.pos,
                                        ),
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            },
                        };
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
                        self.registers.A = RawType::bool(b == c);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::NE(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = RawType::bool(b == c);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::GT(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = RawType::bool(b == c);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::LT(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = RawType::bool(b == c);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::GQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = RawType::bool(b == c);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::LQ(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        self.registers.A = RawType::bool(b == c);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::AND(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let and = if c.is_bool() && b.is_bool() {
                            let b: bool = b.data.first().unwrap() == &1_u8;
                            let c: bool = c.data.first().unwrap() == &1_u8;
                            b && c
                        } else {
                            false
                        };
                        self.registers.A = RawType::bool(and);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::OR(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let and = if c.is_bool() && b.is_bool() {
                            let b: bool = b.data.first().unwrap() == &1_u8;
                            let c: bool = c.data.first().unwrap() == &1_u8;
                            b || c
                        } else {
                            false
                        };
                        self.registers.A = RawType::bool(and);
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::ADD(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        match (b.type_id.id, c.type_id.id) {
                            (1, 1) => {
                                let b_value =
                                    isize::from_le_bytes(b.data.clone().try_into().unwrap());
                                let c_value =
                                    isize::from_le_bytes(c.data.clone().try_into().unwrap());
                                let result = match b_value.checked_add(c_value) {
                                    Some(e) => e,
                                    None => {
                                        println!(
                                            "{:?}: {} - {:?}: {} - {} {:?}",
                                            b.data,
                                            b_value,
                                            c.data,
                                            c_value,
                                            current_stack.pos,
                                            current_instruction
                                        );
                                        return ThreadExit::Panic(ThreadPanic {
                                            reason: ThreadPanicReason::IntegerOverflow,
                                            stack_trace: self.stack.stack.clone(),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    }
                                };
                                //Check emulated platform overflow
                                if self.arch.is_16() && result > 0xffff {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                } else if self.arch.is_32() && result > 0xffff_ffff {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                                self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                            }
                            //Float + int
                            (2, 1) => todo!(),
                            //Float + float
                            (2, 2) => todo!(),
                            // Double + integer
                            (3, 1) => todo!(),
                            // Double + Double
                            (3, 3) => todo!(),
                            // Byte + Byte
                            (4, 4) => todo!(),
                            // String + Integer
                            (6, 1) => {
                                let b_value = String::from_utf8(b.data).unwrap();
                                let c_value =
                                    isize::from_le_bytes(c.data.try_into().unwrap()).to_string();
                                let result = b_value + &c_value;
                                self.registers.A = RawType::string(result.bytes().collect());
                            }
                            //String + Float
                            (6, 2) => todo!(),
                            //String + Double
                            (6, 3) => todo!(),
                            // String + Byte
                            (6, 4) => todo!(),
                            // String + Bool
                            (6, 5) => {
                                let b_value = String::from_utf8(b.data).unwrap();
                                let c_value = c.data.first().unwrap() == &1_u8;
                                let result = b_value + &c_value.to_string();
                                self.registers.A = RawType::string(result.bytes().collect());
                            }
                            // String + String
                            (6, 6) => {
                                let b_value = String::from_utf8(b.data).unwrap();
                                let c_value = String::from_utf8(c.data).unwrap();
                                let result = b_value + &c_value;
                                self.registers.A = RawType::string(result.bytes().collect());
                            }
                            // String + Char
                            (6, 7) => todo!(),
                            _ => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::UnmergebleTypes,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
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

                        let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                        let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                        let result = match b_value.checked_sub(c_value) {
                            Some(e) => e,
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                })
                            }
                        };

                        self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::MUL(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                        let c_value = isize::from_le_bytes(c.data.try_into().unwrap());

                        let result = match b_value.checked_mul(c_value) {
                            Some(e) => e,
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        };

                        //Check emulated platform overflow
                        if self.arch.is_16() && result > 0xffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        } else if self.arch.is_32() && result > 0xffff_ffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }

                        self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::EXP(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let _b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                        let _c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                        //TODO
                        panic!("Exponentiation not implemented");
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::DIV(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let b = self.registers.B.clone();
                        let c = self.registers.C.clone();
                        let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                        let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                        let result = match b_value.checked_div(c_value) {
                            Some(e) => e,
                            None => {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        };

                        //Check emulated platform overflow
                        if self.arch.is_16() && result > 0xffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        } else if self.arch.is_32() && result > 0xffff_ffff {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }

                        self.registers.A = RawType::integer(result.to_le_bytes().to_vec());
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::MOD(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        let _b = self.registers.B.clone();
                        let _c = self.registers.C.clone();
                        panic!("Mod not implemented");
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
                Instructions::JMP(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Absolute(e) => {
                        current_stack.pos = e;
                        continue;
                    }
                    _ => unreachable!("Illegal addressing value"),
                },
                Instructions::CALL(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Absolute(stack_pos) => {
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
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::StackOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        };
                        continue;
                    }
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::CALLN(_) => {
                    match &current_instruction.addressing_value {
                        utils::AddressingValues::Immediate(target) => {
                            let location = String::from_utf8(target.data.clone()).unwrap();
                            let module = location.split(">").collect::<Vec<_>>()[0];
                            let fn_name = location.split(">").collect::<Vec<_>>()[1]
                                .split(":")
                                .collect::<Vec<_>>()[0];
                            let number_of_params = location.split(":").collect::<Vec<_>>()[1]
                                .parse::<usize>()
                                .unwrap();
                            let raw_params = {
                                let mut params = Vec::new();
                                for i in 0..number_of_params {
                                    params.push(match self.heap.get(&(current_stack.pos - (i + 1))) {
                                    Some(e) => e.clone(),
                                    None => {
                                        return ThreadExit::Panic(ThreadPanic {
                                            reason: ThreadPanicReason::ParemeterMemoryAccessViolation(current_stack.pos - (i + 1)),
                                            stack_trace: self.stack.stack.clone(),
                code_location: format!("{}:{}", file!(), line!()),
                                        });
                                    }
                                });
                                }
                                params
                            };
                            let native_call = VmNativeCall {
                                module: module.to_string(),
                                name: fn_name.to_string(),
                                params: raw_params,
                            };

                            self.registers.A = match (self.native_call_channel)(
                                ThreadInfo {
                                    id: self.id,
                                    stack_id: current_stack.id,
                                    stack_name: current_stack.name.clone(),
                                    stack_caller: current_stack.caller.clone(),
                                    stack_pos: current_stack.pos,
                                },
                                native_call,
                            ) {
                                VmNativeAnswer::Ok(e) => e,
                                VmNativeAnswer::RuntimeError(e) => {
                                    return ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::RuntimeError(e),
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    });
                                }
                            };
                            drop_current_stack = true
                        }
                        _ => panic!("Illegal addressing value"),
                    }
                }
                Instructions::RET(_) => match &current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => {
                        drop_current_stack = true;
                    }
                    utils::AddressingValues::Immediate(_) => todo!(),
                    utils::AddressingValues::Absolute(_) => todo!(),
                    utils::AddressingValues::AbsoluteIndex(_, _) => todo!(),
                    utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                    utils::AddressingValues::IndirectA => todo!(),
                    utils::AddressingValues::IndirectB => todo!(),
                    utils::AddressingValues::IndirectC => todo!(),
                    utils::AddressingValues::IndirectX => todo!(),
                    utils::AddressingValues::IndirectY => todo!(),
                },
                Instructions::AOL(_) => {
                    #[cfg(feature = "debug")]
                    println!(
                        "{}[VM]{} Ignore aol: {}",
                        utils::Colors::Yellow,
                        utils::Colors::Reset,
                        match current_instruction.addressing_value {
                            utils::AddressingValues::Absolute(e) => e,
                            _ => unreachable!("Wrong op-code"),
                        }
                    );
                }
                Instructions::PUSHA(_) => todo!(),
                Instructions::LEN(_) => todo!(),
                Instructions::A2I(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        1 => (),
                        2 => {
                            let data = self.registers.A.to_float();
                            self.registers.A =
                                RawType::integer((data as isize).to_le_bytes().to_vec());
                        }
                        3 => {
                            let data = self.registers.A.to_double();
                            self.registers.A =
                                RawType::integer((data as isize).to_le_bytes().to_vec());
                        }
                        4 => {
                            let data = self.registers.A.to_byte();
                            self.registers.A =
                                RawType::integer((data as isize).to_le_bytes().to_vec());
                        }
                        5 => {
                            let data = if self.registers.A.to_bool() {
                                1_isize
                            } else {
                                0_isize
                            };
                            self.registers.A = RawType::integer(data.to_le_bytes().to_vec());
                        }
                        _ => {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    },
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::A2F(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        1 => {
                            let data = self.registers.A.to_int();
                            self.registers.A = RawType::float((data as f32).to_le_bytes().to_vec());
                        }
                        2 => (),
                        3 => {
                            let data = self.registers.A.to_double();
                            self.registers.A = RawType::float((data as f32).to_le_bytes().to_vec());
                        }
                        4 => {
                            let data = self.registers.A.to_byte();
                            self.registers.A = RawType::float((data as f32).to_le_bytes().to_vec());
                        }
                        _ => {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    },
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::A2D(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        1 => {
                            let data = self.registers.A.to_int();
                            self.registers.A =
                                RawType::double((data as f64).to_le_bytes().to_vec());
                        }
                        2 => {
                            let data = self.registers.A.to_float();
                            self.registers.A =
                                RawType::double((data as f64).to_le_bytes().to_vec());
                        }
                        3 => (),
                        4 => {
                            let data = self.registers.A.to_byte();
                            self.registers.A =
                                RawType::double((data as f64).to_le_bytes().to_vec());
                        }
                        _ => {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    },
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::A2B(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        1 => {
                            let data = self.registers.A.to_int();
                            if data < 255 {
                                self.registers.A = RawType::byte(data as u8);
                            } else {
                                return ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                });
                            }
                        }
                        2 => {
                            let data = self.registers.A.to_float();
                            self.registers.A = RawType::byte(if data.is_sign_negative() {
                                data.to_be_bytes()[0]
                            } else {
                                data.to_le_bytes()[0]
                            });
                        }
                        3 => {
                            let data = self.registers.A.to_double();
                            self.registers.A = RawType::byte(if data.is_sign_negative() {
                                data.to_be_bytes()[0]
                            } else {
                                data.to_le_bytes()[0]
                            });
                        }
                        4 => (),
                        5 => {
                            let data = self.registers.A.to_bool();
                            self.registers.A = RawType::byte(if data { 1 } else { 0 });
                        }
                        _ => {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    },
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::A2S(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        1 => {
                            let data = self.registers.A.to_int();
                            self.registers.A =
                                RawType::string(data.to_string().as_bytes().to_vec());
                        }
                        2 => {
                            let data = self.registers.A.to_float();
                            self.registers.A =
                                RawType::string(data.to_string().as_bytes().to_vec());
                        }
                        3 => {
                            let data = self.registers.A.to_double();
                            self.registers.A =
                                RawType::string(data.to_string().as_bytes().to_vec());
                        }
                        4 => {
                            let data = self.registers.A.to_byte();
                            self.registers.A =
                                RawType::string(data.to_string().as_bytes().to_vec());
                        }
                        5 => {
                            let data = self.registers.A.to_bool();
                            self.registers.A =
                                RawType::string(data.to_string().as_bytes().to_vec());
                        }
                        6 => (),
                        7 => {
                            let data = self.registers.A.to_char();
                            self.registers.A =
                                RawType::string(data.to_string().as_bytes().to_vec());
                        }
                        8 => {
                            self.registers.A = RawType::string("void".as_bytes().to_vec());
                        }
                        10 => {
                            self.registers.A = RawType::string("null".as_bytes().to_vec());
                        }
                        _ => {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    },
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::A2C(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        7 => (),
                        6 => {
                            let data = self.registers.A.to_string().chars().collect::<Vec<_>>()[0];
                            self.registers.A = RawType::char((data as u32).to_le_bytes().to_vec());
                        }
                        _ => {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    },
                    _ => panic!("Illegal addressing value"),
                },
                Instructions::A2O(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Implicit => match self.registers.A.type_id.id {
                        1 => {
                            let data = self.registers.A.to_int();
                            self.registers.A = RawType::bool(data.is_positive());
                        }
                        2 => {
                            let data = self.registers.A.to_float();
                            self.registers.A = RawType::bool(data.is_sign_positive());
                        }
                        3 => {
                            let data = self.registers.A.to_double();
                            self.registers.A = RawType::bool(data.is_sign_negative());
                        }
                        4 => {
                            self.registers.A = RawType::bool(true);
                        }
                        5 => (),
                        6 => {
                            let data = self.registers.A.to_string();
                            self.registers.A = RawType::bool(data.len() > 0);
                        }
                        7 => {
                            let data = self.registers.A.to_char();
                            self.registers.A = RawType::bool(data != '\0');
                        }
                        8 => {
                            self.registers.A = RawType::bool(false);
                        }
                        10 => {
                            self.registers.A = RawType::bool(false);
                        }
                        _ => unreachable!(),
                    },
                    _ => unreachable!("Illegal addressing value"),
                },
                Instructions::JMPA(_) => match current_instruction.addressing_value {
                    utils::AddressingValues::Absolute(e) => {
                        if self.registers.A.is_bool() {
                            if self.registers.A.data[0] == 1 {
                                current_stack.pos = e;
                                continue;
                            }
                            #[cfg(feature = "debug")]
                            println!(
                                "{}[VM]{} JMPA: {} - {} - {}",
                                utils::Colors::Yellow,
                                utils::Colors::Reset,
                                self.registers.A.data[0],
                                e,
                                current_stack.pos
                            );
                        } else {
                            return ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnexpectedType,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            });
                        }
                    }
                    _ => unreachable!("Illegal addressing value"),
                },
                Instructions::POPS(_) => todo!(),
                Instructions::ACP(_) => todo!(),
                Instructions::BRK(_) => todo!(),
            }
            if drop_current_stack {
                #[cfg(feature = "debug")]
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
