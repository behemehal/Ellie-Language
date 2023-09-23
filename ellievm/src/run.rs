use ellie_engine::{
    ellie_core::defs::{DebugHeader, DebugInfo, PlatformArchitecture},
    ellie_renderer_utils::utils::{CliColor, ColorDisplay, Colors},
    ellie_vm::{
        channel::{ModuleManager, EllieModule},
        program::{Program, VmProgram},
        thread::{Isolate, Thread},
        utils::ThreadExit,
    },
};

pub struct VmSettings {
    pub json_log: bool,
    pub warnings: bool,
    pub heap_dump: bool,
    pub architecture: PlatformArchitecture,
    pub modules: Vec<EllieModule>,
}

pub fn run(program: Program, vm_settings: VmSettings, debug_file: Option<DebugInfo>) {
    let mut vm_program = VmProgram::new();
    vm_program.fill_from_vector(program.instructions);
    vm_program.fill_traces(program.native_call_traces);
    let mut module_manager = ModuleManager::new();

    //Register incoming modules
    for module in vm_settings.modules {
        module_manager.register_module(module);
    }


    let cli_color = &CliColor;

    let isolate = Isolate::new();
    let mut thread = Thread::new(program.main.hash, PlatformArchitecture::B64, isolate);
    thread.build_thread(program.main.clone());
    let output = thread.run(&mut module_manager, &vm_program);
    match output {
        ThreadExit::ExitGracefully => {
            if vm_settings.heap_dump {
                println!(
                    "{}[VM]{}: Heap Dump\n\n{}",
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset),
                    thread.isolate.heap_dump(),
                );
                println!(
                    "{}[VM]{}: Stack Dump\n\n{}",
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset),
                    thread.isolate.stack_dump(),
                );
            }
        }
        ThreadExit::Panic(panic) => {
            println!(
                "\n{}ThreadPanic{} : {}{:?}{}",
                cli_color.color(Colors::Red),
                cli_color.color(Colors::Reset),
                cli_color.color(Colors::Cyan),
                panic.reason,
                cli_color.color(Colors::Reset),
            );
            for frame in panic.stack_trace {
                match &debug_file {
                    Some(debug_file) => {
                        let coresponding_header = debug_file
                            .debug_headers
                            .iter()
                            .find(|x| frame.pos >= x.start_end.0 && frame.pos <= x.start_end.1);

                        match coresponding_header {
                            Some(e) => {
                                fn get_real_path(
                                    debug_header: &DebugHeader,
                                    debug_file: &DebugInfo,
                                ) -> String {
                                    let module_name = debug_header
                                        .module_name
                                        .split("<ellie_module_")
                                        .nth(1)
                                        .unwrap()
                                        .split(">")
                                        .nth(0)
                                        .unwrap();
                                    let module_path = debug_file
                                        .module_map
                                        .iter()
                                        .find(|map| module_name == map.module_name);
                                    let real_path = match module_path {
                                        Some(module_path) => match &module_path.module_path {
                                            Some(module_path) => {
                                                let new_path = debug_header.module_name.clone();
                                                let starter_name =
                                                    format!("<ellie_module_{}>", module_name);
                                                new_path.replace(&starter_name, &module_path)
                                            }
                                            None => debug_header.module_name.clone(),
                                        },
                                        None => debug_header.module_name.clone(),
                                    };
                                    real_path
                                }

                                let real_path = get_real_path(e, debug_file);

                                println!(
                                    "{}    at {}:{}:{}",
                                    cli_color.color(Colors::Green),
                                    real_path,
                                    e.pos.range_start.0 + 1,
                                    e.pos.range_start.1 + 1,
                                );
                            }
                            None => {
                                println!(
                                    "{}    at {}:{}",
                                    cli_color.color(Colors::Green),
                                    "frame.name",
                                    frame.pos
                                );
                            }
                        }
                    }
                    None => {
                        println!(
                            "{}    at {}:{} ({} + {})",
                            cli_color.color(Colors::Green),
                            "frame.name",
                            frame.pos + frame.frame_pos,
                            frame.pos,
                            frame.frame_pos,
                        );
                    }
                }
            }
            if debug_file.is_none() {
                println!(
                    "\n{}NoDebugFile{} : {}Given error represents stack locations, provide a debug info file to get more readable info{}",
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Cyan),
                    cli_color.color(Colors::Reset),
                );
            }
            println!(
                "{}    at {}",
                cli_color.color(Colors::Red),
                panic.code_location,
            );
            if vm_settings.heap_dump {
                println!(
                    "{}[VM]{}: Heap Dump\n\n{}",
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset),
                    thread.isolate.heap_dump(),
                );
                println!(
                    "{}[VM]{}: Stack Dump\n\n{}",
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset),
                    thread.isolate.stack_dump(),
                );
            }
            std::process::exit(1);
        }
    }
}
