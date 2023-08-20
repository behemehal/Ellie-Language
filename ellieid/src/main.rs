use std::{
    fs::File,
    io::{Read, Write},
    path::Path,
};

use ellie_engine::{
    ellie_core::defs::DebugInfo,
    ellie_renderer_utils::utils::{CliColor, ColorDisplay, Colors},
    ellie_vm::program::Program,
    vm::{parse_debug_file, RFile},
};


fn main() {
    let cli_color = &CliColor;
    println!(
        "{}EllieVM - Interactive Debugger{}: {}Type 'help' for help{}\n",
        cli_color.color(Colors::Green),
        cli_color.color(Colors::Reset),
        cli_color.color(Colors::Yellow),
        cli_color.color(Colors::Reset)
    );

    pub struct EidOptions {
        code_preview: bool,
        heap_preview: bool,
        stack_preview: bool,
        registers_preview: bool,
    }

    pub enum DebuggerState {
        WaitingProgram,
        _WaitingAtBreakpoint,
        _WaitingAtStackPoint,
        _Running,
        ProgramLoaded,
        _ProgramCompleted,
    }

    let mut _state = DebuggerState::WaitingProgram;

    let mut debug_info: Option<DebugInfo> = None;
    let mut program: Option<Program> = None;

    let mut options = EidOptions {
        code_preview: true,
        heap_preview: true,
        stack_preview: true,
        registers_preview: true,
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
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset)
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
                    options.code_preview = !options.code_preview;
                    println!(
                        "Code preview is now {}'{}'{}",
                        if options.code_preview {
                            cli_color.color(Colors::Green)
                        } else {
                            cli_color.color(Colors::Red)
                        },
                        if options.code_preview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        cli_color.color(Colors::Reset)
                    );
                }
                EidCommands::HeapPreview => {
                    options.heap_preview = !options.heap_preview;
                    println!(
                        "Heap preview is now {}'{}'{}",
                        if options.heap_preview {
                            cli_color.color(Colors::Green)
                        } else {
                            cli_color.color(Colors::Red)
                        },
                        if options.heap_preview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        cli_color.color(Colors::Reset)
                    );
                }
                EidCommands::StackPreview => {
                    options.stack_preview = !options.stack_preview;
                    println!(
                        "Stack preview is now {}'{}'{}",
                        if options.stack_preview {
                            cli_color.color(Colors::Green)
                        } else {
                            cli_color.color(Colors::Red)
                        },
                        if options.stack_preview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        cli_color.color(Colors::Reset)
                    );
                }
                EidCommands::RegistersPreview => {
                    options.registers_preview = !options.registers_preview;
                    println!(
                        "Registers preview is now {}'{}'{}",
                        if options.registers_preview {
                            cli_color.color(Colors::Green)
                        } else {
                            cli_color.color(Colors::Red)
                        },
                        if options.registers_preview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        cli_color.color(Colors::Reset)
                    );
                }
                EidCommands::Load => {
                    let path = match &e.args[0].value_type {
                        BuildEidArgTypes::String(path) => Path::new(path),
                        _ => {
                            println!(
                                "{}Invalid argument type{}: Expected String",
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset)
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
                                let mut reader = RFile::new(&mut e);
                                match read_program(&mut reader) {
                                    Ok(e) => {
                                        program = Some(e);
                                        _state = DebuggerState::ProgramLoaded;
                                    }
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed to read program, error code: {}{}{}",
                                            cli_color.color(Colors::Red),
                                            cli_color.color(Colors::Reset),
                                            cli_color.color(Colors::Cyan),
                                            e,
                                            cli_color.color(Colors::Reset)
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Failed to read file {}[{}]{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Cyan),
                                    e,
                                    cli_color.color(Colors::Reset)
                                );
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!(
                            "{}Invalid path{}: Not a file",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset)
                        );
                        continue;
                    }
                    println!(
                        "{}Program loaded{}: {}{}{}",
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Cyan),
                        path.to_str().unwrap(),
                        cli_color.color(Colors::Reset)
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
                                                    cli_color.color(Colors::Green),
                                                    cli_color.color(Colors::Reset),
                                                    cli_color.color(Colors::Cyan),
                                                    path.to_str().unwrap(),
                                                    cli_color.color(Colors::Reset)
                                                );
                                            }
                                            Err(e) => {
                                                println!(
                                                    "{}Error:{} {}",
                                                    cli_color.color(Colors::Red),
                                                    cli_color.color(Colors::Reset),
                                                    e
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed to read file {}[{}]{}",
                                            cli_color.color(Colors::Red),
                                            cli_color.color(Colors::Reset),
                                            cli_color.color(Colors::Cyan),
                                            e,
                                            cli_color.color(Colors::Reset)
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
                            cli_color.color(Colors::Green),
                            cli_color.color(Colors::Reset),
                        );
                        println!(
                            "{}- Arch{}: {}",
                            cli_color.color(Colors::Yellow),
                            cli_color.color(Colors::Reset),
                            e.arch
                        );
                        println!(
                            "{}- Debug Info{}: {}{}{}",
                            cli_color.color(Colors::Yellow),
                            cli_color.color(Colors::Reset),
                            match debug_info {
                                Some(_) => cli_color.color(Colors::Green),
                                None => cli_color.color(Colors::Red),
                            },
                            match debug_info {
                                Some(_) => "Available",
                                None => "Not available",
                            },
                            cli_color.color(Colors::Reset)
                        );
                    }
                    None => {
                        println!(
                            "{}Error:{} No program loaded",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset)
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
                            cli_color.color(Colors::Red),
                            _command,
                            cli_color.color(Colors::Reset)
                        );
                    }
                } else if e == 1 {
                    println!(
                        "{}Invalid argument length{}: Check syntax of the command",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                } else if e == 2 {
                    println!(
                        "{}Invalid argument type{}: Check syntax of the command",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                }
            }
        }
    }
}
