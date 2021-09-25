use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec;
use alloc::vec::Vec;

use crate::heap;
use crate::stack;

#[derive(Debug, Clone)]
pub enum NotifierMessageType {
    StackIdFault,
    HeapIdFault,
    CommandFault,
    StackPaused,
    StackResume,
    StackCrash,
    StackComplete,
    ThreadStopped,
    StackStep,
}

#[derive(Debug, Clone)]
pub struct NotifierMessage {
    pub stack_id: usize,
    pub stack_step: usize,
    pub message_type: NotifierMessageType,
}

pub enum ThreadMessage {
    HANG,
    CONTINUE,
    HALT,
}

pub trait NotifierFn {
    fn call(&mut self, args: NotifierMessage);
}

impl<T> NotifierFn for T
where
    T: FnMut(NotifierMessage) + Sized,
{
    fn call(&mut self, args: NotifierMessage) {
        self(args)
    }
}

pub trait FrameFn {
    fn call(&mut self, args: (usize, usize)) -> ThreadMessage;
}

impl<T> FrameFn for T
where
    T: FnMut((usize, usize)) -> ThreadMessage + Sized,
{
    fn call(&mut self, args: (usize, usize)) -> ThreadMessage {
        self(args)
    }
}

pub struct ThreadController {
    pub notifier: Box<dyn NotifierFn>,
    pub require_frame: Box<dyn FrameFn>,
    pub wait_next_frame: bool,
}

impl ThreadController {
    pub fn new() -> Self {
        ThreadController {
            notifier: Box::new(|_| {}),
            require_frame: Box::new(|_| ThreadMessage::CONTINUE),
            wait_next_frame: true,
        }
    }
}

pub struct Thread {
    pub id: usize,
    pub stack: BTreeMap<usize, stack::Stack>,
    pub heap: heap::Heap,
    pub controller: ThreadController,
    pub tasks: Vec<usize>,
    pub hang: bool,
}

impl Thread {
    pub fn new(
        id: usize,
        stack: BTreeMap<usize, stack::Stack>,
        heap: heap::Heap,
        thread_controller: ThreadController,
    ) -> Thread {
        Thread {
            id,
            stack,
            heap,
            tasks: vec![0],
            hang: true,
            controller: thread_controller,
        }
    }

    pub fn run(&mut self) {
        if self.tasks.len() == 0 {
            self.controller.notifier.call(NotifierMessage {
                stack_id: 0,
                stack_step: 0,
                message_type: NotifierMessageType::ThreadStopped,
            });
        } else {
            let current_job = self.tasks.last().unwrap().clone();
            match self.stack.get_mut(&current_job) {
                Some(stack) => {
                    let _frame_response = self
                        .controller
                        .require_frame
                        .call((current_job, stack.step));
                    match stack.elements.get(stack.step) {
                        Some(element) => {
                            std::println!("THREAD RUN COMMAND: {:#?}", element);
                            //match element {
                            //    stack::StackElements::Function(_) => todo!(),
                            //    stack::StackElements::Class(_) => todo!(),
                            //    stack::StackElements::Variable(e) => {},
                            //};
                            stack.step += 1;
                            if stack.elements.len() == stack.step {
                                self.tasks.pop();
                                self.controller.notifier.call(NotifierMessage {
                                    stack_id: current_job,
                                    stack_step: 0,
                                    message_type: NotifierMessageType::StackComplete,
                                });
                            } else {
                                self.controller.notifier.call(NotifierMessage {
                                    stack_id: current_job,
                                    stack_step: 0,
                                    message_type: NotifierMessageType::StackStep,
                                });
                            }
                        }
                        None => {
                            self.controller.notifier.call(NotifierMessage {
                                stack_id: current_job,
                                stack_step: 0,
                                message_type: NotifierMessageType::CommandFault,
                            });
                            self.tasks = vec![];
                        }
                    }
                }
                None => {
                    self.controller.notifier.call(NotifierMessage {
                        stack_id: current_job,
                        stack_step: 0,
                        message_type: NotifierMessageType::StackIdFault,
                    });
                    self.tasks = vec![];
                }
            }
        }

        if !self.hang {}
    }
}
