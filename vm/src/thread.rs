#![allow(non_snake_case)]
use std::convert::TryInto;

use alloc::{
    format,
    string::{String, ToString},
    vec::Vec,
};
use ellie_core::{
    defs::{PlatformArchitecture, VmNativeAnswer, VmNativeCall},
    raw_type::RawType,
};

use crate::{
    heap::Heap,
    program::ReadInstruction,
    utils::{
        self, Instructions, ThreadExit, ThreadPanic, ThreadPanicReason, ThreadStep, ThreadStepInfo,
    },
};

#[derive(Debug, Clone)]
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
    pub stack_len: usize,
    pub caller: Option<usize>,
    pub registers: Registers,
    pub stack_pos: usize,
    pub frame_pos: usize,
}

#[derive(Debug, Clone)]
pub struct StackController {
    pub stack: Vec<Stack>,
}

impl StackController {
    pub fn new() -> StackController {
        StackController { stack: Vec::new() }
    }

    pub fn calculate_stack_length(&self) -> usize {
        let mut stack_len = 0;
        for stack in &self.stack {
            stack_len += stack.stack_len;
        }
        stack_len
    }

    pub fn get(&self, id: usize) -> Option<&Stack> {
        for stack in self.stack.iter() {
            if stack.id == id {
                return Some(stack);
            }
        }
        None
    }

    //This function finds current stack and registers its ret register(Y) to parent stack
    pub fn ret(&mut self, current_stack_id: usize) -> Result<(), u8> {
        match self.stack.iter().find(|x| x.id == current_stack_id) {
            Some(current_stack) => {
                let current_Y = current_stack.registers.Y.clone();
                match current_stack.caller {
                    Some(caller) => match self.stack.iter_mut().find(|x| x.id == caller) {
                        Some(caller_stack) => {
                            caller_stack.registers.Y = current_Y;
                            Ok(())
                        }
                        None => Err(0),
                    },
                    None => Err(1),
                }
            }
            None => Err(0),
        }
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
        /*
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
           
        }
        */
        self.stack.push(stack);
        Ok(())
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

pub struct Thread<T> {
    pub id: usize,
    pub program: Vec<ReadInstruction>,
    pub stack: StackController,
    pub arch: PlatformArchitecture,
    pub(crate) native_call_channel: T,
}

impl<T> Thread<T>
where
    T: FnMut(ThreadInfo, VmNativeCall) -> VmNativeAnswer + Clone + Sized,
{
    pub fn new(
        id: usize,
        arch: PlatformArchitecture,
        program: Vec<ReadInstruction>,
        native_call_channel: T,
    ) -> Self {
        Thread {
            id,
            program,
            arch,
            stack: StackController::new(),
            native_call_channel,
        }
    }

    pub fn step(&mut self, heap: &mut Heap) -> Result<ThreadStep, ThreadExit> {
        if self.stack.len() == 0 {
            return Err(ThreadExit::ExitGracefully);
        }

        let mut drop_current_stack = false;
        let current_stack = self.stack.last_mut().unwrap();

        if current_stack.stack_pos >= self.program.len() {
            return Err(ThreadExit::Panic(ThreadPanic {
                reason: ThreadPanicReason::OutOfInstructions,
                stack_trace: self.stack.stack.clone(),
                code_location: format!("{}:{}", file!(), line!()),
            }));
        }

        let current_instruction = &self.program[current_stack.stack_pos];

        match &current_instruction.instruction {
            Instructions::LDA(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => todo!(),
                utils::AddressingValues::Immediate(raw_type) => {
                    current_stack.registers.A = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    current_stack.registers.A = match heap.get(&(e + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if (pointers.len() - 1) < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.A =
                        match heap.get(&(pointed_location + current_stack.frame_pos)) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        pointed_location,
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        };
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::IndirectA => panic!("Illigal addressing value"),
                utils::AddressingValues::IndirectB => {
                    current_stack.registers.A = current_stack.registers.B.clone();
                }
                utils::AddressingValues::IndirectC => {
                    current_stack.registers.A = current_stack.registers.C.clone();
                }
                utils::AddressingValues::IndirectX => {
                    current_stack.registers.A = current_stack.registers.X.clone();
                }
                utils::AddressingValues::IndirectY => {
                    current_stack.registers.A = current_stack.registers.Y.clone();
                }
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.A = match heap.get(&pointed_location) {
                        Some(e) => e.clone(),
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::MemoryAccessViolation(
                                    pointed_location,
                                    current_stack.stack_pos,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                }
            },
            Instructions::LDB(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                utils::AddressingValues::Immediate(raw_type) => {
                    current_stack.registers.B = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    current_stack.registers.B = match heap.get(&(e + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if pointers.len() - 1 < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.B =
                        match heap.get(&(pointed_location + current_stack.frame_pos)) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        pointed_location,
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        };
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::IndirectA => {
                    current_stack.registers.B = current_stack.registers.A.clone();
                }
                utils::AddressingValues::IndirectB => panic!("Illegal addressing value"),
                utils::AddressingValues::IndirectC => {
                    current_stack.registers.B = current_stack.registers.C.clone();
                }
                utils::AddressingValues::IndirectX => {
                    current_stack.registers.B = current_stack.registers.X.clone();
                }
                utils::AddressingValues::IndirectY => {
                    current_stack.registers.B = current_stack.registers.Y.clone();
                }
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.B = match heap.get(&pointed_location) {
                        Some(e) => e.clone(),
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::MemoryAccessViolation(
                                    pointed_location,
                                    current_stack.stack_pos,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                }
            },
            Instructions::LDC(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                utils::AddressingValues::Immediate(raw_type) => {
                    current_stack.registers.C = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    current_stack.registers.C = match heap.get(&(e + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if pointers.len() - 1 < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.C =
                        match heap.get(&(pointed_location + current_stack.frame_pos)) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        pointed_location,
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        };
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::IndirectA => {
                    current_stack.registers.C = current_stack.registers.A.clone();
                }
                utils::AddressingValues::IndirectB => {
                    current_stack.registers.C = current_stack.registers.B.clone();
                }
                utils::AddressingValues::IndirectC => panic!("Illegal addressing value"),
                utils::AddressingValues::IndirectX => {
                    current_stack.registers.C = current_stack.registers.X.clone();
                }
                utils::AddressingValues::IndirectY => {
                    current_stack.registers.C = current_stack.registers.Y.clone();
                }
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.C = match heap.get(&pointed_location) {
                        Some(e) => e.clone(),
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::MemoryAccessViolation(
                                    pointed_location,
                                    current_stack.stack_pos,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                }
            },
            Instructions::LDX(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                utils::AddressingValues::Immediate(raw_type) => {
                    current_stack.registers.X = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    current_stack.registers.X = match heap.get(&(e + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if pointers.len() - 1 < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.X =
                        match heap.get(&(pointed_location + current_stack.frame_pos)) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        pointed_location,
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        };
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::IndirectA => {
                    current_stack.registers.X = current_stack.registers.A.clone();
                }
                utils::AddressingValues::IndirectB => {
                    current_stack.registers.X = current_stack.registers.B.clone();
                }
                utils::AddressingValues::IndirectC => {
                    current_stack.registers.X = current_stack.registers.C.clone();
                }
                utils::AddressingValues::IndirectX => panic!("Illegal addressing value"),
                utils::AddressingValues::IndirectY => {
                    current_stack.registers.X = current_stack.registers.Y.clone();
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::LDY(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => unreachable!("Illegal addressing value"),
                utils::AddressingValues::Immediate(raw_type) => {
                    current_stack.registers.Y = raw_type.clone();
                }
                utils::AddressingValues::Absolute(e) => {
                    current_stack.registers.Y = match heap.get(&(e + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*e].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        e.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if pointers.len() - 1 < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.Y =
                        match heap.get(&(pointed_location + current_stack.frame_pos)) {
                            Some(e) => e.clone(),
                            None => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        pointed_location,
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        };
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::IndirectA => {
                    current_stack.registers.Y = current_stack.registers.A.clone();
                }
                utils::AddressingValues::IndirectB => {
                    current_stack.registers.Y = current_stack.registers.B.clone();
                }
                utils::AddressingValues::IndirectC => {
                    current_stack.registers.Y = current_stack.registers.C.clone();
                }
                utils::AddressingValues::IndirectX => {
                    current_stack.registers.X = current_stack.registers.X.clone();
                }
                utils::AddressingValues::IndirectY => panic!("Illegal addressing value"),
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    current_stack.registers.Y = match heap.get(&pointed_location) {
                        Some(e) => e.clone(),
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::MemoryAccessViolation(
                                    pointed_location,
                                    current_stack.stack_pos,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    };
                }
            },
            Instructions::STA(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        current_stack.registers.A.clone(),
                    );
                }
                utils::AddressingValues::Immediate(raw_type) => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        raw_type.clone(),
                    );
                }
                utils::AddressingValues::Absolute(e) => {
                    heap.set(
                        &(e + current_stack.frame_pos),
                        current_stack.registers.A.clone(),
                    );
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(index.type_id.id),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if (pointers.len() - 1) < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());

                    heap.set(
                        &(pointed_location + current_stack.frame_pos),
                        current_stack.registers.A.clone(),
                    )
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    heap.set(&pointed_location, current_stack.registers.A.clone());
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::STB(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        current_stack.registers.B.clone(),
                    );
                }
                utils::AddressingValues::Immediate(raw_type) => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        raw_type.clone(),
                    );
                }
                utils::AddressingValues::Absolute(e) => {
                    heap.set(
                        &(e + current_stack.frame_pos),
                        current_stack.registers.B.clone(),
                    );
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if (pointers.len() - 1) < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());

                    heap.set(
                        &(pointed_location + current_stack.frame_pos),
                        current_stack.registers.B.clone(),
                    )
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    heap.set(&pointed_location, current_stack.registers.B.clone());
                }

                _ => panic!("Illegal addressing value"),
            },
            Instructions::STC(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        current_stack.registers.C.clone(),
                    );
                }
                utils::AddressingValues::Immediate(raw_type) => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        raw_type.clone(),
                    );
                }
                utils::AddressingValues::Absolute(e) => {
                    heap.set(
                        &(e + current_stack.frame_pos),
                        current_stack.registers.C.clone(),
                    );
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if (pointers.len() - 1) < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());

                    heap.set(
                        &(pointed_location + current_stack.frame_pos),
                        current_stack.registers.C.clone(),
                    )
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    heap.set(&pointed_location, current_stack.registers.C.clone());
                }

                _ => panic!("Illegal addressing value"),
            },
            Instructions::STX(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        current_stack.registers.X.clone(),
                    );
                }
                utils::AddressingValues::Immediate(raw_type) => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        raw_type.clone(),
                    );
                }
                utils::AddressingValues::Absolute(e) => {
                    heap.set(
                        &(e + current_stack.frame_pos),
                        current_stack.registers.X.clone(),
                    );
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if (pointers.len() - 1) < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());

                    heap.set(
                        &(pointed_location + current_stack.frame_pos),
                        current_stack.registers.X.clone(),
                    )
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                _ => panic!("Illegal addressing value"),
            },
            Instructions::STY(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        current_stack.registers.Y.clone(),
                    );
                }
                utils::AddressingValues::Immediate(raw_type) => {
                    heap.set(
                        &(current_stack.stack_pos + current_stack.frame_pos),
                        raw_type.clone(),
                    );
                }
                utils::AddressingValues::Absolute(e) => {
                    heap.set(
                        &(e + current_stack.frame_pos),
                        current_stack.registers.Y.clone(),
                    );
                }
                utils::AddressingValues::AbsoluteIndex(array_pointer, idx_pointer) => {
                    let array = match heap.get(&(array_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*array_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        array_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if array.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let index = match heap.get(&(idx_pointer + current_stack.frame_pos)) {
                        Some(e) => e.clone(),
                        None => match &self.program[*idx_pointer].addressing_value {
                            utils::AddressingValues::Immediate(e) => e.clone(),
                            _ => {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::MemoryAccessViolation(
                                        idx_pointer.clone(),
                                        current_stack.stack_pos,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        },
                    };

                    if index.type_id.id != 1 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexAccessViolation(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointers = array
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let idx = usize::from_le_bytes(index.data.clone().try_into().unwrap());

                    if (pointers.len() - 1) < idx {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::IndexOutOfBounds(idx),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }

                    let pointed_location = pointers[idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());

                    heap.set(
                        &(pointed_location + current_stack.frame_pos),
                        current_stack.registers.Y.clone(),
                    )
                }
                utils::AddressingValues::AbsoluteProperty(_, _) => todo!(),
                utils::AddressingValues::Parameter(idx) => {
                    let x_register_value = &current_stack.registers.X;
                    if x_register_value.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let pointers = x_register_value
                        .data
                        .chunks(self.arch.usize_len().into())
                        .collect::<Vec<_>>();
                    let pointed_location = pointers[*idx];
                    let pointed_location =
                        usize::from_le_bytes(pointed_location.clone().try_into().unwrap());
                    heap.set(&pointed_location, current_stack.registers.Y.clone());
                }

                _ => panic!("Illegal addressing value"),
            },
            Instructions::EQ(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    current_stack.registers.A = RawType::bool(b == c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::NE(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    current_stack.registers.A = RawType::bool(b != c);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::GT(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value > c_value);
                        }
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value > c_value);
                        }
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value > c_value);
                        }
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value > c_value);
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::LT(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value < c_value);
                        }
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value < c_value);
                        }
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value < c_value);
                        }
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value < c_value);
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::GQ(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value >= c_value);
                        }
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value >= c_value);
                        }
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value >= c_value);
                        }
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value >= c_value);
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::LQ(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value <= c_value);
                        }
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value <= c_value);
                        }
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value <= c_value);
                        }
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            current_stack.registers.A = RawType::bool(b_value <= c_value);
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::AND(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    let and = if c.is_bool() && b.is_bool() {
                        let b: bool = b.data.first().unwrap() == &1_u8;
                        let c: bool = c.data.first().unwrap() == &1_u8;
                        b && c
                    } else {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnmergebleTypes(b.type_id.id, c.type_id.id),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    };
                    current_stack.registers.A = RawType::bool(and);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::OR(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    let and = if c.is_bool() && b.is_bool() {
                        let b: bool = b.data.first().unwrap() == &1_u8;
                        let c: bool = c.data.first().unwrap() == &1_u8;
                        b || c
                    } else {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::UnmergebleTypes(b.type_id.id, c.type_id.id),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    };
                    current_stack.registers.A = RawType::bool(and);
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::ADD(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
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
                            current_stack.registers.A =
                                RawType::integer(result.to_le_bytes().to_vec());
                        }
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value + c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::float(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value + c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::double(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::DoubleOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Byte + Byte
                        (4, 4) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value + c_value;
                            if result > -128 && result < 127 {
                                current_stack.registers.A =
                                    RawType::integer(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::ByteOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // String + Integer
                        (6, 1) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value =
                                isize::from_le_bytes(c.data.try_into().unwrap()).to_string();
                            let result = b_value + &c_value;
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        //String + Float
                        (6, 2) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value + &c_value.to_string();
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        //String + Double
                        (6, 3) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value + &c_value.to_string();
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        // String + Byte
                        (6, 4) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value + &c_value.to_string();
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        // String + Bool
                        (6, 5) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = c.data.first().unwrap() == &1_u8;
                            let result = b_value + &c_value.to_string();
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        // String + String
                        (6, 6) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = String::from_utf8(c.data).unwrap();
                            let result = b_value + &c_value;
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        // String + Char
                        (6, 7) => {
                            let b_value = String::from_utf8(b.data).unwrap();
                            let c_value = char::from(u8::from_le_bytes(c.data.try_into().unwrap()));
                            let result = b_value + &c_value.to_string();
                            current_stack.registers.A = RawType::string(result.bytes().collect());
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
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
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();

                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
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
                            current_stack.registers.A =
                                RawType::integer(result.to_le_bytes().to_vec());
                        }
                        //Float - float
                        (2, 2) => todo!(),
                        // Double - Double
                        (3, 3) => todo!(),
                        // Byte - Byte
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
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
                            current_stack.registers.A =
                                RawType::integer(result.to_le_bytes().to_vec());
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}:\n{:?}:{:?}", file!(), line!(), b, c),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::MUL(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();

                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value * c_value;
                            current_stack.registers.A =
                                RawType::integer(result.to_le_bytes().to_vec());
                        }
                        //Float * float
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value * c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::float(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Double * Double
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value * c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::double(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Byte * Byte
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value * c_value;
                            if result > i8::MAX || result < i8::MIN {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            } else {
                                current_stack.registers.A =
                                    RawType::integer(result.to_le_bytes().to_vec());
                            }
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::EXP(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let c_value: u32 = match c_value.try_into() {
                                Ok(e) => e,
                                Err(_) => {
                                    return Err(ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    }));
                                }
                            };
                            let result = b_value.pow(c_value);
                            current_stack.registers.A =
                                RawType::integer(result.to_le_bytes().to_vec());
                        }
                        //Float ^ float
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value.powf(c_value);
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::float(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Double ^ Double
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value.powf(c_value);
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::double(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Byte ^ Byte
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            let c_value: u32 = match c_value.try_into() {
                                Ok(e) => e,
                                Err(_) => {
                                    return Err(ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::IntegerOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    }));
                                }
                            };
                            let result = b_value.pow(c_value);
                            if result > i8::MAX || result < i8::MIN {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            } else {
                                current_stack.registers.A =
                                    RawType::integer(result.to_le_bytes().to_vec());
                            }
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::DIV(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let result = isize::checked_div(b_value, c_value);
                            if result.is_some() {
                                current_stack.registers.A =
                                    RawType::integer(result.unwrap().to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        //Float / float
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value / c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::float(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Double / Double
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value / c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::double(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Byte / Byte
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value / c_value;
                            if result > i8::MAX || result < i8::MIN {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            } else {
                                current_stack.registers.A =
                                    RawType::integer(result.to_le_bytes().to_vec());
                            }
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::MOD(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Implicit => {
                    let b = current_stack.registers.B.clone();
                    let c = current_stack.registers.C.clone();
                    match (b.type_id.id, c.type_id.id) {
                        (1, 1) => {
                            let b_value = isize::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = isize::from_le_bytes(c.data.try_into().unwrap());
                            let result = isize::checked_rem(b_value, c_value);
                            if result.is_some() {
                                current_stack.registers.A =
                                    RawType::integer(result.unwrap().to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        //Float / float
                        (2, 2) => {
                            let b_value = f32::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f32::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value % c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::float(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Double / Double
                        (3, 3) => {
                            let b_value = f64::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = f64::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value % c_value;
                            if result.is_finite() {
                                current_stack.registers.A =
                                    RawType::double(result.to_le_bytes().to_vec());
                            } else {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::FloatOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                        }
                        // Byte / Byte
                        (4, 4) => {
                            let b_value = i8::from_le_bytes(b.data.try_into().unwrap());
                            let c_value = i8::from_le_bytes(c.data.try_into().unwrap());
                            let result = b_value % c_value;
                            if result > i8::MAX || result < i8::MIN {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::IntegerOverflow,
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            } else {
                                current_stack.registers.A =
                                    RawType::integer(result.to_le_bytes().to_vec());
                            }
                        }
                        _ => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::UnmergebleTypes(
                                    b.type_id.id,
                                    c.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
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
                    current_stack.stack_pos = e;
                    return Ok(ThreadStep {
                        instruction: current_instruction.clone(),
                        stack_pos: current_stack.stack_pos,
                        stack_id: current_stack.id,
                        info: ThreadStepInfo::JMP(e),
                    });
                }
                _ => unreachable!("Illegal addressing value"),
            },
            Instructions::CALL(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Absolute(stack_pos) => {
                    let target = &self.program[*stack_pos];
                    match &target.addressing_value {
                        utils::AddressingValues::Immediate(e) => {
                            let hash = e.data[0..self.arch.usize_len() as usize].to_vec();
                            let hash = usize::from_le_bytes(hash.try_into().unwrap());
                            let function_escape_pos = usize::from_le_bytes(
                                e.data[self.arch.usize_len() as usize..].try_into().unwrap(),
                            );
                            current_stack.stack_pos += 1;
                            let current_stack_id = current_stack.id.clone();
                            let current_stack_x = current_stack.registers.X.clone();

                            //let frame_pos = self.program.len() + self.stack.calculate_stack_length() + (function_escape_pos - (stack_pos + 1));
                            let frame_pos = self.program.len() + self.stack.calculate_stack_length();

                            match self.stack.push(Stack {
                                id: hash,
                                name: format!("fn<{}>", stack_pos),
                                stack_len: function_escape_pos - (stack_pos + 1),
                                registers: Registers {
                                    A: RawType::void(),
                                    B: RawType::void(),
                                    C: RawType::void(),
                                    X: current_stack_x,
                                    Y: RawType::void(),
                                },
                                caller: Some(current_stack_id),
                                stack_pos: *stack_pos,
                                frame_pos,
                            }) {
                                Ok(_) => {
                                    return Ok(ThreadStep {
                                        instruction: current_instruction.clone(),
                                        stack_pos: stack_pos.clone(),
                                        stack_id: hash,
                                        info: ThreadStepInfo::CALL(*stack_pos),
                                    });
                                }
                                Err(_) => {
                                    return Err(ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::StackOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    }));
                                }
                            };

                            /*
                            match self.program.len().checked_mul(function_escape_pos - (stack_pos + 1)) {
                                Some(frame_pos) => {
                                    
                                }
                                None => {
                                    return Err(ThreadExit::Panic(ThreadPanic {
                                        reason: ThreadPanicReason::StackOverflow,
                                        stack_trace: self.stack.stack.clone(),
                                        code_location: format!("{}:{}", file!(), line!()),
                                    }));
                                }
                            }
                            */
                        }
                        _ => panic!("Illegal addressing value"),
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::CALLN(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::Immediate(target) => {
                    let location = String::from_utf8(target.data.clone()).unwrap();
                    let module = location.split(">").collect::<Vec<_>>()[0];
                    let fn_name = location.split(">").collect::<Vec<_>>()[1];
                    let raw_params = {
                        if current_stack.registers.X.type_id.id != 9 {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::InvalidRegisterAccess(
                                    current_stack.registers.X.type_id.id,
                                ),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }

                        let mut params = Vec::new();
                        let _params: Vec<usize> = current_stack
                            .registers
                            .X
                            .data
                            .chunks(self.arch.usize_len().into())
                            .map(|x| usize::from_le_bytes(x.try_into().unwrap()))
                            .collect();

                        for param_location in _params {
                            params.push(match heap.get(&param_location) {
                                Some(e) => e.clone(),
                                None => match &self.program[param_location].addressing_value {
                                    utils::AddressingValues::Immediate(e) => e.clone(),
                                    _ => {
                                        return Err(ThreadExit::Panic(ThreadPanic {
                                            reason: ThreadPanicReason::MemoryAccessViolation(
                                                param_location.clone(),
                                                current_stack.stack_pos,
                                            ),
                                            stack_trace: self.stack.stack.clone(),
                                            code_location: format!("{}:{}", file!(), line!()),
                                        }));
                                    }
                                },
                            });
                        }
                        params
                    };
                    let native_call = VmNativeCall {
                        module: module.to_string(),
                        name: fn_name.to_string(),
                        params: raw_params,
                    };

                    current_stack.registers.Y = match (self.native_call_channel)(
                        ThreadInfo {
                            id: self.id,
                            stack_id: current_stack.id,
                            stack_name: current_stack.name.clone(),
                            stack_caller: current_stack.caller.clone(),
                            stack_pos: current_stack.stack_pos,
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
                utils::AddressingValues::Parameter(_) => todo!(),
            },
            Instructions::PUSH(_) => match &current_instruction.addressing_value {
                utils::AddressingValues::IndirectA => {
                    if current_stack.registers.A.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.A.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let current_pos = &((current_stack.stack_pos + current_stack.frame_pos) - 1);
                    current_stack
                        .registers
                        .A
                        .data
                        .extend(current_pos.to_le_bytes());
                    current_stack.registers.A.type_id.size += self.arch.usize_len() as usize;
                }
                utils::AddressingValues::IndirectB => {
                    if current_stack.registers.B.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.B.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let current_pos = &((current_stack.stack_pos + current_stack.frame_pos) - 1);
                    current_stack
                        .registers
                        .B
                        .data
                        .extend(current_pos.to_le_bytes());
                    current_stack.registers.B.type_id.size += self.arch.usize_len() as usize;
                }
                utils::AddressingValues::IndirectC => {
                    if current_stack.registers.C.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.C.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let current_pos = &((current_stack.stack_pos + current_stack.frame_pos) - 1);
                    current_stack
                        .registers
                        .C
                        .data
                        .extend(current_pos.to_le_bytes());
                    current_stack.registers.C.type_id.size += self.arch.usize_len() as usize;
                }
                utils::AddressingValues::IndirectX => {
                    if current_stack.registers.X.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.X.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let current_pos = &((current_stack.stack_pos + current_stack.frame_pos) - 1);
                    current_stack
                        .registers
                        .X
                        .data
                        .extend(current_pos.to_le_bytes());
                    current_stack.registers.X.type_id.size += self.arch.usize_len() as usize;
                }
                utils::AddressingValues::IndirectY => {
                    if current_stack.registers.Y.type_id.id != 9 {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::InvalidRegisterAccess(
                                current_stack.registers.Y.type_id.id,
                            ),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                    let current_pos = &((current_stack.stack_pos + current_stack.frame_pos) - 1);
                    current_stack
                        .registers
                        .Y
                        .data
                        .extend(current_pos.to_le_bytes());
                    current_stack.registers.Y.type_id.size += self.arch.usize_len() as usize;
                }
                utils::AddressingValues::Absolute(e) => {
                    match heap.get_mut(&(e + current_stack.frame_pos)) {
                        Some(heap_value) => {
                            if heap_value.type_id.id != 9 {
                                return Err(ThreadExit::Panic(ThreadPanic {
                                    reason: ThreadPanicReason::InvalidRegisterAccess(
                                        heap_value.type_id.id,
                                    ),
                                    stack_trace: self.stack.stack.clone(),
                                    code_location: format!("{}:{}", file!(), line!()),
                                }));
                            }
                            let current_pos =
                                &((current_stack.stack_pos + current_stack.frame_pos) - 1);
                            heap_value.type_id.size += self.arch.usize_len() as usize;
                            heap_value.data.extend(current_pos.to_le_bytes());
                        }
                        None => {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::InvalidMemoryAccess(*e),
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                }
                _ => panic!("Illegal addressing value"),
            },
            Instructions::LEN(_) => todo!(),
            Instructions::A2I(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    1 => (),
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A =
                            RawType::integer((data as isize).to_le_bytes().to_vec());
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A =
                            RawType::integer((data as isize).to_le_bytes().to_vec());
                    }
                    4 => {
                        let data = current_stack.registers.A.to_byte();
                        current_stack.registers.A =
                            RawType::integer((data as isize).to_le_bytes().to_vec());
                    }
                    5 => {
                        let data = if current_stack.registers.A.to_bool() {
                            1_isize
                        } else {
                            0_isize
                        };
                        current_stack.registers.A = RawType::integer(data.to_le_bytes().to_vec());
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
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    1 => {
                        let data = current_stack.registers.A.to_int();
                        current_stack.registers.A =
                            RawType::float((data as f32).to_le_bytes().to_vec());
                    }
                    2 => (),
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A =
                            RawType::float((data as f32).to_le_bytes().to_vec());
                    }
                    4 => {
                        let data = current_stack.registers.A.to_byte();
                        current_stack.registers.A =
                            RawType::float((data as f32).to_le_bytes().to_vec());
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
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    1 => {
                        let data = current_stack.registers.A.to_int();
                        current_stack.registers.A =
                            RawType::double((data as f64).to_le_bytes().to_vec());
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A =
                            RawType::double((data as f64).to_le_bytes().to_vec());
                    }
                    3 => (),
                    4 => {
                        let data = current_stack.registers.A.to_byte();
                        current_stack.registers.A =
                            RawType::double((data as f64).to_le_bytes().to_vec());
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
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    1 => {
                        let data = current_stack.registers.A.to_int();
                        if data < 255 {
                            current_stack.registers.A = RawType::byte(data as u8);
                        } else {
                            return Err(ThreadExit::Panic(ThreadPanic {
                                reason: ThreadPanicReason::IntegerOverflow,
                                stack_trace: self.stack.stack.clone(),
                                code_location: format!("{}:{}", file!(), line!()),
                            }));
                        }
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A = RawType::byte(if data.is_sign_negative() {
                            data.to_be_bytes()[0]
                        } else {
                            data.to_le_bytes()[0]
                        });
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A = RawType::byte(if data.is_sign_negative() {
                            data.to_be_bytes()[0]
                        } else {
                            data.to_le_bytes()[0]
                        });
                    }
                    4 => (),
                    5 => {
                        let data = current_stack.registers.A.to_bool();
                        current_stack.registers.A = RawType::byte(if data { 1 } else { 0 });
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
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    1 => {
                        let data = current_stack.registers.A.to_int();
                        current_stack.registers.A =
                            RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A =
                            RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A =
                            RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    4 => {
                        let data = current_stack.registers.A.to_byte();
                        current_stack.registers.A =
                            RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    5 => {
                        let data = current_stack.registers.A.to_bool();
                        current_stack.registers.A =
                            RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    6 => (),
                    7 => {
                        let data = current_stack.registers.A.to_char();
                        current_stack.registers.A =
                            RawType::string(data.to_string().as_bytes().to_vec());
                    }
                    8 => {
                        current_stack.registers.A = RawType::string("void".as_bytes().to_vec());
                    }
                    10 => {
                        current_stack.registers.A = RawType::string("null".as_bytes().to_vec());
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
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    7 => (),
                    6 => {
                        let data = current_stack
                            .registers
                            .A
                            .to_string()
                            .chars()
                            .collect::<Vec<_>>()[0];
                        current_stack.registers.A =
                            RawType::char((data as u32).to_le_bytes().to_vec());
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
                utils::AddressingValues::Implicit => match current_stack.registers.A.type_id.id {
                    1 => {
                        let data = current_stack.registers.A.to_int();
                        current_stack.registers.A = RawType::bool(data.is_positive());
                    }
                    2 => {
                        let data = current_stack.registers.A.to_float();
                        current_stack.registers.A = RawType::bool(data.is_sign_positive());
                    }
                    3 => {
                        let data = current_stack.registers.A.to_double();
                        current_stack.registers.A = RawType::bool(data.is_sign_negative());
                    }
                    4 => {
                        current_stack.registers.A = RawType::bool(true);
                    }
                    5 => (),
                    6 => {
                        let data = current_stack.registers.A.to_string();
                        current_stack.registers.A = RawType::bool(data.len() > 0);
                    }
                    7 => {
                        let data = current_stack.registers.A.to_char();
                        current_stack.registers.A = RawType::bool(data != '\0');
                    }
                    8 => {
                        current_stack.registers.A = RawType::bool(false);
                    }
                    10 => {
                        current_stack.registers.A = RawType::bool(false);
                    }
                    _ => unreachable!(),
                },
                _ => unreachable!("Illegal addressing value"),
            },
            Instructions::JMPA(_) => match current_instruction.addressing_value {
                utils::AddressingValues::Absolute(e) => {
                    if current_stack.registers.A.is_bool() {
                        if current_stack.registers.A.data[0] == 1 {
                            current_stack.stack_pos = e;
                            return Ok(ThreadStep {
                                instruction: current_instruction.clone(),
                                stack_pos: current_stack.stack_pos,
                                stack_id: current_stack.id,
                                info: ThreadStepInfo::JMP(e),
                            });
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
            Instructions::CO(_) => todo!(),
            Instructions::FN(_) => {
                match &current_instruction.addressing_value {
                    utils::AddressingValues::Immediate(e) => {
                        let hash = e.data[0..self.arch.usize_len() as usize].to_vec();
                        let hash = usize::from_le_bytes(hash.try_into().unwrap());
                        let escape = e.data[self.arch.usize_len() as usize..].to_vec();
                        let escape = usize::from_le_bytes(escape.try_into().unwrap());
                        if hash != current_stack.id {
                            current_stack.stack_pos = escape;
                            return Ok(ThreadStep {
                                instruction: current_instruction.clone(),
                                stack_pos: current_stack.stack_pos,
                                stack_id: current_stack.id,
                                info: ThreadStepInfo::JMP(escape),
                            });
                        }
                    }
                    _ => unreachable!("Illegal addressing value"),
                };
            }
            Instructions::UGR(_) => todo!(),
            Instructions::ULR(_) => todo!(),
        }
        let stack_id = current_stack.id;
        let stack_pos = current_stack.stack_pos;
        if drop_current_stack {
            let current_Y = current_stack.registers.Y.clone();
            match current_stack.caller {
                Some(caller) => match self.stack.stack.iter_mut().find(|x| x.id == caller) {
                    Some(caller_stack) => {
                        caller_stack.registers.Y = current_Y;
                        let stack_id = caller_stack.id;
                        self.stack.pop();
                        return Ok(ThreadStep {
                            instruction: current_instruction.clone(),
                            stack_pos,
                            stack_id,
                            info: ThreadStepInfo::DropStack,
                        })
                    }
                    None => {
                        return Err(ThreadExit::Panic(ThreadPanic {
                            reason: ThreadPanicReason::BrokenStackTree(0),
                            stack_trace: self.stack.stack.clone(),
                            code_location: format!("{}:{}", file!(), line!()),
                        }));
                    }
                },
                None => {
                    self.stack.pop();
                },
            }
        } else {
            current_stack.stack_pos += 1;
        }

        Ok(ThreadStep {
            instruction: current_instruction.clone(),
            stack_pos,
            stack_id,
            info: ThreadStepInfo::StepNext,
        })
    }
}
