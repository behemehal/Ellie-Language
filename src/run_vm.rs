use ellie_vm::{program::Program, utils};
use std::fs::File;
use std::io::Read;
use std::path::Path;

use crate::cli_utils;
use crate::cli_utils::read_error_text;

use ellie_vm::{
    utils::{ProgramReader, Reader},
    vm::VM,
};

pub struct VmSettings {
    pub json_log: bool,
    pub warnings: bool,
    pub architecture: ellie_core::defs::PlatformArchitecture,
}

pub struct RFile<'a, T> {
    pub source: &'a mut T,
}

impl<'a, T> RFile<'a, T> {
    fn new(source: &'a mut T) -> Self {
        RFile { source }
    }
}

impl<'a, T> Reader for RFile<'a, T>
where
    T: Read,
{
    fn read(&mut self) -> Option<u8> {
        let mut b = [0u8];
        match self.source.read_exact(&mut b) {
            Ok(_) => Some(b[0]),
            Err(_) => None,
        }
    }
}

pub fn run(target_path: &Path, vm_settings: VmSettings) {
    let mut program = File::open(target_path).unwrap();

    let mut reader = RFile::new(&mut program);
    let mut program_reader = ProgramReader::new(&mut reader);
    let program = match Program::build_from_reader(&mut program_reader) {
        Ok(e) => e,
        Err(e) => {
            println!(
                "{}Error:{} Failed to run program '{}{}{}'",
                cli_utils::Colors::Red,
                cli_utils::Colors::Reset,
                cli_utils::Colors::Yellow,
                read_error_text(e),
                cli_utils::Colors::Reset,
            );
            std::process::exit(1);
        }
    };

    let mut vm = VM::new(vm_settings.architecture, |native_message| {
        if native_message.module == "test" && native_message.name == "print" {
            let string = String::from_utf8(native_message.params[0].data.clone()).unwrap();
            println!("NativePrint: {}", string)
        }
        true
    });
    let main = program.main;
    vm.load(program);

    let output = vm.run(main);
    let dump = vm.heap_dump();

    match output {
        ellie_vm::utils::ThreadExit::Panic(e) => {
            println!(
                "\n{}ThreadPanic{} : {}{:?}{}",
                utils::Colors::Red,
                utils::Colors::Reset,
                utils::Colors::Cyan,
                e.reason,
                utils::Colors::Reset,
            );
            for frame in e.stack_trace {
                println!(
                    "{}    at {}:{}",
                    utils::Colors::Green,
                    frame.name,
                    frame.pos
                );
            }
        }
        ellie_vm::utils::ThreadExit::OutOfInstructions => todo!(),
        ellie_vm::utils::ThreadExit::ExitGracefully => {
            println!(
                "{}[VM]{}: Thread Exited Gracefully",
                utils::Colors::Green,
                utils::Colors::Reset
            );

            println!("======");
            println!(
                "{}[VM]{}: Heap Dump\n\n{}",
                utils::Colors::Yellow,
                utils::Colors::Reset,
                dump
            );
        }
    }
}
