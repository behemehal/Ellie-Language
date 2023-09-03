use std::{
    collections::{HashMap, HashSet},
    fs::File,
    io::{self, Read, Write},
};

use ellie_engine::{
    ellie_core::defs::PlatformArchitecture,
    ellie_vm::{
        channel::ModuleManager,
        program::{Program, VmProgram},
        thread::{Isolate, Thread},
        utils::{ProgramReader, StepResult, ThreadExit},
    },
    vm::{parse_debug_file, RFile},
};

use crate::{
    commands::{parse_args, parse_command, BuildDebuggerArgTypes, DebuggerCommands, COMMANDS},
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
        vm_program: None,
        program: None,
        debug_file: None,
        module_manager: ModuleManager::new(),
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
        if debugger_state.state == DebuggerState::Running {
            let thread = debugger_state.thread.as_mut().unwrap();
            let current_stack = thread.stack.last();
            if let Some(current_stack) = current_stack {
                let break_point = debugger_state
                    .breakpoints
                    .iter()
                    .find(|break_point| break_point.stack_location == current_stack.pos);
                if let Some(break_point) = break_point {
                    debugger_state.state =
                        DebuggerState::WaitingAtBreakpoint((*break_point).clone());
                    output_message(&HIT_BREAKPOINT);
                }
            }

            let step_info = thread.step(
                &mut debugger_state.module_manager,
                debugger_state.vm_program.as_mut().unwrap(),
            );

            match &step_info {
                StepResult::Step => (),
                StepResult::ThreadExit(e) => {
                    match e {
                        ThreadExit::Panic(panic) => {
                            output_message(&{
                                let mut thread_panic = THREAD_PANIC.clone();
                                let mut variables = HashMap::new();
                                variables.insert(
                                    "panic_reason".to_string(),
                                    format!("{:?}", panic.reason),
                                );
                                variables.insert(
                                    "panic_code_location".to_string(),
                                    format!("{:?}", panic.code_location),
                                );
                                variables.insert(
                                    "panic_code_location".to_string(),
                                    format!("{:?}", panic.stack_trace),
                                );
                                thread_panic.variables = Some(variables);
                                thread_panic
                            });
                        }
                        ThreadExit::ExitGracefully => {
                            output_message(&THREAD_EXITED_GRACEFULLY);
                        }
                    }
                    debugger_state.state = DebuggerState::ProgramCompleted;
                }
            }
            continue;
        }

        let mut input = String::new();
        if !json_output && stream_len != 0 {
            std::io::stdout().write_all("> ".as_bytes()).unwrap();
            std::io::stdout().flush().unwrap();
        }

        let (read_len, is_external_line) =
            stdin.read_line(&mut input).expect("Failed to read input");
        input = input
            .to_string()
            .replace("\n", "")
            .replace("\r", "")
            .trim()
            .to_string();

        if is_external_line && read_len > 0 {
            std::io::stdout().write_all(input.as_bytes()).unwrap();
            std::io::stdout().flush().unwrap();
        }

        if stream_len == 0 {
            continue;
        }

        if input.is_empty() {
            continue;
        }
        if !json_output {
            println!("");
        }
        let args = input.split_whitespace().collect::<Vec<&str>>();
        let command = &args[0];
        let args = args[1..].to_vec();
        let args = parse_args(args);
        let command = parse_command(command, args.clone(), json_output);

        match command {
            Ok(matched) => match matched.command {
                DebuggerCommands::Help => {
                    if matched.args.len() == 0 {
                        output_message(&EllieMessage::new("log", "Available commands:\n", -1));
                        for command in COMMANDS.iter() {
                            output_message(&EllieMessage::new_with_variables(
                                "log",
                                "Command    : {command} | {long}\nDescription: {description}\n",
                                -1,
                                {
                                    let mut map = HashMap::new();
                                    map.insert("command".to_string(), command.short.to_string());
                                    map.insert("long".to_string(), command.long.to_string());
                                    map.insert("description".to_string(), command.help.to_string());
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
                                                    map.insert("command".to_string(), command.short.to_string());
                                                    map.insert("long".to_string(), command.long.to_string());
                                                    map.insert(
                                                        "description".to_string(),
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
                                                        "command".to_string(),
                                                        command.short.to_string(),
                                                    );
                                                    map.insert(
                                                        "long".to_string(),
                                                        command.long.to_string(),
                                                    );
                                                    map.insert(
                                                        "description".to_string(),
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
                DebuggerCommands::Exit => {
                    output_message(&EXIT_MESSAGE);
                    break;
                }
                DebuggerCommands::Clear => {
                    input = String::new();
                    print!("{}[2J", 27 as char);
                }
                DebuggerCommands::Load => {
                    let main_file = match &matched.args[0].value_type {
                        BuildDebuggerArgTypes::String(file_path) => file_path.to_string(),
                        _ => unreachable!(),
                    };

                    let debug_file = match &matched.args[1].value_type {
                        BuildDebuggerArgTypes::String(debug_file) => debug_file.to_string(),
                        _ => unreachable!(),
                    };

                    let mut main_file = match File::open(&main_file) {
                        Ok(e) => e,
                        Err(e) => {
                            output_message(&EllieMessage::new_with_variables(
                                "error",
                                "Failed to read the file: '{file}' ({error})",
                                5,
                                {
                                    let mut hash_map = HashMap::new();
                                    hash_map.insert("file".to_string(), main_file);
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

                    let mut debug_file: File = match File::open(&debug_file) {
                        Ok(e) => e,
                        Err(e) => {
                            output_message(&EllieMessage::new_with_variables(
                                "error",
                                "Failed to read the debug file: '{file}' ({error})",
                                5,
                                {
                                    let mut hash_map = HashMap::new();
                                    hash_map.insert("file".to_string(), debug_file);
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
                                    "Failed to parse debug file: {error}",
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
                    debugger_state.vm_program = Some(vm_program);
                    debugger_state.debug_file = Some(debug_file);

                    let isolate = Isolate::new();
                    debugger_state.thread =
                        Some(Thread::new(main_hash, PlatformArchitecture::B64, isolate));
                    debugger_state.state = DebuggerState::ProgramLoaded;
                    output_message(&PROGRAM_LOADED);
                }
                DebuggerCommands::Run => {
                    if debugger_state.state == DebuggerState::ProgramLoaded {
                        debugger_state
                            .thread
                            .as_mut()
                            .unwrap()
                            .build_thread(debugger_state.program.as_ref().unwrap().main.clone());
                        debugger_state.state = DebuggerState::Running;
                    } else {
                        output_message({
                            let mut wrong_state = DEBUGER_IS_NOT_ON_EXPECTED_STATE.clone();
                            wrong_state.variables = {
                                let mut variables = HashMap::new();
                                variables.insert(
                                    "current_state".to_string(),
                                    debugger_state.state.to_string().to_owned(),
                                );
                                variables.insert(
                                    "expected_state".to_string(),
                                    DebuggerState::ProgramLoaded.to_string().to_owned(),
                                );
                                Some(variables)
                            };
                            &wrong_state.clone()
                        });
                    }
                }
                DebuggerCommands::Wait => {
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

                    let break_point = {
                        if *use_stack_pos {
                            BreakPoint {
                                stack_location: *pos as usize,
                                code_location: None,
                                module_name: None,
                            }
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
                            let filtered_headers_by_path = filtered_headers_by_path.unwrap();
                            BreakPoint {
                                module_name: Some(filtered_headers_by_path.module_name.to_string()),
                                stack_location: filtered_headers_by_path.start_end.0,
                                code_location: Some(*pos as usize),
                            }
                        }
                    };
                    debugger_state.breakpoints.push(break_point);
                    output_message(&BREAKPOINT_ADDED);
                }
                DebuggerCommands::Step => {
                    if let DebuggerState::WaitingAtBreakpoint(_) = debugger_state.state {
                        output_message(&STEP_FORWARD);
                        debugger_state.state = DebuggerState::Running;
                    } else {
                        output_message(&NOT_IN_BREAKPOINT);
                    }
                }
                DebuggerCommands::ReloadVm => todo!(),
                DebuggerCommands::ReadAtPosition => {}
                DebuggerCommands::GetPaths => {
                    if debugger_state.state == DebuggerState::ProgramNotLoaded {
                        output_message(&PROGRAM_NOT_LOADED);
                        continue;
                    }

                    output_message(&GET_PATHS_START);

                    let debug_headers = &debugger_state.debug_file.as_ref().unwrap().debug_headers;
                    let mut modules = HashSet::new();

                    for header in debug_headers {
                        let is_new = modules.insert(header.module_name.clone());
                        if is_new {
                            output_message({
                                let mut module_entry = GET_PATHS_ENTRY.clone();
                                module_entry.variables = {
                                    let mut variables = HashMap::new();
                                    variables.insert(
                                        "module_name".to_string(),
                                        header
                                            .module_name
                                            .split("<ellie_module_")
                                            .last()
                                            .unwrap()
                                            .split('>')
                                            .next()
                                            .unwrap()
                                            .to_string(),
                                    );
                                    variables.insert(
                                        "module_path".to_string(),
                                        format!(
                                            "/{}",
                                            header.module_name.split('/').collect::<Vec<_>>()[1..]
                                                .join("/"),
                                        ),
                                    );
                                    variables.insert(
                                        "module_file_path".to_string(),
                                        header.module_name.clone(),
                                    );
                                    Some(variables)
                                };
                                &module_entry.clone()
                            });
                        }
                    }

                    output_message(&GET_PATHS_END);
                }
                DebuggerCommands::GetBreakpoints => {
                    if debugger_state.state == DebuggerState::ProgramNotLoaded {
                        output_message(&PROGRAM_NOT_LOADED);
                        continue;
                    }
                    output_message(&GET_BREAKPOINTS_START);
                    for breakpoint in debugger_state.breakpoints.iter() {
                        output_message({
                            let mut module_entry = GET_BREAKPOINTS_ENTRY.clone();
                            module_entry.variables = {
                                let mut variables = HashMap::new();
                                variables.insert(
                                    "module_file_path".to_string(),
                                    breakpoint.module_name.clone().unwrap_or_default(),
                                );
                                variables.insert(
                                    "code_location".to_string(),
                                    breakpoint.code_location.unwrap_or_default().to_string(),
                                );
                                variables.insert(
                                    "stack_location".to_string(),
                                    breakpoint.stack_location.to_string(),
                                );
                                Some(variables)
                            };
                            &module_entry.clone()
                        });
                    }

                    output_message(&GET_BREAKPOINTS_END);
                }
                DebuggerCommands::GetRegisters => {
                    if let DebuggerState::WaitingAtBreakpoint(_) = debugger_state.state {
                        output_message(&GET_REGISTERS_START);
                        let thread = debugger_state.thread.as_ref().unwrap();
                        let stack = thread.stack.last().unwrap();

                        let registers = [
                            ("A", stack.registers.A),
                            ("B", stack.registers.B),
                            ("C", stack.registers.C),
                            ("X", stack.registers.X),
                            ("Y", stack.registers.Y),
                        ];

                        for register in &registers {
                            output_message({
                                let mut module_entry = GET_REGISTERS_ENTRY.clone();
                                module_entry.variables = {
                                    let mut variables = HashMap::new();
                                    variables.insert(
                                        "register_name".to_string(),
                                        register.0.to_string(),
                                    );
                                    variables.extend(render_static_raw_type(register.1));
                                    Some(variables)
                                };
                                &module_entry.clone()
                            });
                        }
                        output_message(&GET_REGISTERS_END);
                    } else {
                        output_message(&NOT_IN_BREAKPOINT);
                        continue;
                    }
                }
                DebuggerCommands::GetStackMemory => {
                    if let DebuggerState::WaitingAtBreakpoint(_) = debugger_state.state {
                        output_message(&GET_STACK_MEMORY_START);
                        let isolate = &debugger_state.thread.as_ref().unwrap().isolate;

                        for (stack_location, static_raw_type) in
                            isolate.stack_memory.data.iter().enumerate()
                        {
                            if !static_raw_type.type_id.is_void() {
                                output_message({
                                    let mut module_entry = GET_STACK_MEMORY_ENTRY.clone();
                                    module_entry.variables = {
                                        let mut variables = HashMap::new();
                                        variables.insert(
                                            "stack_location".to_string(),
                                            stack_location.to_string(),
                                        );
                                        variables.extend(render_static_raw_type(*static_raw_type));
                                        Some(variables)
                                    };
                                    &module_entry.clone()
                                });
                            }
                        }
                        output_message(&GET_STACK_MEMORY_END);
                    } else {
                        output_message(&NOT_IN_BREAKPOINT);
                        continue;
                    }
                }
                DebuggerCommands::GetHeapMemory => {
                    if let DebuggerState::WaitingAtBreakpoint(_) = debugger_state.state {
                        output_message(&GET_HEAP_MEMORY_START);
                        let isolate = &debugger_state.thread.as_ref().unwrap().isolate;
                        for (heap_location, heap_entry) in &isolate.heap_memory.data {
                            output_message({
                                let mut module_entry = GET_STACK_MEMORY_ENTRY.clone();
                                module_entry.variables = {
                                    let mut variables = HashMap::new();
                                    variables.insert(
                                        "heap_location".to_string(),
                                        heap_location.to_string(),
                                    );
                                    variables
                                        .insert("data".to_string(), format!("{:?}", heap_entry));
                                    Some(variables)
                                };
                                &module_entry.clone()
                            });
                        }
                        output_message(&GET_HEAP_MEMORY_END);
                    } else {
                        output_message(&NOT_IN_BREAKPOINT);
                        continue;
                    }
                }
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
