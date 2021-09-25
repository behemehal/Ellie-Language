//use ellie_parser::syntax::{
//    caller, class, condition, constructor, definers, file_key, for_loop, function, import,
//    import_item, native_function, ret, types, variable,
//};

use core::borrow::Borrow;

use crate::heap;
use crate::stack;
use crate::thread;
use alloc::boxed::Box;
use alloc::collections::BTreeMap;
use alloc::vec::Vec;
use ellie_core::definite;

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
        let mut stacks: BTreeMap<usize, stack::Stack> = BTreeMap::new();
        let main_stack = stack::Stack::new(0);
        stacks.insert(0, main_stack);

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
                std::println!("require_frame: stackid: {} STEP: {}", stack_id, stack_step);
                //self.events.call(RuntimeEventMessage::None);
                thread::ThreadMessage::CONTINUE
            });

        let main_thread = thread::Thread::new(0, stacks, heap::Heap::new(), thread_controller);
        self.threads.push(main_thread);
    }
}
