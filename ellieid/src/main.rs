fn main() {}
/*
fn main() {
    let cli_color = &CliColor;
    println!(
        "{}EllieVM - Interactive Debugger{}: {}Type 'help' for help{}\n",
        Colors::Green,
        Colors::Reset,
        Colors::Yellow,
        Colors::Reset
    );

    pub struct EidOptions {
        CodePreview: bool,
        HeapPreview: bool,
        StackPreview: bool,
        RegistersPreview: bool,
    }

    pub enum DebuggerState {
        WaitingProgram,
        WaitingAtBreakpoint,
        WaitingAtStackPoint,
        Running,
        ProgramLoaded,
        ProgramCompleted,
    }

    let mut state = DebuggerState::WaitingProgram;

    let mut debug_info: Option<ellie_core::defs::DebugInfo> = None;
    let mut program: Option<ellie_vm::program::Program> = None;

    let mut options = EidOptions {
        CodePreview: true,
        HeapPreview: true,
        StackPreview: true,
        RegistersPreview: true,
    };

    fn new_prompt() {
        std::io::stdout().write_all("> ".as_bytes()).unwrap();
        std::io::stdout().flush().unwrap();
    }

    let commands = vec![
        EidCommand {
            short: "e",
            long: "exit",
            help: "Exit the program",
            command: EidCommands::Exit,
            args: vec![],
        },
        EidCommand {
            short: "h",
            long: "help",
            help: "Show this help message",
            command: EidCommands::Help,
            args: vec![EidArg {
                name: "command",
                value_type: EidArgTypes::String,
                optional: true,
            }],
        },
        EidCommand {
            short: "c",
            long: "clear",
            help: "Clear the screen",
            command: EidCommands::Clear,
            args: vec![],
        },
        EidCommand {
            short: "cp",
            long: "code-preview",
            help: "Show the code preview",
            command: EidCommands::CodePreview,
            args: vec![EidArg {
                name: "show",
                value_type: EidArgTypes::Bool,
                optional: false,
            }],
        },
        EidCommand {
            short: "hp",
            long: "heap-preview",
            help: "Show the heap preview",
            command: EidCommands::HeapPreview,
            args: vec![EidArg {
                name: "show",
                value_type: EidArgTypes::Bool,
                optional: false,
            }],
        },
        EidCommand {
            short: "sp",
            long: "stack-preview",
            help: "Show the stack preview",
            command: EidCommands::StackPreview,
            args: vec![EidArg {
                name: "show",
                value_type: EidArgTypes::Bool,
                optional: false,
            }],
        },
        EidCommand {
            short: "rp",
            long: "registers-preview",
            help: "Show the registers preview",
            command: EidCommands::RegistersPreview,
            args: vec![EidArg {
                name: "show",
                value_type: EidArgTypes::Bool,
                optional: false,
            }],
        },
        EidCommand {
            short: "pi",
            long: "program-info",
            help: "Give info about loaded program",
            command: EidCommands::ProgramInfo,
            args: vec![],
        },
        EidCommand {
            short: "l",
            long: "load",
            help: "Load a program",
            command: EidCommands::Load,
            args: vec![EidArg {
                name: "file",
                value_type: EidArgTypes::String,
                optional: false,
            }],
        },
        EidCommand {
            short: "rh",
            long: "reset-heap",
            help: "Reset the heap",
            command: EidCommands::ResetHeap,
            args: vec![],
        },
        EidCommand {
            short: "rs",
            long: "reset-stack",
            help: "Reset the stack",
            command: EidCommands::ResetStack,
            args: vec![],
        },
        EidCommand {
            short: "ra",
            long: "reset-all",
            help: "Reset the heap and stack",
            command: EidCommands::ResetAll,
            args: vec![],
        },
        EidCommand {
            short: "r",
            long: "run",
            help: "Run the program",
            command: EidCommands::Run,
            args: vec![],
        },
        EidCommand {
            short: "w",
            long: "wait",
            help: "Wait program at given stack position",
            command: EidCommands::Wait,
            args: vec![EidArg {
                name: "stack_pos",
                value_type: EidArgTypes::Int,
                optional: false,
            }],
        },
        EidCommand {
            short: "s",
            long: "step",
            help: "Run the program one step",
            command: EidCommands::Step,
            args: vec![],
        },
        EidCommand {
            short: "sb",
            long: "step-back",
            help: "Run the program one step back",
            command: EidCommands::StepBack,
            args: vec![],
        },
        EidCommand {
            short: "ghv",
            long: "get-heap-value",
            help: "Get the value of the heap at given position",
            command: EidCommands::GetHeapValue,
            args: vec![EidArg {
                name: "heap_pos",
                value_type: EidArgTypes::Int,
                optional: false,
            }],
        },
        EidCommand {
            short: "gsv",
            long: "get-stack-value",
            help: "Get the value of the stack at given position",
            command: EidCommands::GetStackValue,
            args: vec![EidArg {
                name: "stack_pos",
                value_type: EidArgTypes::Int,
                optional: false,
            }],
        },
        EidCommand {
            short: "gr",
            long: "get-register",
            help: "Get the value of the register",
            command: EidCommands::GetRegister,
            args: vec![EidArg {
                name: "register",
                value_type: EidArgTypes::String,
                optional: false,
            }],
        },
        EidCommand {
            short: "grs",
            long: "get-registers",
            help: "Get the values of all registers",
            command: EidCommands::GetRegisters,
            args: vec![],
        },
        EidCommand {
            short: "gh",
            long: "get-heap",
            help: "Get the heap",
            command: EidCommands::GetHeap,
            args: vec![],
        },
        EidCommand {
            short: "gcp",
            long: "get-code-pos",
            help: "Get the code position",
            command: EidCommands::GetCodePos,
            args: vec![],
        },
        EidCommand {
            short: "sc",
            long: "step-changes",
            help: "Show the changes between the last step",
            command: EidCommands::StepChanges,
            args: vec![],
        },
    ];

    loop {
        new_prompt();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let command = input
            .trim()
            .split(" ")
            .map(|x| x.to_string())
            .collect::<Vec<String>>();
        let args = command[1..].to_vec();
        let _command = &command[0];
        let args = parse_args(args);
        let command = parse_command(_command, args.clone());

        match command {
            Ok(e) => match e.command {
                EidCommands::Help => {
                    if e.args.len() == 0 {
                        println!("Available commands:");
                        for command in &commands {
                            println!(
                                "Command    : {} | {}\nDescription: {}\n",
                                command.short, command.long, command.help
                            );
                        }
                    } else {
                        match &e.args[0].value_type {
                            BuildEidArgTypes::String(command_type) => {
                                let help_command = commands
                                    .iter()
                                    .find(|x| x.short == *command_type || x.long == *command_type);
                                match help_command {
                                    Some(command) => {
                                        println!(
                                            "Command    : {} | {}\nDescription: {}\n",
                                            command.short, command.long, command.help
                                        );
                                    }
                                    None => {
                                        println!("Command not found");
                                    }
                                }
                            }
                            _ => {
                                println!(
                                    "{}Invalid argument type{}: Expected String",
                                    Colors::Red,
                                    Colors::Reset
                                );
                            }
                        }
                    }
                }
                EidCommands::Exit => {
                    println!("Bye...");
                    break;
                }
                EidCommands::Clear => {
                    print!("{}[2J", 27 as char);
                }
                EidCommands::CodePreview => {
                    options.CodePreview = !options.CodePreview;
                    println!(
                        "Code preview is now {}'{}'{}",
                        if options.CodePreview {
                            Colors::Green
                        } else {
                            Colors::Red
                        },
                        if options.CodePreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        Colors::Reset
                    );
                }
                EidCommands::HeapPreview => {
                    options.HeapPreview = !options.HeapPreview;
                    println!(
                        "Heap preview is now {}'{}'{}",
                        if options.HeapPreview {
                            Colors::Green
                        } else {
                            Colors::Red
                        },
                        if options.HeapPreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        Colors::Reset
                    );
                }
                EidCommands::StackPreview => {
                    options.StackPreview = !options.StackPreview;
                    println!(
                        "Stack preview is now {}'{}'{}",
                        if options.StackPreview {
                            Colors::Green
                        } else {
                            Colors::Red
                        },
                        if options.StackPreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        Colors::Reset
                    );
                }
                EidCommands::RegistersPreview => {
                    options.RegistersPreview = !options.RegistersPreview;
                    println!(
                        "Registers preview is now {}'{}'{}",
                        if options.RegistersPreview {
                            Colors::Green
                        } else {
                            Colors::Red
                        },
                        if options.RegistersPreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        Colors::Reset
                    );
                }
                EidCommands::Load => {
                    let path = match &e.args[0].value_type {
                        BuildEidArgTypes::String(path) => Path::new(path),
                        _ => {
                            println!(
                                "{}Invalid argument type{}: Expected String",
                                Colors::Red,
                                Colors::Reset
                            );
                            continue;
                        }
                    };
                    let debug_file_path = match &e.args[0].value_type {
                        BuildEidArgTypes::String(path) => {
                            let path = Path::new(path);
                            if path.is_file() {
                                let file_name =
                                    path.file_name().unwrap().to_str().unwrap().to_string();
                                let file_name = file_name.split(".").collect::<Vec<&str>>();
                                let file_name = file_name[0].to_string();
                                let mut path =
                                    path.parent().unwrap().to_str().unwrap().to_string().clone();
                                path.push_str(&format!("/{file_name}.eig"));
                                Some(path)
                            } else {
                                None
                            }
                        }
                        _ => None,
                    };

                    if path.is_file() {
                        match File::open(path) {
                            Ok(mut e) => {
                                let mut reader = ellie_engine::vm::RFile::new(&mut e);
                                match ellie_engine::vm::read_program(&mut reader) {
                                    Ok(e) => {
                                        program = Some(e);
                                        state = DebuggerState::ProgramLoaded;
                                    }
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed to read program, error code: {}{}{}",
                                            Colors::Red,
                                            Colors::Reset,
                                            Colors::Cyan,
                                            e,
                                            Colors::Reset
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Failed to read file {}[{}]{}",
                                    Colors::Red,
                                    Colors::Reset,
                                    Colors::Cyan,
                                    e,
                                    Colors::Reset
                                );
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!(
                            "{}Invalid path{}: Not a file",
                            Colors::Red,
                            Colors::Reset
                        );
                        continue;
                    }
                    println!(
                        "{}Program loaded{}: {}{}{}",
                        Colors::Green,
                        Colors::Reset,
                        Colors::Cyan,
                        path.to_str().unwrap(),
                        Colors::Reset
                    );
                    match debug_file_path {
                        Some(path) => {
                            let path = Path::new(&path);
                            if path.is_file() {
                                let mut file_contents = String::new();
                                match File::open(path) {
                                    Ok(mut e) => {
                                        e.read_to_string(&mut file_contents).unwrap();
                                        match parse_debug_file(file_contents) {
                                            Ok(e) => {
                                                debug_info = Some(e);
                                                println!(
                                                    "{}Debug file loaded{}: {}{}{}",
                                                    Colors::Green,
                                                    Colors::Reset,
                                                    Colors::Cyan,
                                                    path.to_str().unwrap(),
                                                    Colors::Reset
                                                );
                                            }
                                            Err(e) => {
                                                println!(
                                                    "{}Error:{} {}",
                                                    Colors::Red,
                                                    Colors::Reset,
                                                    e
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed to read file {}[{}]{}",
                                            Colors::Red,
                                            Colors::Reset,
                                            Colors::Cyan,
                                            e,
                                            Colors::Reset
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                        }
                        None => (),
                    }
                }
                EidCommands::ResetHeap => todo!(),
                EidCommands::ResetStack => todo!(),
                EidCommands::ResetAll => todo!(),
                EidCommands::Run => todo!(),
                EidCommands::Wait => todo!(),
                EidCommands::Step => todo!(),
                EidCommands::StepBack => todo!(),
                EidCommands::GetHeapValue => todo!(),
                EidCommands::GetStackValue => todo!(),
                EidCommands::GetRegister => todo!(),
                EidCommands::GetRegisters => todo!(),
                EidCommands::GetHeap => todo!(),
                EidCommands::GetCodePos => todo!(),
                EidCommands::StepChanges => todo!(),
                EidCommands::ProgramInfo => match &program {
                    Some(e) => {
                        println!(
                            "{}Program info{}:",
                            Colors::Green,
                            Colors::Reset,
                        );
                        println!(
                            "{}- Arch{}: {}",
                            Colors::Yellow,
                            Colors::Reset,
                            e.arch
                        );
                        println!(
                            "{}- Debug Info{}: {}{}{}",
                            Colors::Yellow,
                            Colors::Reset,
                            match debug_info {
                                Some(_) => Colors::Green,
                                None => Colors::Red,
                            },
                            match debug_info {
                                Some(_) => "Available",
                                None => "Not available",
                            },
                            Colors::Reset
                        );
                    }
                    None => {
                        println!(
                            "{}Error:{} No program loaded",
                            Colors::Red,
                            Colors::Reset
                        );
                    }
                },
            },
            Err(e) => {
                if e == 0 {
                    if _command == "" {
                        println!("TODO STEP FORWARD")
                    } else {
                        println!(
                            "{}Unknown command: {}{}\n",
                            Colors::Red,
                            _command,
                            Colors::Reset
                        );
                    }
                } else if e == 1 {
                    println!(
                        "{}Invalid argument length{}: Check syntax of the command",
                        Colors::Red,
                        Colors::Reset
                    );
                } else if e == 2 {
                    println!(
                        "{}Invalid argument type{}: Check syntax of the command",
                        Colors::Red,
                        Colors::Reset
                    );
                }
            }
        }
    }
}
*/