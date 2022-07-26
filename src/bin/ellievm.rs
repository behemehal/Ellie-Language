use ellie_bytecode::utils::RawType;
use ellie_cli_utils::{
    options, outputs,
    utils::{self, Colors},
};
use ellie_core::defs::VmNativeAnswer;
use ellie_engine::{
    engine_constants,
    vm::{parse_debug_file, read_program, RFile},
};
use ellie_vm::utils::{ProgramReader, Reader};
use std::{fs::File, io::Read, path::Path};

pub struct VmSettings {
    pub json_log: bool,
    pub warnings: bool,
    pub heap_dump: bool,
    pub architecture: ellie_core::defs::PlatformArchitecture,
}

fn main() {
    let app = options::generate_ellievm_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("run", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            utils::Colors::Blue,
                            utils::Colors::Reset
                        );
                        println!(
                            "{}{}VM halted{}\n",
                            utils::Colors::Yellow,
                            utils::TextStyles::Bold,
                            utils::Colors::Reset
                        );
                        println!(
                            "{}{}{}",
                            utils::Colors::Blue,
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split("@")
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            utils::Colors::Red
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            utils::Colors::Blue,
                            utils::Colors::Reset
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        utils::Colors::Blue,
                        utils::Colors::Reset
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        utils::Colors::Red,
                        utils::TextStyles::Bold,
                        utils::Colors::Red
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        utils::Colors::Green,
                        utils::Colors::Reset,
                        utils::Colors::Yellow,
                        utils::Colors::Reset,
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=vm,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20ellievm%20located%20at%20{}%0AEllieVersion:{}%0A{}", utils::TextStyles::Underline,utils::Colors::Green,line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, utils::Colors::Reset);
                    println!(
                        "\n{}-----------------{}\n\n",
                        utils::Colors::Blue,
                        utils::Colors::Reset
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
                            ellie_core::defs::PlatformArchitecture::B64
                        } else if e == "32" {
                            ellie_core::defs::PlatformArchitecture::B32
                        } else if e == "16" {
                            ellie_core::defs::PlatformArchitecture::B16
                        } else {
                            println!(
                                "{}Error:{} Unknown architecture '{}{}{}'",
                                utils::Colors::Red,
                                utils::Colors::Reset,
                                utils::Colors::Yellow,
                                e,
                                utils::Colors::Reset,
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
                                e.read_to_string(&mut file_contents);
                                match parse_debug_file(file_contents) {
                                    Ok(e) => Some(e),
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
                    } else {
                        println!(
                            "{}Error:{} Given path is not a file",
                            utils::Colors::Red,
                            utils::Colors::Reset
                        );
                        std::process::exit(1);
                    }
                }
                None => None,
            };

            let path = Path::new(matches.value_of("target").unwrap().clone());
            let mut program = if path.exists() {
                if path.is_file() {
                    if path.is_file() {
                        let mut file_contents = String::new();
                        match File::open(path) {
                            Ok(mut e) => {
                                let mut reader = RFile::new(&mut e);
                                match read_program(&mut reader) {
                                    Ok(e) => e,
                                    Err(e) => {
                                        println!(
                                            "{}Error:{} Failed read program, error code: {}{}{}",
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
                            "{}Error:{} Given path is not a file",
                            utils::Colors::Red,
                            utils::Colors::Reset
                        );
                        std::process::exit(1);
                    }
                } else {
                    println!(
                        "{}Error:{} Given path is not a file",
                        utils::Colors::Red,
                        utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            } else {
                println!(
                    "{}Error:{} Target path does not exist",
                    utils::Colors::Red,
                    utils::Colors::Reset
                );
                std::process::exit(1);
            };

            let mut vm = ellie_vm::vm::VM::new(vm_settings.architecture, |_, e| {
                if e.module == "ellieStd" && e.name == "println" {
                    let string = String::from_utf8(e.params[0].data.clone());
                    println!("{}", string.unwrap());
                    VmNativeAnswer::Ok(().into())
                } else if e.module == "main" && e.name == "±get_val_n" {
                    VmNativeAnswer::Ok("Cevaaap".into())
                } else {
                    VmNativeAnswer::RuntimeError("Call to unknown function".into())
                }
            });
            vm.load(&program);
            match vm.run(program.main) {
                ellie_vm::utils::ThreadExit::Panic(e) => {
                    println!(
                        "\n{}ThreadPanic{} : {}{:?}{}",
                        Colors::Red,
                        Colors::Reset,
                        Colors::Cyan,
                        e.reason,
                        Colors::Reset,
                    );

                    for frame in e.stack_trace {
                        match &debug_file {
                            Some(debug_file) => {
                                let coresponding_header =
                                    debug_file.debug_headers.iter().find(|x| {
                                        frame.pos >= x.start_end.0 && frame.pos <= x.start_end.1
                                    });

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
                                        let module_path = debug_file
                                            .module_map
                                            .iter()
                                            .find(|map| module_name == map.module_name);
                                        let real_path = match module_path {
                                            Some(module_path) => match &module_path.module_path {
                                                Some(module_path) => {
                                                    let new_path = e.module.clone();
                                                    let starter_name =
                                                        format!("<ellie_module_{}>", module_name);
                                                    new_path.replace(&starter_name, &module_path)
                                                }
                                                None => e.module.clone(),
                                            },
                                            None => e.module.clone(),
                                        };

                                        println!(
                                            "{}    at {}:{}:{}",
                                            Colors::Green,
                                            real_path,
                                            e.pos.range_start.0 + 1,
                                            e.pos.range_start.1 + 1,
                                        );
                                    }
                                    None => {
                                        println!(
                                            "{}    at {}:{}",
                                            Colors::Green,
                                            frame.name,
                                            frame.pos
                                        );
                                    }
                                }
                            }
                            None => {
                                println!("{}    at {}:{}", Colors::Green, frame.name, frame.pos);
                            }
                        }
                    }
                    if debug_file.is_none() {
                        println!(
                            "\n{}NoDebugFile{} : {}Given error represents stack locations, provide a debug info file to get more readable info{}",
                            Colors::Yellow,
                            Colors::Reset,
                            Colors::Cyan,
                            Colors::Reset,
                        );
                    }
                    println!("{}    at {}", Colors::Red, e.code_location,);
                    if vm_settings.heap_dump {
                        println!(
                            "{}[VM]{}: Heap Dump\n\n{}",
                            Colors::Yellow,
                            Colors::Reset,
                            vm.heap_dump()
                        );
                    }
                }
                ellie_vm::utils::ThreadExit::ExitGracefully => {
                    if vm_settings.heap_dump {
                        println!(
                            "{}[VM]{}: Heap Dump\n\n{}",
                            Colors::Yellow,
                            Colors::Reset,
                            vm.heap_dump()
                        );
                    }
                }
            }
        }
        Some(("version", matches)) => {
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION_DETAILED.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "code".to_string(),
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
                        "Ellie v{} - Code: {}\nVM Version: v{}\nCore version: v{}\n",
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
                        key: "code".to_string(),
                        value: engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}",
                        engine_constants::ELLIE_ENGINE_VERSION,
                        engine_constants::ELLIE_ENGINE_VERSION_NAME
                    );
                }
            }
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
