use std::{
    collections::HashMap,
    fs::File,
    io::{self, BufRead, Read, Write},
};

use ellie_engine::{
    ellie_core::defs::{DebugHeader, DebugInfo, PlatformArchitecture},
    ellie_renderer_utils::utils::{read_file, CliColor, ColorDisplay, Colors},
    ellie_vm::{
        channel::{EllieModule, ModuleManager},
        program::{Program, VmProgram},
        thread::{Isolate, Thread},
        utils::{ProgramReader, ThreadExit},
    },
    vm::{parse_debug_file, RFile},
};

use crate::{
    commands::{parse_args, parse_command, BuildDebuggerArgTypes, COMMANDS},
    debugger_messages::*,
    stream::InputStream,
    utils::{BreakPoint, DebuggerState, DebuggerStatus},
};

pub fn debug(json_output: bool, imported_commands: Vec<String>) {
    let output_message = |message: &EllieMessage| {
        if json_output {
            println!("{}", message.build_json_message());
        } else {
            println!("{}", message.build_plain_message());
        }
    };

    let reset_cursor = |command: Option<&String>| {
        print!("\x1B[1A\x1B[2K");
        io::stdout().flush().unwrap();
        if let Some(command) = command {
            print!("> {}\n", command.to_string());
        } else {
            std::io::stdout().write_all(">".as_bytes()).unwrap();
            std::io::stdout().flush().unwrap();
        }
    };

    //let mut module_manager = ModuleManager::new();
    ////Register incoming modules
    //for module in debugger_settings.modules {
    //    module_manager.register_module(module);
    //}

    let mut debugger_state = DebuggerStatus {
        program: None,
        debug_file: None,
        thread: None,
        state: DebuggerState::ProgramNotLoaded,
        breakpoints: Vec::new(),
    };

    let mut binding = io::stdin().lock();
    let mut stdin = InputStream::new(&mut binding);
    stdin.external_lines = imported_commands;
    let mut stream_len = 1;

    output_message(&READY);

    loop {
        let mut input = String::new();
        if !json_output && stream_len != 0 {
            std::io::stdout().write_all("> ".as_bytes()).unwrap();
            std::io::stdout().flush().unwrap();
        }

        stream_len = stdin.read_line(&mut input).expect("Failed to read input");
        input = input
            .to_string()
            .replace("\n", "")
            .replace("\r", "")
            .trim()
            .to_string();

        if stream_len == 0 {
            continue;
        }

        if input.is_empty() {
            continue;
        }
        println!("");
        let args = input.split_whitespace().collect::<Vec<&str>>();
        let command = &args[0];
        let args = args[1..].to_vec();
        let args = parse_args(args);
        let command = parse_command(command, args.clone(), json_output);

        match command {
            Ok(matched) => match matched.command {
                crate::commands::DebuggerCommands::Help => {
                    if matched.args.len() == 0 {
                        output_message(&EllieMessage::new("log", "Available commands:\n", -1));
                        for command in COMMANDS.iter() {
                            output_message(&EllieMessage::new_with_variables(
                                "log",
                                "Command    : {command} | {long}\nDescription: {description}\n",
                                -1,
                                {
                                    let mut map = HashMap::new();
                                    map.insert("{command}".to_string(), command.short.to_string());
                                    map.insert("{long}".to_string(), command.long.to_string());
                                    map.insert(
                                        "{description}".to_string(),
                                        command.help.to_string(),
                                    );
                                    map
                                },
                            ));
                        }
                    } else {
                        match &matched.args[0].value_type {
                            BuildDebuggerArgTypes::String(command_type) => {
                                let help_command = COMMANDS
                                    .iter()
                                    .find(|x| x.short == *command_type || x.long == *command_type);
                                match help_command {
                                    Some(command) => {
                                        if command.args.is_empty() {
                                            output_message(&EllieMessage::new_with_variables(
                                                "log",
                                                "Command    : {command} | {long}\nDescription: {description}\n",
                                                -1,
                                                {
                                                    let mut map = HashMap::new();
                                                    map.insert("{command}".to_string(), command.short.to_string());
                                                    map.insert("{long}".to_string(), command.long.to_string());
                                                    map.insert(
                                                        "{description}".to_string(),
                                                        command.help.to_string(),
                                                    );
                                                    map
                                                },
                                            ));
                                        } else {
                                            let mut message = "Command     : {command} | {long}\nDescription : {description}".to_string();
                                            let mut map = HashMap::new();
                                            map.insert(
                                                "{command}".to_string(),
                                                command.short.to_string(),
                                            );
                                            map.insert(
                                                "{long}".to_string(),
                                                command.long.to_string(),
                                            );
                                            map.insert(
                                                "{description}".to_string(),
                                                command.help.to_string(),
                                            );
                                            if !command.args.is_empty() {
                                                message += "\nUsage       : {long}";
                                                for arg in &command.args {
                                                    message += format!(
                                                        " ({}:{}{})",
                                                        arg.name,
                                                        arg.value_type,
                                                        if arg.optional {
                                                            " <optional>"
                                                        } else {
                                                            ""
                                                        }
                                                    )
                                                    .as_str();
                                                }
                                                message += "\n";
                                            }

                                            output_message(&EllieMessage::new_with_variables(
                                                "log",
                                                &message,
                                                -1,
                                                {
                                                    let mut map = HashMap::new();
                                                    map.insert(
                                                        "{command}".to_string(),
                                                        command.short.to_string(),
                                                    );
                                                    map.insert(
                                                        "{long}".to_string(),
                                                        command.long.to_string(),
                                                    );
                                                    map.insert(
                                                        "{description}".to_string(),
                                                        command.help.to_string(),
                                                    );
                                                    map
                                                },
                                            ));
                                        }
                                    }
                                    None => {
                                        output_message(&EllieMessage::new(
                                            "log",
                                            "Command Not found",
                                            -1,
                                        ));
                                    }
                                }
                            }
                            _ => unreachable!(),
                        }
                    }
                }
                crate::commands::DebuggerCommands::Exit => {
                    output_message(&EXIT_MESSAGE);
                    break;
                }
                crate::commands::DebuggerCommands::Clear => {
                    input = String::new();
                    print!("{}[2J", 27 as char);
                }
                crate::commands::DebuggerCommands::Load => {
                    let main_file = match &matched.args[0].value_type {
                        BuildDebuggerArgTypes::String(file_path) => file_path.to_string(),
                        _ => unreachable!(),
                    };

                    let debug_file = match &matched.args[1].value_type {
                        BuildDebuggerArgTypes::String(debug_file) => debug_file.to_string(),
                        _ => unreachable!(),
                    };

                    let mut main_file = match File::open(main_file) {
                        Ok(e) => e,
                        Err(e) => {
                            output_message(&EllieMessage::new_with_variables(
                                "error",
                                "Failed to read the file: {error}",
                                5,
                                {
                                    let mut hash_map = HashMap::new();
                                    hash_map.insert("error".to_string(), e.to_string());
                                    hash_map
                                },
                            ));
                            continue;
                        }
                    };

                    let main_file = {
                        let mut reader = RFile::new(&mut main_file);
                        let mut program_reader = ProgramReader::new(&mut reader);
                        let mut program = Program::new();
                        match program.build_from_reader(&mut program_reader) {
                            Ok(_) => program,
                            Err(e) => {
                                output_message(&EllieMessage::new_with_variables(
                                    "error",
                                    "Failed to read program: {error}",
                                    5,
                                    {
                                        let mut hash_map = HashMap::new();
                                        hash_map.insert("error".to_string(), format!("{:#?}", e));
                                        hash_map
                                    },
                                ));
                                continue;
                            }
                        }
                    };

                    let mut debug_file: File = match File::open(debug_file) {
                        Ok(e) => e,
                        Err(e) => {
                            output_message(&EllieMessage::new_with_variables(
                                "error",
                                "Failed to read the file: {error}",
                                5,
                                {
                                    let mut hash_map = HashMap::new();
                                    hash_map.insert("error".to_string(), e.to_string());
                                    hash_map
                                },
                            ));
                            continue;
                        }
                    };
                    let mut debug_file_text = String::new();
                    debug_file.read_to_string(&mut debug_file_text).unwrap();

                    let debug_file = {
                        match parse_debug_file(debug_file_text) {
                            Ok(e) => e,
                            Err(e) => {
                                output_message(&EllieMessage::new_with_variables(
                                    "error",
                                    "Failed to parse debug fie: {error}",
                                    7,
                                    {
                                        let mut hash_map = HashMap::new();
                                        hash_map.insert("error".to_string(), e.to_string());
                                        hash_map
                                    },
                                ));
                                continue;
                            }
                        }
                    };
                    let main_hash = main_file.main.hash;

                    let mut vm_program = VmProgram::new();
                    vm_program.fill_from_vector(main_file.instructions.clone());
                    vm_program.fill_traces(main_file.native_call_traces.clone());

                    debugger_state.program = Some(main_file);
                    debugger_state.debug_file = Some(debug_file);

                    let isolate = Isolate::new();
                    debugger_state.thread =
                        Some(Thread::new(main_hash, PlatformArchitecture::B64, isolate));
                    debugger_state.state = DebuggerState::ProgramLoaded;
                    output_message(&PROGRAM_LOADED);
                }
                crate::commands::DebuggerCommands::Run => todo!(),
                crate::commands::DebuggerCommands::Wait => {
                    if debugger_state.state == DebuggerState::ProgramNotLoaded {
                        output_message(&PROGRAM_NOT_LOADED);
                        continue;
                    }
                    let use_stack_pos = match &matched.args[0].value_type {
                        BuildDebuggerArgTypes::Bool(e) => e,
                        _ => unreachable!(),
                    };

                    let pos = match &matched.args[1].value_type {
                        BuildDebuggerArgTypes::Int(pos) => pos,
                        _ => unreachable!(),
                    };

                    let module_path = match &matched.args[2].value_type {
                        BuildDebuggerArgTypes::String(module_path) => module_path,
                        _ => unreachable!(),
                    };

                    let location = {
                        if *use_stack_pos {
                            *pos as usize
                        } else {
                            let debug_headers =
                                &debugger_state.debug_file.as_ref().unwrap().debug_headers;
                            let filtered_headers_by_path =
                                debug_headers.into_iter().find(|header| {
                                    header.module_name == module_path.to_string()
                                        && header.pos.range_start.0 == (*pos + -1) as usize
                                });

                            if filtered_headers_by_path.is_none() {
                                output_message(&CANT_FIND_ELEMENT_AT_LOCATION);
                                continue;
                            }
                            filtered_headers_by_path.unwrap().start_end.0
                        }
                    };

                    debugger_state.breakpoints.push(BreakPoint { location });
                    output_message(&BREAKPOINT_ADDED);
                }
                crate::commands::DebuggerCommands::Step => todo!(),
                crate::commands::DebuggerCommands::ReloadVm => todo!(),
                crate::commands::DebuggerCommands::ReadAtPosition => todo!(),
            },
            Err(error_code) => {
                if error_code == 0 {
                    output_message(&UNKNOWN_COMMAND);
                } else if error_code == 1 {
                    output_message(&INVALID_ARGUMENT_LENGTH);
                } else if error_code == 2 {
                    output_message(&INVALID_ARGUMENT_TYPE);
                } else if error_code == 3 {
                    output_message(&CANT_RENDER_INFO);
                }
            }
        }
    }
}
