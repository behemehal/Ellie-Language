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
