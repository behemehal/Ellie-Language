use std::fmt::Display;

pub enum DebuggerArgTypes {
    String,
    Int,
    Bool,
}

impl Display for DebuggerArgTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DebuggerArgTypes::String => write!(f, "String"),
            DebuggerArgTypes::Int => write!(f, "Int"),
            DebuggerArgTypes::Bool => write!(f, "Bool"),
        }
    }
}

pub struct DebuggerArg {
    pub name: &'static str,
    pub value_type: DebuggerArgTypes,
    pub optional: bool,
}

pub struct DebuggerCommand {
    pub short: &'static str,
    pub long: &'static str,
    pub help: &'static str,
    pub args: Vec<DebuggerArg>,
    pub command: DebuggerCommands,
    pub has_json_output: bool,
}

#[derive(Clone, Debug)]
pub enum BuildDebuggerArgTypes {
    String(String),
    Int(isize),
    Bool(bool),
}

pub fn eq_b(a: &BuildDebuggerArgTypes, b: &DebuggerArgTypes) -> bool {
    match (a, b) {
        (BuildDebuggerArgTypes::String(_), DebuggerArgTypes::String) => true,
        (BuildDebuggerArgTypes::Int(_), DebuggerArgTypes::Int) => true,
        (BuildDebuggerArgTypes::Bool(_), DebuggerArgTypes::Bool) => true,
        _ => false,
    }
}

#[derive(Clone, Debug)]
pub struct BuildDebuggerArg {
    pub name: &'static str,
    pub value_type: BuildDebuggerArgTypes,
    pub optional: bool,
}

#[derive(Clone, Debug)]
pub struct BuildDebuggerCommand {
    pub args: Vec<BuildDebuggerArg>,
    pub command: DebuggerCommands,
}

#[derive(Clone, Debug)]
pub enum DebuggerCommands {
    // CLI commands
    Exit,
    Help,
    Clear,

    // Program Management
    Load,
    ReloadVm,

    // Program Execution
    Run,
    Wait,
    Step,

    // Information
    ReadAtPosition,
    GetPaths,
    GetBreakpoints,
    GetRegisters,
    GetStackMemory,
    GetHeapMemory,
    // GetCodePos,
    // StepChanges,
}

pub fn parse_args(args: Vec<&str>) -> Vec<BuildDebuggerArgTypes> {
    let mut parsed_args = Vec::new();
    for arg in args {
        if arg == "on"
            || arg == "off"
            || arg == "true"
            || arg == "false"
            || arg == "aç"
            || arg == "kapat"
        {
            parsed_args.push(BuildDebuggerArgTypes::Bool(
                arg == "on" || arg == "true" || arg == "aç",
            ));
        } else if let Ok(num) = arg.parse::<isize>() {
            parsed_args.push(BuildDebuggerArgTypes::Int(num));
        } else {
            parsed_args.push(BuildDebuggerArgTypes::String(arg.to_string()));
        }
    }
    parsed_args
}

pub fn parse_command(
    input: &str,
    args: Vec<BuildDebuggerArgTypes>,
    json_output: bool,
) -> Result<BuildDebuggerCommand, u8> {
    match COMMANDS
        .iter()
        .find(|command| command.short == input || command.long == input)
    {
        Some(e) => {
            if !e.has_json_output && json_output {
                return Err(3);
            }

            let min_arg_len = e.args.iter().filter(|arg| !arg.optional).count();
            let max_arg_len = e.args.len();
            if args.len() < min_arg_len || args.len() > max_arg_len {
                return Err(1);
            }

            let mut processed_args = vec![];
            for (idx, arg) in args.iter().enumerate() {
                if eq_b(&arg, &e.args[idx].value_type) {
                    processed_args.push(BuildDebuggerArg {
                        name: e.args[idx].name.clone(),
                        value_type: arg.clone(),
                        optional: e.args[idx].optional,
                    });
                } else {
                    return Err(2);
                }
            }
            Ok(BuildDebuggerCommand {
                command: e.command.clone(),
                args: processed_args,
            })
        }
        None => Err(0),
    }
}

lazy_static! {
    pub static ref COMMANDS: [DebuggerCommand; 14] = [
        DebuggerCommand {
            short: "e",
            long: "exit",
            help: "Exit the program",
            command: DebuggerCommands::Exit,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "h",
            long: "help",
            help: "Show help message or target a command for getting help",
            command: DebuggerCommands::Help,
            args: vec![DebuggerArg {
                name: "command",
                value_type: DebuggerArgTypes::String,
                optional: true,
            }],
            has_json_output: false,
        },
        DebuggerCommand {
            short: "c",
            long: "clear",
            help: "Clear the screen",
            command: DebuggerCommands::Clear,
            args: vec![],
            has_json_output: false,
        },
        DebuggerCommand {
            short: "l",
            long: "load",
            help: "Load a program",
            command: DebuggerCommands::Load,
            args: vec![
                DebuggerArg {
                    name: "file",
                    value_type: DebuggerArgTypes::String,
                    optional: false,
                },
                DebuggerArg {
                    name: "debug_file",
                    value_type: DebuggerArgTypes::String,
                    optional: false,
                }
            ],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "rv",
            long: "reload-vm",
            help: "Reset vm to initial state with program loaded",
            command: DebuggerCommands::ReloadVm,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "r",
            long: "run",
            help: "Run the program",
            command: DebuggerCommands::Run,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "w",
            long: "wait",
            help: "Wait program at given stack position",
            command: DebuggerCommands::Wait,
            args: vec![
                DebuggerArg {
                    name: "use_stack_pos",
                    value_type: DebuggerArgTypes::Bool,
                    optional: false,
                },
                DebuggerArg {
                    name: "pos",
                    value_type: DebuggerArgTypes::Int,
                    optional: false,
                },
                DebuggerArg {
                    name: "module_path",
                    value_type: DebuggerArgTypes::String,
                    optional: false,
                },
            ],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "s",
            long: "step",
            help: "Run the program one step",
            command: DebuggerCommands::Step,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "rap",
            long: "read-at-position",
            help: "Read the variable at given ",
            command: DebuggerCommands::ReadAtPosition,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "gp",
            long: "get-paths",
            help: "Get the paths of all the modules",
            command: DebuggerCommands::GetPaths,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "gbp",
            long: "get-breakpoints",
            help: "Get list of the breakpoints",
            command: DebuggerCommands::GetBreakpoints,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "gr",
            long: "get-registers",
            help: "Get list of the registers",
            command: DebuggerCommands::GetRegisters,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "gs",
            long: "get-stack-memory",
            help: "Get list of the stack memory",
            command: DebuggerCommands::GetStackMemory,
            args: vec![],
            has_json_output: true,
        },
        DebuggerCommand {
            short: "gh",
            long: "get-heap-memory",
            help: "Get list of the heap memory",
            command: DebuggerCommands::GetHeapMemory,
            args: vec![],
            has_json_output: true,
        },
    ];
}
