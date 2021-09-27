use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::heap;
use crate::stack;
use crate::stack::StackElements;

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

impl Page {
    pub fn header_exists(&self, target_name: &str) -> Option<usize> {
        let mut found = false;
        let mut found_id = 0;
        for (id, name) in self.headers.clone() {
            if name == target_name {
                found = true;
                found_id = id;
                break;
            }
        }
        if found {
            Some(found_id)
        } else {
            None
        }
    }
}

pub struct Thread {
    pub id: usize,
    pub pages: BTreeMap<u64, Page>,
    pub controller: ThreadController,
    pub tasks: Vec<u64>,
    pub hang: bool,
}

impl Thread {
    pub fn new(id: usize, thread_controller: ThreadController) -> Thread {
        let mut pages: BTreeMap<u64, Page> = BTreeMap::new();
        //Ring zero
        pages.insert(
            1,
            Page {
                page_id: 1,
                headers: BTreeMap::new(),
                heap: heap::Heap::new(),
                stack: stack::Stack::new(1),
                step: 0,
            },
        );

        Thread {
            id,
            pages,
            tasks: vec![0],
            hang: true,
            controller: thread_controller,
        }
    }

    pub fn glb_look_up_for_item_by_name(&self, name: &str) -> Option<stack::StackElements> {
        let mut found = false;
        let mut found_type = stack::StackElements::None;

        'page_loop: for (_, current_page) in self.pages.clone() {
            match current_page.header_exists(name) {
                Some(id) => {
                    //If current page has the type
                     match current_page.stack.clone().element_exists(id) {
                        Some(element_exists) => {
                            found_type = element_exists;
                            found = true;
                            break 'page_loop;
                        },
                        _ => ()
                    }
                }
                None => {
                    //Else look up in imported pages
                    for i in current_page.stack.elements.clone() {
                        if let stack::StackElements::Reference(reference) = i {
                            match self.pages.get(&(reference.page_id as u64)) {
                                Some(page) => match page.header_exists(name) {
                                    Some(e) => match page.stack.clone().element_exists(e) {
                                        Some(found_rtype) => {
                                            found = true;
                                            found_type = found_rtype;
                                            break 'page_loop;
                                        }
                                        None => panic!("UNEXPECTED RUNTIME BEHAVIOUR"),
                                    },
                                    None => (),
                                },
                                None => panic!("UNEXPECTED BRIDGE ERROR"),
                            }
                        }
                    }
                }
            }
        }

        if found {
            Some(found_type)
        } else {
            None
        }
    }

    pub fn _look_up_for_item_by_name(
        &self,
        name: &str,
        current_page_id: u64,
    ) -> Option<stack::StackElements> {
        match self.pages.get(&current_page_id.clone()) {
            Some(current_page) => {
                match current_page.header_exists(name) {
                    Some(id) => {
                        //If current page has the type
                        current_page.stack.clone().element_exists(id)
                    }
                    None => {
                        //Else look up in imported pages

                        let mut found = false;
                        let mut found_type = stack::StackElements::None;

                        'search: for i in current_page.stack.elements.clone() {
                            if let stack::StackElements::Reference(reference) = i {
                                match self.pages.get(&(reference.page_id as u64)) {
                                    Some(page) => match page.header_exists(name) {
                                        Some(e) => match page.stack.clone().element_exists(e) {
                                            Some(found_rtype) => {
                                                found = true;
                                                found_type = found_rtype;
                                                break 'search;
                                            }
                                            None => panic!("UNEXPECTED RUNTIME BEHAVIOUR"),
                                        },
                                        None => (),
                                    },
                                    None => panic!("UNEXPECTED BRIDGE ERROR"),
                                }
                            }
                        }

                        if found {
                            Some(found_type)
                        } else {
                            None
                        }
                    }
                }
            }
            None => None,
        }
    }

    pub fn look_up_for_item(&self, page_id: u64, target: usize) -> Option<stack::StackElements> {
        match self.pages.get(&page_id) {
            Some(page) => {
                let element = page.stack.clone().element_exists(target);
                match element.clone() {
                    Some(e) => match e {
                        stack::StackElements::Reference(e) => {
                            self.look_up_for_item(e.page_id as u64, e.type_id)
                        }
                        _ => element,
                    },
                    None => None,
                }
            }
            None => None,
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
