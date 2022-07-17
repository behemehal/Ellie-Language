use ellie_core::defs::{PlatformArchitecture, VmNativeCall};

use crate::{
    heap::Heap,
    program::{Program, ReadInstruction},
    thread::{Stack, Thread},
    utils::ThreadExit,
};

pub struct VM<'a, T> {
    pub stack: Vec<ReadInstruction>,
    pub threads: Vec<Thread<'a, T>>,
    pub heap: Heap,
    native_call_channel: T,
    target_arch: PlatformArchitecture,
}

impl<'a, T> VM<'a, T>
where
    T: Fn(VmNativeCall) -> bool + Clone + Copy + Sized,
{
    pub fn new(target_arch: PlatformArchitecture, native_call_channel: T) -> Self {
        #[cfg(feature = "debug")]
        println!(
            "{}[VM]{}: Creating vm instance",
            utils::Colors::Yellow,
            utils::Colors::Reset
        );
        VM {
            stack: Vec::new(),
            threads: Vec::new(),
            heap: Heap::new(),
            target_arch,
            native_call_channel,
        }
    }

    pub fn load(&mut self, program: Program) {
        if self.target_arch != program.arch {
            #[cfg(feature = "debug")]
            panic!(
                "{}[VM]{}: Target arch {} does not match program arch {}",
                utils::Colors::Red,
                utils::Colors::Reset,
                self.target_arch,
                program.arch
            );
        }

        #[cfg(feature = "debug")]
        println!(
            "{}[VM]{}: Loading program that contains '{}' instructions",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            program.instructions.len()
        );

        self.stack = program.instructions;

        #[cfg(feature = "debug")]
        println!(
            "{}[VM]{}: Program loaded",
            utils::Colors::Yellow,
            utils::Colors::Reset,
        );
    }

    pub fn run(&mut self, main: usize) -> ThreadExit {
        let mut thread = Thread::new(
            0,
            self.target_arch,
            &self.stack,
            &mut self.heap,
            self.native_call_channel,
        );
        thread
            .stack
            .push(Stack {
                id: 0,
                name: "<ellie_main>".to_string(),
                caller: None,
                pos: main,
            })
            .unwrap();
        thread.run()
    }

    pub fn heap_dump(&mut self) -> String {
        self.heap.dump()
    }
}
