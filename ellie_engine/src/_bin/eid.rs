use ellie_cli_utils::{
    options, outputs,
    utils::{self, Colors},
};
use ellie_engine::vm::parse_debug_file;
use std::{
    collections::HashMap,
    fs::File,
    io::{Read, Write},
    path::Path,
};

pub enum EidArgTypes {
    String,
    Int,
    Bool,
}

pub struct EidArg {
    pub name: &'static str,
    pub value_type: EidArgTypes,
    pub optional: bool,
}

pub struct EidCommand {
    pub short: &'static str,
    pub long: &'static str,
    pub help: &'static str,
    pub args: Vec<EidArg>,
    pub command: EidCommands,
}

#[derive(Clone)]
pub enum BuildEidArgTypes {
    String(String),
    Int(isize),
    Bool(bool),
}

pub fn eq_b(a: &BuildEidArgTypes, b: &EidArgTypes) -> bool {
    match (a, b) {
        (BuildEidArgTypes::String(_), EidArgTypes::String) => true,
        (BuildEidArgTypes::Int(_), EidArgTypes::Int) => true,
        (BuildEidArgTypes::Bool(_), EidArgTypes::Bool) => true,
        _ => false,
    }
}

#[derive(Clone)]
pub struct BuildEidArg {
    pub name: &'static str,
    pub value_type: BuildEidArgTypes,
    pub optional: bool,
}

pub struct BuildEidCommand {
    pub args: Vec<BuildEidArg>,
    pub command: EidCommands,
}

#[derive(Clone)]
pub enum EidCommands {
    // CLI commands
    Help,
    Exit,
    Clear,

    // Information
    CodePreview,
    HeapPreview,
    StackPreview,
    RegistersPreview,

    //Tools
    //HexToDec,
    //DecToHex,
    //HexToString,
    //StringToHex,

    // Program Management
    ProgramInfo,
    Load,
    ResetHeap,
    ResetStack,
    ResetAll,

    // Program Execution
    Run,
    Wait,
    Step,
    StepBack,

    // VM Information
    GetHeapValue,
    GetStackValue,
    GetRegister,
    GetRegisters,
    GetHeap,
    GetCodePos,
    StepChanges,
}

fn parse_args(args: Vec<String>) -> Vec<BuildEidArgTypes> {
    let mut parsed_args = Vec::new();
    for arg in args {
        if arg == "on"
            || arg == "off"
            || arg == "true"
            || arg == "false"
            || arg == "aç"
            || arg == "kapat"
        {
            parsed_args.push(BuildEidArgTypes::Bool(
                arg == "on" || arg == "true" || arg == "aç",
            ));
        } else if let Ok(num) = arg.parse::<isize>() {
            parsed_args.push(BuildEidArgTypes::Int(num));
        } else {
            parsed_args.push(BuildEidArgTypes::String(arg.to_string()));
        }
    }
    parsed_args
}

fn parse_command(input: &String, args: Vec<BuildEidArgTypes>) -> Result<BuildEidCommand, u8> {
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

    match commands
        .iter()
        .find(|command| command.short == input || command.long == input)
    {
        Some(e) => {
            let min_arg_len = e.args.iter().filter(|arg| !arg.optional).count();
            let max_arg_len = e.args.len();
            if args.len() < min_arg_len || args.len() > max_arg_len {
                return Err(1);
            }

            let mut processed_args = vec![];
            for (idx, arg) in args.iter().enumerate() {
                if eq_b(&arg, &e.args[idx].value_type) {
                    processed_args.push(BuildEidArg {
                        name: e.args[idx].name.clone(),
                        value_type: arg.clone(),
                        optional: e.args[idx].optional,
                    });
                } else {
                    return Err(2);
                }
            }
            Ok(BuildEidCommand {
                command: e.command.clone(),
                args: processed_args,
            })
        }
        None => Err(0),
    }
}

fn main() {
    println!(
        "{}EllieVM - Interactive Debugger{}: {}Type 'help' for help{}\n",
        utils::Colors::Green,
        utils::Colors::Reset,
        utils::Colors::Yellow,
        utils::Colors::Reset
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
                                    utils::Colors::Red,
                                    utils::Colors::Reset
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
                            utils::Colors::Green
                        } else {
                            utils::Colors::Red
                        },
                        if options.CodePreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        utils::Colors::Reset
                    );
                }
                EidCommands::HeapPreview => {
                    options.HeapPreview = !options.HeapPreview;
                    println!(
                        "Heap preview is now {}'{}'{}",
                        if options.HeapPreview {
                            utils::Colors::Green
                        } else {
                            utils::Colors::Red
                        },
                        if options.HeapPreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        utils::Colors::Reset
                    );
                }
                EidCommands::StackPreview => {
                    options.StackPreview = !options.StackPreview;
                    println!(
                        "Stack preview is now {}'{}'{}",
                        if options.StackPreview {
                            utils::Colors::Green
                        } else {
                            utils::Colors::Red
                        },
                        if options.StackPreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        utils::Colors::Reset
                    );
                }
                EidCommands::RegistersPreview => {
                    options.RegistersPreview = !options.RegistersPreview;
                    println!(
                        "Registers preview is now {}'{}'{}",
                        if options.RegistersPreview {
                            utils::Colors::Green
                        } else {
                            utils::Colors::Red
                        },
                        if options.RegistersPreview {
                            "enabled"
                        } else {
                            "disabled"
                        },
                        utils::Colors::Reset
                    );
                }
                EidCommands::Load => {
                    let path = match &e.args[0].value_type {
                        BuildEidArgTypes::String(path) => Path::new(path),
                        _ => {
                            println!(
                                "{}Invalid argument type{}: Expected String",
                                utils::Colors::Red,
                                utils::Colors::Reset
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
                                            utils::Colors::Red,
                                            utils::Colors::Reset,
                                            utils::Colors::Cyan,
                                            e,
                                            utils::Colors::Reset
                                        );
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Failed to read file {}[{}]{}",
                                    utils::Colors::Red,
                                    utils::Colors::Reset,
                                    utils::Colors::Cyan,
                                    e,
                                    utils::Colors::Reset
                                );
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!(
                            "{}Invalid path{}: Not a file",
                            utils::Colors::Red,
                            utils::Colors::Reset
                        );
                        continue;
                    }
                    println!(
                        "{}Program loaded{}: {}{}{}",
                        utils::Colors::Green,
                        utils::Colors::Reset,
                        utils::Colors::Cyan,
                        path.to_str().unwrap(),
                        utils::Colors::Reset
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
                                                    utils::Colors::Green,
                                                    utils::Colors::Reset,
                                                    utils::Colors::Cyan,
                                                    path.to_str().unwrap(),
                                                    utils::Colors::Reset
                                                );
                                            }
                                            Err(e) => {
                                                println!(
                                                    "{}Error:{} {}",
                                                    utils::Colors::Red,
                                                    utils::Colors::Reset,
                                                    e
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed to read file {}[{}]{}",
                                            utils::Colors::Red,
                                            utils::Colors::Reset,
                                            utils::Colors::Cyan,
                                            e,
                                            utils::Colors::Reset
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
                            utils::Colors::Green,
                            utils::Colors::Reset,
                        );
                        println!(
                            "{}- Arch{}: {}",
                            utils::Colors::Yellow,
                            utils::Colors::Reset,
                            e.arch
                        );
                        println!(
                            "{}- Debug Info{}: {}{}{}",
                            utils::Colors::Yellow,
                            utils::Colors::Reset,
                            match debug_info {
                                Some(_) => utils::Colors::Green,
                                None => utils::Colors::Red,
                            },
                            match debug_info {
                                Some(_) => "Available",
                                None => "Not available",
                            },
                            utils::Colors::Reset
                        );
                    }
                    None => {
                        println!(
                            "{}Error:{} No program loaded",
                            utils::Colors::Red,
                            utils::Colors::Reset
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
                            utils::Colors::Red,
                            _command,
                            utils::Colors::Reset
                        );
                    }
                } else if e == 1 {
                    println!(
                        "{}Invalid argument length{}: Check syntax of the command",
                        utils::Colors::Red,
                        utils::Colors::Reset
                    );
                } else if e == 2 {
                    println!(
                        "{}Invalid argument type{}: Check syntax of the command",
                        utils::Colors::Red,
                        utils::Colors::Reset
                    );
                }
            }
        }
    }
}
