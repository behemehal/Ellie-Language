use crate::heap;
use crate::stack;
use crate::thread;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;
use ellie_core::definite;
use rand;

pub enum RuntimeEventMessage {
    None,
}

pub enum RuntimeEventResponse {
    None,
}

pub trait RuntimeEventFn {
    fn call(self, args: RuntimeEventMessage) -> RuntimeEventResponse;
}

impl<T> RuntimeEventFn for T
where
    T: FnOnce(RuntimeEventMessage) -> RuntimeEventResponse + Sized,
{
    fn call(self, args: RuntimeEventMessage) -> RuntimeEventResponse {
        self(args)
    }
}

pub struct Runtime {
    pub threads: Vec<thread::Thread>,
    pub events: Box<dyn RuntimeEventFn>,
    initialized: bool,
}

pub fn panic_dumper(thread: &thread::Thread) -> String {
    let mut dump_data = "\r\n---\n\r".to_owned();
    let mut stack_dump = String::new();
    for stack in thread.pages.clone() {
        let mut headers = String::new();

        for item in stack.1.headers {
            headers += &format!("\t\t\t{:#04x} : {}\n", item.0, item.1)
        }

        stack_dump += &format!(
            "\t---\n\tPage {:#04x}:\n\t\tHeaders:\n{}\n\t\tStack:\n\t{}\n",
            stack.0,
            headers,
            stack.1.stack.dump(),
        );
    }

    dump_data += &format!("Pages:\n{}\n\t---", stack_dump);
    dump_data
}

impl Runtime {
    pub fn spawn(event_listener: Box<dyn RuntimeEventFn>) -> Runtime {
        let mut runtime = Runtime {
            threads: Vec::new(),
            events: event_listener,
            initialized: true,
        };
        runtime.initialized = true;
        runtime
    }

    pub fn run(&mut self, code: Vec<definite::items::Collecting>) {
        let mut thread_controller = thread::ThreadController::new();
        thread_controller.notifier = Box::new(move |x: thread::NotifierMessage| {
            std::println!(
                "THREAD MESSAGE: {}",
                alloc::format!("{:?}", x.clone()).to_uppercase()
            );

            //event_listener.call(RuntimeEventMessage::None);
        });

        thread_controller.require_frame =
            Box::new(move |(stack_id, stack_step)| -> thread::ThreadMessage {
                std::println!("require_frame: stack_id: {} STEP: {}", stack_id, stack_step);
                //self.events.call(RuntimeEventMessage::None);
                thread::ThreadMessage::CONTINUE
            });

        self.threads.push(thread::Thread::new(1, thread_controller));

        for item in code {
            //Split code to pages

            match self.threads.iter_mut().find(|x| x.id == 1) {
                Some(thread) => {
                    thread.add_item_to_stack(1, item);
                }
                None => {
                    panic!("Unexpected runtime error, cannot find thread");
                }
            }
        }
    }

    pub fn dump(&self) -> String {
        let mut dump_data = "DUMP:\r\n".to_owned();
        for thread in &self.threads {
            let mut stack_dump = String::new();
            for (ind, stack) in thread.pages.clone().into_iter().enumerate() {
                let mut headers = String::new();

                if stack.1.headers.is_empty() {
                    headers += "\t\t\tEMPTY";
                }

                for item in stack.1.headers {
                    headers += &format!("\t\t\t{:#04x} : {}\n\t", item.0, item.1)
                }

                stack_dump += &format!(
                    "{}\t\tPage {:#04x}:\n\t\t\tHeaders:\n\t{}\n\t\t\tStack:\n\t{}\n",
                    if ind == 0 { "" } else { "\n" },
                    stack.0,
                    headers,
                    stack.1.stack.dump()
                );
            }

            dump_data += &format!(
                "Thread: {:#04x}:\n\tHeap:\n\t{}\n\tPages:\n{}\n\t",
                thread.id,
                match thread.heap.clone() {
                    thread::HeapType::SolidHeap(e) => e.dump(),
                    thread::HeapType::SharedHeap(e) =>
                        panic!("SHARED HEAP: ({:#04x}, {:#04x})", e.0, e.1),
                },
                stack_dump
            );
        }
        dump_data
    }
}
