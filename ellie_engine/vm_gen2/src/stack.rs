use crate::config::STACK_SIZE;

pub type StackOverflowCallback = Box<dyn FnMut()>;

pub struct Stack {
    pub data: heapless::Vec<u8, STACK_SIZE>,
    pub pos: usize,
    pub on_stack_overflow: Option<Box<dyn FnMut()>>,
    pub on_segmentation_fault: Option<Box<dyn FnMut()>>,
}

impl Clone for Stack {
    fn clone(&self) -> Self {
        Stack {
            data: self.data.clone(),
            pos: self.pos,
            on_stack_overflow: None,
        }
    }
}

impl Default for Stack {
    fn default() -> Self {
        Self::new()
    }
}

impl Stack {
    pub fn new() -> Stack {
        Stack {
            data: heapless::Vec::new(),
            pos: 0,
            on_stack_overflow: None,
            on_segmentation_fault: None,
        }
    }

    pub fn set_on_stack_overflow(&mut self, callback: StackOverflowCallback) {
        self.on_stack_overflow = Some(callback);
    }

    pub fn set_on_segmentation_fault(&mut self, callback: StackOverflowCallback) {
        self.on_segmentation_fault = Some(callback);
    }

    pub fn push(&mut self, value: u8) {
        if self.data.len() >= STACK_SIZE {
            if let Some(callback) = &mut self.on_stack_overflow {
                callback();
                return;
            }
        }

        self.data.push(value).unwrap();
        self.pos += 1;
    }

    pub fn pop(&mut self) -> Option<u8> {
        if self.pos > 0 {
            self.pos -= 1;
            self.data.pop()
        } else {
            if let Some(callback) = &mut self.on_segmentation_fault {
                callback();
            }
            None
        }
    }
}
