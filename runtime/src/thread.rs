use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
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
    pub stack_id: u64,
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
    fn call(&mut self, args: (u64, usize)) -> ThreadMessage;
}

impl<T> FrameFn for T
where
    T: FnMut((u64, usize)) -> ThreadMessage + Sized,
{
    fn call(&mut self, args: (u64, usize)) -> ThreadMessage {
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

#[derive(Debug, Clone)]
pub struct Page {
    pub page_id: u64,
    pub headers: BTreeMap<usize, String>,
    pub heap: heap::Heap,
    pub stack: stack::Stack,
    pub step: usize,
}

pub struct Thread {
    pub id: usize,
    pub pages: BTreeMap<u64, Page>,
    pub controller: ThreadController,
    pub tasks: Vec<u64>,
    pub hang: bool,
}

impl Thread {
    pub fn new(
        id: usize,
        pages: BTreeMap<u64, Page>,
        thread_controller: ThreadController,
    ) -> Thread {
        Thread {
            id,
            pages,
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
            match self.pages.get_mut(&current_job) {
                Some(page) => {
                    let _frame_response =
                        self.controller.require_frame.call((current_job, page.step));
                    match page.stack.elements.get(page.step) {
                        Some(element) => {
                            std::println!("THREAD RUN COMMAND: {:#?}", element);
                            //match element {
                            //    stack::StackElements::Function(_) => todo!(),
                            //    stack::StackElements::Class(_) => todo!(),
                            //    stack::StackElements::Variable(e) => {},
                            //};
                            page.step += 1;
                            if page.stack.elements.len() == page.step {
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
