use crate::{config::STACK_SIZE, thread::Registers, raw_type::StaticRawType};
use alloc::vec::Vec;

#[derive(Debug, Clone, Copy)]
pub struct Stack {
    // Stack Position
    pub pos: usize,
    // Exact copy of frame_pos in thread. This is for easy access never changes after creation
    pub frame_pos: usize,
    // ID of the stack, this is hash of the called function
    pub id: usize,
    // Length of the stack
    pub stack_len: usize,
    // Registers of the stack
    pub registers: Registers,
    // Caller of the stack, this is hash of the caller
    pub caller: Option<usize>,
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            pos: 0,
            frame_pos: 0,
            id: 0,
            stack_len: 0,
            registers: Registers {
                A: StaticRawType::from_void(),
                B: StaticRawType::from_void(),
                C: StaticRawType::from_void(),
                X: StaticRawType::from_void(),
                Y: StaticRawType::from_void(),
            },
            caller: None,
        }
    }

    pub fn get_pos(&self) -> usize {
        self.frame_pos + self.pos
    }

    pub fn calculate_frame_pos(&mut self, pos: usize) -> usize {
        self.frame_pos + pos
    }
}

#[derive(Debug, Clone)]
pub struct StackArray {
    pub data: [Stack; STACK_SIZE],
    pub len: usize,
}

impl StackArray {
    pub fn new() -> Self {
        StackArray {
            data: [Stack::new(); STACK_SIZE],
            len: 0,
        }
    }

    pub fn get(&mut self, index: usize) -> Option<&mut Stack> {
        if index < self.len {
            Some(&mut self.data[index])
        } else {
            None
        }
    }

    pub fn push(&mut self, stack: Stack) {
        self.data[self.len] = stack;
        self.len += 1;
    }

    pub fn len(&self) -> usize {
        self.len
    }

    pub fn clone(&self) -> Vec<Stack> {
        self.data[0..self.len].to_vec()
    }

    pub fn last_mut(&mut self) -> Option<&mut Stack> {
        Some(&mut self.data[self.len - 1])
    }

    pub fn pop(&mut self) {
        self.data[self.len] = Stack::new();
        self.len -= 1;
    }
}
