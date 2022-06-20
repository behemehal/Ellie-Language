use std::cell::RefCell;

use crate::{
    heap::Heap,
    program::{Program, ReadInstruction},
    thread::{Stack, Thread},
    utils::{self},
};

pub struct VM<'a> {
    pub stack: Vec<ReadInstruction>,
    pub threads: Vec<Thread<'a>>,
    pub heap: Heap,
    target_arch: u8,
}

impl<'a> VM<'a> {
    pub fn new(target_arch: u8) -> Self {
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
        }
    }

    pub fn execute(&'a mut self, program: Program) {
        if self.target_arch != program.arch {
            panic!(
                "{}[VM]{}: Target arch {} does not match program arch {}",
                utils::Colors::Red,
                utils::Colors::Reset,
                self.target_arch,
                program.arch
            );
        }

        println!(
            "{}[VM]{}: Loading program that contains '{}' instructions",
            utils::Colors::Yellow,
            utils::Colors::Reset,
            program.instructions.len()
        );

        self.stack = program.instructions;

        println!(
            "{}[VM]{}: Creating thread",
            utils::Colors::Yellow,
            utils::Colors::Reset
        );
        {
            let mut thread = Thread::new(0, self.target_arch, &self.stack, &mut self.heap);
            thread.stack.push(Stack {
                id: 0,
                name: "<ellie_main>".to_string(),
                caller: None,
                pos: program.main,
            });
            let main_thread = thread.run();
        }

        let dump = self.heap.dump();
        println!("\n{}", dump);
    }
}
