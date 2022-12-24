pub mod vm;
use crate::vm::{parse_debug_file, read_program, RFile};
use ellie_engine::{
    ellie_core::{
        defs::{DebugHeader, DebugInfo, PlatformArchitecture, VmNativeAnswer},
        raw_type,
    },
    ellie_renderer_utils::{
        options, outputs,
        utils::{self, CliColor, ColorDisplay, Colors, TextStyles},
    },
    ellie_vm::{
        heap::{self, Heap},
        thread::Stack,
        utils::{Instructions, ThreadExit, ThreadStep, ThreadStepInfo},
        vm::VM,
    },
    engine_constants,
};

use std::{
    fs::{self, File},
    io::{Read, Write},
    path::Path,
    thread,
};

pub struct VmSettings {
    pub json_log: bool,
    pub warnings: bool,
    pub heap_dump: bool,
    pub architecture: PlatformArchitecture,
}

fn main() {
    let app = options::generate_ellievm_options();
    let matches = app.get_matches();
    let version = format!("0.1.0",);
    let cli_color = &CliColor;

    match matches.subcommand() {
        Some(("run", matches)) => {
            let is_vm_debug = matches.is_present("vmDebug");
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}VM halted{}\n",
                            cli_color.color(Colors::Yellow),
                            cli_color.text_style(TextStyles::Bold),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}{}",
                            cli_color.color(Colors::Blue),
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split("@")
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_color.color(Colors::Red)
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_color.color(Colors::Red),
                        cli_color.text_style(TextStyles::Bold),
                        cli_color.color(Colors::Red)
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Yellow),
                        cli_color.color(Colors::Reset),
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=vm,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20ellievm%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_color.text_style(TextStyles::Underline),cli_color.color(Colors::Green),line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, cli_color.color(Colors::Reset));
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }));
            }

            let vm_settings = VmSettings {
                json_log: matches.is_present("jsonLog"),
                warnings: true,
                heap_dump: matches.is_present("heapDump"),
                architecture: match matches.value_of("targetArchitecture") {
                    Some(e) => {
                        if e == "64" {
                            PlatformArchitecture::B64
                        } else if e == "32" {
                            PlatformArchitecture::B32
                        } else if e == "16" {
                            PlatformArchitecture::B16
                        } else {
                            println!(
                                "{}Error:{} Unknown architecture '{}{}{}'",
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Yellow),
                                e,
                                cli_color.color(Colors::Reset),
                            );
                            std::process::exit(1);
                        }
                    }
                    None => unreachable!(),
                },
            };

            let debug_file = match matches.value_of("debugInfo") {
                Some(e) => {
                    let path = Path::new(e);
                    if path.is_file() {
                        let mut file_contents = String::new();
                        match File::open(e) {
                            Ok(mut e) => {
                                e.read_to_string(&mut file_contents).unwrap();
                                match parse_debug_file(file_contents) {
                                    Ok(e) => Some(e),
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
                    } else {
                        println!(
                            "{}Error:{} Given path is not a file",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset)
                        );
                        std::process::exit(1);
                    }
                }
                None => None,
            };

            let path = Path::new(matches.value_of("target").unwrap().clone());
            let program = if path.exists() {
                if path.is_file() {
                    match File::open(path) {
                        Ok(mut e) => {
                            let mut reader = RFile::new(&mut e);
                            match read_program(&mut reader) {
                                Ok(e) => e,
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
                        "{}Error:{} Given path is not a file",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }
            } else {
                println!(
                    "{}Error:{} Target path does not exist",
                    cli_color.color(Colors::Red),
                    cli_color.color(Colors::Reset)
                );
                std::process::exit(1);
            };

            let mut step_into = false;
            let mut show_heap_dump = false;
            let mut show_registers = true;
            let mut wait_pos = None;
            let mut show_code = false;
            let mut show_stack_info = false;
            let mut last_step: Option<ThreadStep> = None;

            let mut vm = VM::new(vm_settings.architecture, |_, e| {
                if e.module == "ellieStd" && e.name == "heapDump" {
                    println!(
                        "{}HeapDump{}: {}Queued: heapDump.txt{}",
                        cli_color.color(Colors::Green),
                        cli_color.color(Colors::Reset),
                        cli_color.color(Colors::Cyan),
                        cli_color.color(Colors::Reset)
                    );
                    VmNativeAnswer::Ok(().into())
                } else if e.module == "ellieStd" && e.name == "println" {
                    let string = String::from_utf8(e.params[0].data.clone());
                    println!("println: {:?}", string.unwrap());
                    VmNativeAnswer::Ok(().into())
                } else if e.module == "main" && e.name == "get_val_n" {
                    VmNativeAnswer::Ok("Cevaaap".into())
                } else {
                    VmNativeAnswer::RuntimeError("Call to unknown function".into())
                }
            });
            vm.load(&program).unwrap();

            //let (tx, rx) = mpsc::channel();

            vm.build_main_thread(program.main);

            if is_vm_debug {
                println!(
                    "{}EllieVM - Interactive Debugger{}: {}Type 'help' for help{}\n",
                    cli_color.color(Colors::Green),
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset)
                );
                println!(
                    "{}VM{}: {}Program loaded {}\n",
                    cli_color.color(Colors::Green),
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Yellow),
                    cli_color.color(Colors::Reset)
                );
            }

            let main_thread = thread::spawn(move || loop {
                if is_vm_debug {
                    //clear console
                    fn step(
                        heap: &mut Heap,
                        stack: Stack,
                        debug_file: Option<DebugInfo>,
                        thread_step: ThreadStep,
                        show_heap_dump: bool,
                        show_registers: bool,
                        wait_pos: Option<usize>,
                        show_code: bool,
                        show_stack_info: bool,
                    ) {
                        let cli_color = &CliColor;
                        if show_stack_info {
                            println!(
                                "{}Stack info:{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset)
                            );
                            println!(
                                "- {}ID{}: {}",
                                cli_color.color(Colors::Yellow),
                                cli_color.color(Colors::Reset),
                                stack.id
                            );
                            println!(
                                "- {}NAME{}: {}",
                                cli_color.color(Colors::Yellow),
                                cli_color.color(Colors::Reset),
                                stack.name
                            );
                            println!(
                                "- {}CALLER{}: {:?}",
                                cli_color.color(Colors::Yellow),
                                cli_color.color(Colors::Reset),
                                stack.caller
                            );
                        } else {
                            println!(
                                "{}StackInfo{}: {}Disabled{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset)
                            );
                        }
                        println!(
                            "{}Instruction{}:",
                            cli_color.color(Colors::Green),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}{}: {}{:?}{}",
                            cli_color.color(Colors::Yellow),
                            thread_step.stack_pos,
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Cyan),
                            match thread_step.instruction.instruction {
                                Instructions::LDA(_) =>
                                    format!("LDA {:?}", thread_step.instruction.addressing_value),
                                Instructions::LDB(_) =>
                                    format!("LDB {:?}", thread_step.instruction.addressing_value),
                                Instructions::LDC(_) =>
                                    format!("LDC {:?}", thread_step.instruction.addressing_value),
                                Instructions::LDX(_) =>
                                    format!("LDX {:?}", thread_step.instruction.addressing_value),
                                Instructions::LDY(_) =>
                                    format!("LDY {:?}", thread_step.instruction.addressing_value),
                                Instructions::STA(_) =>
                                    format!("STA {:?}", thread_step.instruction.addressing_value),
                                Instructions::STB(_) =>
                                    format!("STB {:?}", thread_step.instruction.addressing_value),
                                Instructions::STC(_) =>
                                    format!("STC {:?}", thread_step.instruction.addressing_value),
                                Instructions::STX(_) =>
                                    format!("STX {:?}", thread_step.instruction.addressing_value),
                                Instructions::STY(_) =>
                                    format!("STY {:?}", thread_step.instruction.addressing_value),
                                Instructions::EQ(_) =>
                                    format!("EQ {:?}", thread_step.instruction.addressing_value),
                                Instructions::NE(_) =>
                                    format!("NE {:?}", thread_step.instruction.addressing_value),
                                Instructions::GT(_) =>
                                    format!("GT {:?}", thread_step.instruction.addressing_value),
                                Instructions::LT(_) =>
                                    format!("LT {:?}", thread_step.instruction.addressing_value),
                                Instructions::GQ(_) =>
                                    format!("GQ {:?}", thread_step.instruction.addressing_value),
                                Instructions::LQ(_) =>
                                    format!("LQ {:?}", thread_step.instruction.addressing_value),
                                Instructions::AND(_) =>
                                    format!("AND {:?}", thread_step.instruction.addressing_value),
                                Instructions::OR(_) =>
                                    format!("OR {:?}", thread_step.instruction.addressing_value),
                                Instructions::ADD(_) =>
                                    format!("ADD {:?}", thread_step.instruction.addressing_value),
                                Instructions::SUB(_) =>
                                    format!("SUB {:?}", thread_step.instruction.addressing_value),
                                Instructions::MUL(_) =>
                                    format!("MUL {:?}", thread_step.instruction.addressing_value),
                                Instructions::EXP(_) =>
                                    format!("EXP {:?}", thread_step.instruction.addressing_value),
                                Instructions::DIV(_) =>
                                    format!("DIV {:?}", thread_step.instruction.addressing_value),
                                Instructions::MOD(_) =>
                                    format!("MOD {:?}", thread_step.instruction.addressing_value),
                                Instructions::INC(_) =>
                                    format!("INC {:?}", thread_step.instruction.addressing_value),
                                Instructions::DEC(_) =>
                                    format!("DEC {:?}", thread_step.instruction.addressing_value),
                                Instructions::JMP(_) =>
                                    format!("JMP {:?}", thread_step.instruction.addressing_value),
                                Instructions::CALL(_) =>
                                    format!("CALL {:?}", thread_step.instruction.addressing_value),
                                Instructions::RET(_) =>
                                    format!("RET {:?}", thread_step.instruction.addressing_value),
                                Instructions::UGR(_) =>
                                    format!("UGR {:?}", thread_step.instruction.addressing_value),
                                Instructions::ULR(_) =>
                                    format!("ULR {:?}", thread_step.instruction.addressing_value),
                                Instructions::PUSH(_) =>
                                    format!("PUSH {:?}", thread_step.instruction.addressing_value),
                                Instructions::LEN(_) =>
                                    format!("LEN {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2I(_) =>
                                    format!("A2I {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2F(_) =>
                                    format!("A2F {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2D(_) =>
                                    format!("A2D {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2B(_) =>
                                    format!("A2B {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2S(_) =>
                                    format!("A2S {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2C(_) =>
                                    format!("A2C {:?}", thread_step.instruction.addressing_value),
                                Instructions::A2O(_) =>
                                    format!("A2O {:?}", thread_step.instruction.addressing_value),
                                Instructions::JMPA(_) =>
                                    format!("JMPA {:?}", thread_step.instruction.addressing_value),
                                Instructions::POPS(_) =>
                                    format!("POPS {:?}", thread_step.instruction.addressing_value),
                                Instructions::ACP(_) =>
                                    format!("ACP {:?}", thread_step.instruction.addressing_value),
                                Instructions::BRK(_) =>
                                    format!("BRK {:?}", thread_step.instruction.addressing_value),
                                Instructions::CALLN(_) =>
                                    format!("CALLN {:?}", thread_step.instruction.addressing_value),
                                Instructions::CO(_) =>
                                    format!("CO {:?}", thread_step.instruction.addressing_value),
                                Instructions::FN(_) =>
                                    format!("FN {:?}", thread_step.instruction.addressing_value),
                                Instructions::CALLC(_) => todo!(),
                            },
                            cli_color.color(Colors::Reset)
                        );

                        if show_code {
                            if let Some(debug_file) = &debug_file {
                                println!(
                                    "{}ShowCode:{} {}Line{}",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset)
                                );
                                let coresponding_header =
                                    debug_file.debug_headers.iter().find(|x| {
                                        thread_step.stack_pos >= x.start_end.0
                                            && thread_step.stack_pos <= x.start_end.1
                                    });
                                if let Some(e) = &coresponding_header {
                                    fn get_real_path(
                                        debug_header: &DebugHeader,
                                        debug_file: &DebugInfo,
                                    ) -> String {
                                        let module_name = debug_header
                                            .module
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
                                                    let new_path = debug_header.module.clone();
                                                    let starter_name =
                                                        format!("<ellie_module_{}>", module_name);
                                                    new_path.replace(&starter_name, &module_path)
                                                }
                                                None => debug_header.module.clone(),
                                            },
                                            None => debug_header.module.clone(),
                                        };
                                        real_path
                                    }

                                    let real_path = get_real_path(e, debug_file);

                                    println!(
                                        "{}[~]{}  ╞ {}{real_path}{}:{}:{}",
                                        cli_color.color(Colors::Red),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Green),
                                        cli_color.color(Colors::Reset),
                                        e.pos.range_start.0 + 1,
                                        e.pos.range_start.1 + 1,
                                    );
                                    let target_file = fs::read_to_string(real_path).unwrap();
                                    let target_file = target_file.lines().collect::<Vec<&str>>();
                                    let target_line = target_file[e.pos.range_start.0].to_string();
                                    println!(
                                        "{}{}{}  │ {}{}{}",
                                        cli_color.color(Colors::Yellow),
                                        e.pos.range_start.0 + 1,
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Green),
                                        target_line,
                                        cli_color.color(Colors::Reset),
                                    );
                                }
                            } else {
                                println!(
                                    "{}ShowCode:{}: {}Debug file not found!{}",
                                    cli_color.color(Colors::Green),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                );
                            }
                        } else {
                            println!(
                                "{}ShowCode:{} {}Disabled{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset)
                            );
                        }

                        if show_heap_dump {
                            println!(
                                "{}HeapDump:{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset)
                            );
                            println!("{}\n", heap.dump());
                        } else {
                            println!(
                                "{}HeapDump:{} {}Disabled{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset),
                            );
                        }

                        fn render_raw_type(raw_type: &raw_type::RawType) -> String {
                            match raw_type.type_id.id {
                                1 => return format!("Int({})", raw_type.to_int()),
                                2 => return format!("Float({})", raw_type.to_float()),
                                3 => return format!("Double({})", raw_type.to_double()),
                                4 => return format!("Byte({})", raw_type.to_byte()),
                                5 => return format!("Bool({})", raw_type.to_bool()),
                                6 => return format!("String({})", raw_type.to_string()),
                                7 => return format!("Char({})", raw_type.to_char()),
                                8 => return format!("Void"),
                                9 => return format!("Array"),
                                10 => return format!("Void"),
                                _ => return format!("Unknown"),
                            }
                        }

                        if show_registers {
                            println!(
                                "{}Registers:{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset)
                            );
                            println!(
                                "A: {:?}: {}",
                                stack.registers.A,
                                render_raw_type(&stack.registers.A)
                            );
                            println!(
                                "B: {:?}: {}",
                                stack.registers.B,
                                render_raw_type(&stack.registers.B)
                            );
                            println!(
                                "C: {:?}: {}",
                                stack.registers.C,
                                render_raw_type(&stack.registers.C)
                            );
                            println!(
                                "X: {:?}: {}",
                                stack.registers.X,
                                render_raw_type(&stack.registers.X)
                            );
                            println!(
                                "Y: {:?}: {}",
                                stack.registers.Y,
                                render_raw_type(&stack.registers.Y)
                            );
                        } else {
                            println!(
                                "{}Registers:{} {}Disabled{}",
                                cli_color.color(Colors::Green),
                                cli_color.color(Colors::Reset),
                                cli_color.color(Colors::Red),
                                cli_color.color(Colors::Reset),
                            );
                        }
                    }

                    if step_into {
                        step_into = false;
                        match vm.threads[0].step(&mut vm.heap) {
                            Ok(thread_step) => {
                                last_step = Some(thread_step.clone());
                                if let Some(pos) = wait_pos {
                                    if thread_step.stack_pos.clone() == pos {
                                        std::io::stdout()
                                            .write_all("\x1B[2J\x1B[1;1H".as_bytes())
                                            .unwrap();
                                        println!(
                                            "{}WaitPos{}: {}BreakPoint Reached{}\n",
                                            cli_color.color(Colors::Green),
                                            cli_color.color(Colors::Reset),
                                            cli_color.color(Colors::Yellow),
                                            cli_color.color(Colors::Reset)
                                        );
                                        wait_pos = None;
                                        step(
                                            &mut vm.heap,
                                            vm.threads[0]
                                                .stack
                                                .get(thread_step.stack_id)
                                                .unwrap()
                                                .clone(),
                                            debug_file.clone(),
                                            thread_step.clone(),
                                            show_heap_dump,
                                            show_registers,
                                            wait_pos,
                                            show_code,
                                            show_stack_info,
                                        );
                                    } else {
                                        step_into = true;
                                    }
                                } else {
                                    step(
                                        &mut vm.heap,
                                        vm.threads[0]
                                            .stack
                                            .get(thread_step.stack_id)
                                            .unwrap()
                                            .clone(),
                                        debug_file.clone(),
                                        thread_step.clone(),
                                        show_heap_dump,
                                        show_registers,
                                        wait_pos,
                                        show_code,
                                        show_stack_info,
                                    );
                                }
                            }
                            Err(e) => match e {
                                ThreadExit::Panic(e) => {
                                    println!(
                                        "\n{}ThreadPanic{} : {}{:?}{}",
                                        cli_color.color(Colors::Red),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Cyan),
                                        e.reason,
                                        cli_color.color(Colors::Reset),
                                    );

                                    for frame in e.stack_trace {
                                        match &debug_file {
                                            Some(debug_file) => {
                                                let coresponding_header =
                                                    debug_file.debug_headers.iter().find(|x| {
                                                        frame.stack_pos >= x.start_end.0
                                                            && frame.stack_pos <= x.start_end.1
                                                    });

                                                match coresponding_header {
                                                    Some(e) => {
                                                        fn get_real_path(
                                                            debug_header: &DebugHeader,
                                                            debug_file: &DebugInfo,
                                                        ) -> String
                                                        {
                                                            let module_name = debug_header
                                                                .module
                                                                .split("<ellie_module_")
                                                                .nth(1)
                                                                .unwrap()
                                                                .split(">")
                                                                .nth(0)
                                                                .unwrap();
                                                            let module_path = debug_file
                                                                .module_map
                                                                .iter()
                                                                .find(|map| {
                                                                    module_name == map.module_name
                                                                });
                                                            let real_path = match module_path {
                                                                Some(module_path) => {
                                                                    match &module_path.module_path {
                                                                        Some(module_path) => {
                                                                            let new_path =
                                                                                debug_header
                                                                                    .module
                                                                                    .clone();
                                                                            let starter_name = format!(
                                                                                "<ellie_module_{}>",
                                                                                module_name
                                                                            );
                                                                            new_path.replace(
                                                                                &starter_name,
                                                                                &module_path,
                                                                            )
                                                                        }
                                                                        None => debug_header
                                                                            .module
                                                                            .clone(),
                                                                    }
                                                                }
                                                                None => debug_header.module.clone(),
                                                            };
                                                            real_path
                                                        }

                                                        let real_path =
                                                            get_real_path(e, debug_file);

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
                                                            frame.name,
                                                            frame.stack_pos
                                                        );
                                                    }
                                                }
                                            }
                                            None => {
                                                println!(
                                                    "{}    at {}:{} ({} + {})",
                                                    cli_color.color(Colors::Green),
                                                    frame.name,
                                                    frame.stack_pos + frame.frame_pos,
                                                    frame.stack_pos,
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
                                        e.code_location,
                                    );
                                    if vm_settings.heap_dump {
                                        println!(
                                            "{}[VM]{}: Heap Dump\n\n{}",
                                            cli_color.color(Colors::Yellow),
                                            cli_color.color(Colors::Reset),
                                            vm.heap_dump()
                                        );
                                    }
                                    println!(
                                        "{}ThreadPanic{}: {}Program Halted{}\n",
                                        cli_color.color(Colors::Red),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Yellow),
                                        cli_color.color(Colors::Reset)
                                    );
                                }
                                ThreadExit::ExitGracefully => {
                                    if vm_settings.heap_dump {
                                        println!(
                                            "{}[VM]{}: Heap Dump\n\n{}",
                                            cli_color.color(Colors::Yellow),
                                            cli_color.color(Colors::Reset),
                                            vm.heap_dump()
                                        );
                                    }
                                    println!(
                                        "{}ExitGracefully{}: {}ProgramEnded{}\n",
                                        cli_color.color(Colors::Green),
                                        cli_color.color(Colors::Reset),
                                        cli_color.color(Colors::Yellow),
                                        cli_color.color(Colors::Reset)
                                    );
                                }
                            },
                        }
                    } else {
                        std::io::stdout().write_all("> ".as_bytes()).unwrap();
                        std::io::stdout().flush().unwrap();
                        //clear console
                        let mut input = String::new();
                        std::io::stdin().read_line(&mut input).unwrap();

                        if input.trim() == "exit" {
                            println!("Bye...");
                            break;
                        } else if input.trim() == "clear" {
                            std::io::stdout()
                                .write_all("\x1B[2J\x1B[1;1H".as_bytes())
                                .unwrap();
                        } else if input.trim() == "heap off" {
                            show_heap_dump = false;
                            println!("! HeapDump disabled");
                        } else if input.trim() == "heap on" {
                            show_heap_dump = true;
                            println!("! HeapDump enabled");
                        } else if input.trim() == "reg off" {
                            show_registers = false;
                            println!("! Registers disabled");
                        } else if input.trim() == "reg on" {
                            show_registers = true;
                            println!("! Registers enabled");
                        } else if input.trim().starts_with("wait pos") {
                            match input.trim().split(" ").nth(2).unwrap().parse::<usize>() {
                                Ok(pos) => {
                                    wait_pos = Some(pos);
                                    println!("! Waiting at pos {}", pos);
                                }
                                Err(_) => {
                                    println!("!: Invalid position");
                                }
                            }
                        } else if input.trim().starts_with("wait pos disable") {
                            wait_pos = None;
                            println!("! Waiting disabled");
                        } else if input.trim().starts_with("code off") {
                            show_code = false;
                            println!("! Code disabled");
                        } else if input.trim().starts_with("code on") {
                            show_code = true;
                            println!("! Code enabled");
                        } else if input.trim().starts_with("stackinfo on") {
                            show_stack_info = true;
                            println!("! StackInfo enabled");
                        } else if input.trim().starts_with("stackinfo off") {
                            show_stack_info = false;
                            println!("! StackInfo disabled");
                        } else if input.trim().starts_with("step info") {
                            if last_step.is_some() {
                                step(
                                    &mut vm.heap,
                                    vm.threads[0].stack.last().unwrap().clone(),
                                    debug_file.clone(),
                                    last_step.clone().unwrap(),
                                    show_heap_dump,
                                    show_registers,
                                    wait_pos,
                                    show_code,
                                    show_stack_info,
                                );
                            } else {
                                println!("! No step info available, step once.");
                            }
                        } else if input.trim().starts_with("heap dump") {
                            println!(
                                "{}[VM]{}: Heap Dump
                            ",
                                cli_color.color(Colors::Yellow),
                                cli_color.color(Colors::Reset),
                            );
                            println!("{}", vm.heap_dump());
                        } else if input.trim().starts_with("help") {
                            println!("Commands:");
                            println!("  - run       - Start running the program");
                            println!("  - exit      - Exit the debugger");
                            println!("  - heap on   - Show heap dump");
                            println!("  - heap off  - Hide heap dump");
                            println!("  - reg on    - Show registers");
                            println!("  - reg off   - Hide registers");
                            println!(
                                "  - wait pos <pos> - Wait until the given stack position executed"
                            );
                            println!("  - wait pos disable - Disable waiting");
                            println!("  - code on - Show code");
                            println!("  - code off - Hide code");
                            println!("  - stackinfo on - Show stack info");
                            println!("  - stackinfo off - Hide stack info");
                            println!("  - clear - Clear the console");
                            println!("  - step info - Show last step info");
                            println!("  - heap dump - Show heap dump");
                        } else if input.trim() == "" {
                            std::io::stdout()
                                .write_all("\x1B[2J\x1B[1;1H".as_bytes())
                                .unwrap();
                            println!("---");
                            step_into = true;
                        } else {
                            std::io::stdout()
                                .write_all("\x1B[2J\x1B[1;1H".as_bytes())
                                .unwrap();
                            println!("Unknown command! Type 'help' for help");
                        }
                    }
                } else {
                    match vm.threads[0].step(&mut vm.heap) {
                        Ok(_) => (),
                        Err(e) => match e {
                            ThreadExit::Panic(e) => {
                                println!(
                                    "\n{}ThreadPanic{} : {}{:?}{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Cyan),
                                    e.reason,
                                    cli_color.color(Colors::Reset),
                                );

                                for frame in e.stack_trace {
                                    match &debug_file {
                                        Some(debug_file) => {
                                            let coresponding_header =
                                                debug_file.debug_headers.iter().find(|x| {
                                                    frame.stack_pos >= x.start_end.0
                                                        && frame.stack_pos <= x.start_end.1
                                                });

                                            match coresponding_header {
                                                Some(e) => {
                                                    fn get_real_path(
                                                        debug_header: &DebugHeader,
                                                        debug_file: &DebugInfo,
                                                    ) -> String
                                                    {
                                                        let module_name = debug_header
                                                            .module
                                                            .split("<ellie_module_")
                                                            .nth(1)
                                                            .unwrap()
                                                            .split(">")
                                                            .nth(0)
                                                            .unwrap();
                                                        let module_path = debug_file
                                                            .module_map
                                                            .iter()
                                                            .find(|map| {
                                                                module_name == map.module_name
                                                            });
                                                        let real_path = match module_path {
                                                            Some(module_path) => match &module_path
                                                                .module_path
                                                            {
                                                                Some(module_path) => {
                                                                    let new_path =
                                                                        debug_header.module.clone();
                                                                    let starter_name = format!(
                                                                        "<ellie_module_{}>",
                                                                        module_name
                                                                    );
                                                                    new_path.replace(
                                                                        &starter_name,
                                                                        &module_path,
                                                                    )
                                                                }
                                                                None => debug_header.module.clone(),
                                                            },
                                                            None => debug_header.module.clone(),
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
                                                        frame.name,
                                                        frame.stack_pos
                                                    );
                                                }
                                            }
                                        }
                                        None => {
                                            println!(
                                                "{}    at {}:{} ({} + {})",
                                                cli_color.color(Colors::Green),
                                                frame.name,
                                                frame.stack_pos + frame.frame_pos,
                                                frame.stack_pos,
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
                                    e.code_location,
                                );
                                if vm_settings.heap_dump {
                                    println!(
                                        "{}[VM]{}: Heap Dump\n\n{}",
                                        cli_color.color(Colors::Yellow),
                                        cli_color.color(Colors::Reset),
                                        vm.heap_dump()
                                    );
                                }
                                break;
                            }
                            ThreadExit::ExitGracefully => {
                                if vm_settings.heap_dump {
                                    println!(
                                        "{}[VM]{}: Heap Dump\n\n{}",
                                        cli_color.color(Colors::Yellow),
                                        cli_color.color(Colors::Reset),
                                        vm.heap_dump()
                                    );
                                }
                                break;
                            }
                        },
                    }
                }
            });

            main_thread.join().unwrap();

            if matches.is_present("vmDebug") {}
        }
        Some(("version", matches)) => {
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION_DETAILED.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: version,
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "build_date".to_string(),
                        value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "engine_version".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "engine_code".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "vm_version".to_string(),
                        value: engine_constants::ELLIE_VM_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieVM v{} ({}: {}){}\n\nEllie v{} - Code: {}\nVM Version: v{}\nCore version: v{}\n",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE,
                        if engine_constants::ELLIE_BUILD_GIT_BRANCH != "main" {
                            format!(
                                " [{}{}{}] ",
                                cli_color.color(Colors::Yellow),
                                engine_constants::ELLIE_BUILD_GIT_BRANCH,
                                cli_color.color(Colors::Reset)
                            )
                        } else {
                            String::new()
                        },
                        engine_constants::ELLIE_ENGINE_VERSION,
                        engine_constants::ELLIE_ENGINE_VERSION_NAME,
                        engine_constants::ELLIE_VM_VERSION,
                        engine_constants::ELLIE_CORE_VERSION,
                    );
                }
            } else {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_branch".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_BRANCH.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "build_date".to_string(),
                        value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieVM v{} ({} : {}){}",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE,
                        if engine_constants::ELLIE_BUILD_GIT_BRANCH != "main" {
                            format!(
                                " [{}{}{}] ",
                                cli_color.color(Colors::Yellow),
                                engine_constants::ELLIE_BUILD_GIT_BRANCH,
                                cli_color.color(Colors::Reset)
                            )
                        } else {
                            String::new()
                        },
                    );
                }
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
