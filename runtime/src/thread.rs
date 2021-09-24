use alloc::collections::BTreeMap;
use alloc::vec::Vec;

use crate::heap;
use crate::stack;

#[derive(Debug)]
pub enum NotifierMessageType {
    StackFault,
    HeapIdFault,
    CommandFault,
    StackHang,
    StackResume,
    StackCrash,
}

#[derive(Debug)]
pub struct NotifierMessage {
    pub stack_id: usize,
    pub message_type: NotifierMessageType,
}

pub enum ThreadMessage {
    HANG,
    CONTINUE,
    HALT,
}

#[derive(Default)]
pub struct ThreadController<F, E> {
    pub notifier: F,
    pub require_frame: E,
    pub wait_next_frame: bool,
}

impl<F, E> ThreadController<F, E>
where
    E: FnMut(usize) + Clone + Sized,
    F: FnMut(NotifierMessage) -> ThreadMessage + Clone + Sized,
{
    pub fn new(
        notifier: F,
        require_frame: E,
    ) -> ThreadController<
        impl FnMut(NotifierMessage) -> ThreadMessage + Clone + Sized,
        impl FnMut(usize) + Clone + Sized,
    > {
        ThreadController {
            notifier,
            require_frame,
            wait_next_frame: true,
        }
    }
}

pub struct Thread<F, E> {
    pub id: usize,
    pub stack: BTreeMap<usize, stack::Stack>,
    pub heap: heap::Heap,
    pub controller: ThreadController<F, E>,
    pub current_stack: usize,
    pub hang: bool,
}

impl<F, E> Thread<F, E>
where
    E: FnMut(usize) + Clone + Sized,
    F: FnMut(NotifierMessage) -> ThreadMessage + Clone + Sized,
{
    pub fn new(
        id: usize,
        stack: BTreeMap<usize, stack::Stack>,
        heap: heap::Heap,
        thread_controller: ThreadController<F, E>,
    ) -> Thread<F, E> {
        Thread {
            id,
            stack,
            heap,
            current_stack: 0,
            hang: true,
            controller: thread_controller,
        }
    }

    pub fn run(&mut self) {
        if !self.hang {
            match self.stack.get(&self.current_stack) {
                Some(stack) => {
                    let frame_response = (self.controller.require_frame)(self.current_stack);

                },
                None => ,
            }

        }
    }
}
