use ellie_bytecode::assembler::{DebugHeader, DebugHeaderType};
use ellie_core::defs::CursorPosition;
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

pub fn run(target_path: &Path, dbg_target_path: &Path, vm_settings: VmSettings) {
    let mut program = File::open(target_path).unwrap();

    let mut dbg_file = String::new();
    File::open(dbg_target_path)
        .unwrap()
        .read_to_string(&mut dbg_file)
        .unwrap();

    let dbg_headers = dbg_file
        .split("\n")
        .collect::<Vec<_>>()
        .iter()
        .enumerate()
        .map(|(idx, s)| {
            let logs = s.split(":").collect::<Vec<_>>();
            if logs.len() != 9 {
                println!(
                    "{}[Error]{}: Broken debug header, line: {}",
                    utils::Colors::Red,
                    utils::Colors::Reset,
                    idx + 1
                );
                std::process::exit(1);
            }
            DebugHeader {
                start_end: (
                    logs[0].parse::<usize>().unwrap_or_else(|_| {
                        println!(
                            "{}[Error]{}: Broken debug header, line: {}",
                            utils::Colors::Red,
                            utils::Colors::Reset,
                            idx + 1
                        );
                        std::process::exit(1);
                    }),
                    logs[1].parse::<usize>().unwrap_or_else(|_| {
                        println!(
                            "{}[Error]{}: Broken debug header, line: {}",
                            utils::Colors::Red,
                            utils::Colors::Reset,
                            idx + 1
                        );
                        std::process::exit(1);
                    }),
                ),
                module: logs[2].to_string(),
                name: logs[3].to_string(),
                pos: ellie_core::defs::Cursor {
                    range_start: CursorPosition(
                        logs[4].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                        logs[5].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                    ),
                    range_end: CursorPosition(
                        logs[6].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                        logs[7].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                    ),
                },
                rtype: DebugHeaderType::Variable,
                hash: logs[8].parse().unwrap_or_else(|_| {
                    println!(
                        "{}[Error]{}: Broken debug header",
                        utils::Colors::Red,
                        utils::Colors::Reset
                    );
                    std::process::exit(1);
                }),
            }
        })
        .collect::<Vec<_>>();

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
                let coresponding_header = dbg_headers
                    .iter()
                    .find(|x| frame.pos >= x.start_end.0 && frame.pos <= x.start_end.1);

                match coresponding_header {
                    Some(e) => {
                        println!(
                            "{}    at Thread: {}, File: {}:{}:{} - {}:{}~{}:{}",
                            utils::Colors::Green,
                            frame.name,
                            e.module,
                            e.pos.range_start.0 + 1,
                            e.pos.range_start.1 + 1,
                            e.pos.range_start.0 + 1,
                            e.pos.range_start.1 + 1,
                            e.pos.range_end.0 + 1,
                            e.pos.range_end.1 + 1,
                        );
                    }
                    None => {
                        println!(
                            "{}    at {}:{}",
                            utils::Colors::Green,
                            frame.name,
                            frame.pos
                        );
                    }
                }
            }
        }
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
