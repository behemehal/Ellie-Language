use ellie_bytecode::assembler::{DebugHeader, DebugHeaderType};
use ellie_core::defs::CursorPosition;
use ellie_vm::{program::Program, utils};
use std::io::{Read, Write};
use std::path::Path;
use std::{fs::File, io};

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

    let mut dbg_headers = dbg_file.split("\n").collect::<Vec<_>>().into_iter();
    let mut module_maps_ended = false;

    let mut module_maps = Vec::new();

    while let Some(line) = dbg_headers.next() {
        if line == "---" {
            module_maps_ended = true;
            break;
        } else {
            let line = line.split(":").collect::<Vec<_>>();
            let module_name = line[0];
            let path = line[1].trim();
            module_maps.push((module_name, if path == "-" { None } else { Some(path) }));
        }
    }

    if !module_maps_ended {
        println!(
            "{}[Error]{}: Broken debug header, line: {}",
            utils::Colors::Red,
            utils::Colors::Reset,
            module_maps.len(),
        );
    }

    let dbg_headers = dbg_headers
        .enumerate()
        .map(|(idx, s)| {
            let line = s.split(":").collect::<Vec<_>>();
            if line.len() != 9 {
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
                    line[0].parse::<usize>().unwrap_or_else(|_| {
                        println!(
                            "{}[Error]{}: Broken debug header, line: {}",
                            utils::Colors::Red,
                            utils::Colors::Reset,
                            idx + 1
                        );
                        std::process::exit(1);
                    }),
                    line[1].parse::<usize>().unwrap_or_else(|_| {
                        println!(
                            "{}[Error]{}: Broken debug header, line: {}",
                            utils::Colors::Red,
                            utils::Colors::Reset,
                            idx + 1
                        );
                        std::process::exit(1);
                    }),
                ),
                module: line[2].to_string(),
                name: line[3].to_string(),
                pos: ellie_core::defs::Cursor {
                    range_start: CursorPosition(
                        line[4].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                        line[5].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                    ),
                    range_end: CursorPosition(
                        line[6].parse::<usize>().unwrap_or_else(|_| {
                            println!(
                                "{}[Error]{}: Broken debug header",
                                utils::Colors::Red,
                                utils::Colors::Reset
                            );
                            std::process::exit(1);
                        }),
                        line[7].parse::<usize>().unwrap_or_else(|_| {
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
                hash: line[8].parse().unwrap_or_else(|_| {
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

    let mut vm = VM::new(vm_settings.architecture, |thread_info, native_message| {
        if native_message.module == "test" && native_message.name == "println" {
            io::stdout()
                .write_all(&native_message.params[0].data)
                .unwrap();
        } else if native_message.module == "ellieStd" && native_message.name == "print" {
            io::stdout()
                .write_all(&native_message.params[0].data)
                .unwrap();
        } else if native_message.module == "ellieStd" && native_message.name == "println" {
            io::stdout()
                .write_all(&native_message.params[0].data)
                .unwrap();
            io::stdout().write_all(b"\n").unwrap();
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
                        let module_name = e
                            .module
                            .split("<ellie_module_")
                            .nth(1)
                            .unwrap()
                            .split(">")
                            .nth(0)
                            .unwrap();
                        let module_path =
                            module_maps.iter().find(|(mname, _)| module_name == *mname);
                        let real_path = match module_path {
                            Some(module_path) => match module_path.1 {
                                Some(module_path) => {
                                    let new_path = e.module.clone();
                                    let starter_name = format!("<ellie_module_{}>", module_name);
                                    new_path.replace(&starter_name, module_path)
                                }
                                None => e.module.clone(),
                            },
                            None => e.module.clone(),
                        };

                        println!(
                            "{}    at {}:{}:{}",
                            utils::Colors::Green,
                            real_path,
                            e.pos.range_start.0 + 1,
                            e.pos.range_start.1 + 1,
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
            #[cfg(feature = "debug")]
            println!(
                "{}[VM]{}: Thread Exited Gracefully",
                utils::Colors::Green,
                utils::Colors::Reset
            );

            println!(
                "{}[VM]{}: Heap Dump\n\n{}",
                utils::Colors::Yellow,
                utils::Colors::Reset,
                dump
            );
        }
    }
}
