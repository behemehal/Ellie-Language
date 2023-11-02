mod commands;
pub mod debugger;
mod debugger_messages;
mod run;
mod stream;
mod utils;

#[macro_use]
extern crate lazy_static;

use ellie_engine::{
    ellie_core::defs::PlatformArchitecture,
    ellie_renderer_utils::{
        options, outputs,
        utils::{CliColor, ColorDisplay, Colors, TextStyles},
    },
    ellie_vm::{
        channel::{EllieModule, FunctionElement, ModuleElements},
        program::Program,
        raw_type::StaticRawType,
        utils::{ProgramReader, VmNativeAnswer, VmNativeCallParameters},
    },
    engine_constants,
    vm::{parse_debug_file, RFile},
};
use run::VmSettings;

use std::{fs::File, io::Read, path::Path, time::SystemTime};

fn main() {
    let app = options::generate_ellievm_options();
    let matches = app.get_matches();
    let version = "0.1.0".to_string();
    let cli_color = &CliColor;

    match matches.subcommand() {
        Some(("run", matches)) => {
            let mut _debugger_wait = false;
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
                                .split('@')
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

            let mut vm_settings = VmSettings {
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
                modules: Vec::new(),
            };

            let mut ellie_core_module = EllieModule::new("ellieCore".to_string());
            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "println",
                Box::new(|_, args| {
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }
                    match &args[0] {
                        VmNativeCallParameters::Static(_e) => VmNativeAnswer::RuntimeError(
                            "Signature mismatch, expected 'dynamic' argument".to_string(),
                        ),
                        VmNativeCallParameters::Dynamic(dynamic_value) => {
                            if dynamic_value.is_string() {
                                eprintln!("{}", dynamic_value.to_string());
                                VmNativeAnswer::Ok(VmNativeCallParameters::Static(
                                    StaticRawType::from_void(),
                                ))
                            } else {
                                VmNativeAnswer::RuntimeError(
                                    "Signature mismatch expected 'string' argument".to_string(),
                                )
                            }
                        }
                    }
                }),
            )));

            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "timestamp",
                Box::new(|_, args| {
                    if !args.is_empty() {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 0 argument(s)".to_string(),
                        );
                    }
                    VmNativeAnswer::Ok(VmNativeCallParameters::Static(StaticRawType::from_int(
                        SystemTime::now()
                            .duration_since(SystemTime::UNIX_EPOCH)
                            .unwrap()
                            .as_nanos() as isize,
                    )))
                }),
            )));

            vm_settings.modules.push(ellie_core_module);

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

            let path = Path::new(matches.value_of("target").unwrap());
            let program = if path.exists() {
                if path.is_file() {
                    match File::open(path) {
                        Ok(mut e) => {
                            let mut reader = RFile::new(&mut e);
                            let mut program_reader = ProgramReader::new(&mut reader);
                            let mut program = Program::new();
                            match program.build_from_reader(&mut program_reader) {
                                Ok(_) => program,
                                Err(e) => {
                                    println!(
                                        "{}Error:{} Failed to read program {}[{:?}]{}",
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
            run::run(program, vm_settings, debug_file);
        }
        Some(("debug", matches)) => {
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
                                .split('@')
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

            /* let mut ellie_core_module = EllieModule::new("ellieCore".to_string());
            ellie_core_module.register_element(ModuleElements::Function(FunctionElement::new(
                "println",
                Box::new(|_, args| {
                    if args.len() != 1 {
                        return VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 1 argument(s)".to_string(),
                        );
                    }
                    match &args[0] {
                        VmNativeCallParameters::Static(_) => VmNativeAnswer::RuntimeError(
                            "Signature mismatch expected 'dynamic' argument".to_string(),
                        ),
                        VmNativeCallParameters::Dynamic(dynamic_value) => {
                            if dynamic_value.is_string() {
                                eprintln!("{}", dynamic_value.to_string());
                                VmNativeAnswer::Ok(VmNativeCallParameters::Static(
                                    StaticRawType::from_void(),
                                ))
                            } else {
                                VmNativeAnswer::RuntimeError(
                                    "Signature mismatch expected 'string' argument".to_string(),
                                )
                            }
                        }
                    }
                }),
            )));

            vm_settings.modules.push(ellie_core_module); */

            /* let path = Path::new(matches.value_of("target").unwrap().clone());
            let program = if path.exists() {
                if path.is_file() {
                    match File::open(path) {
                        Ok(mut e) => {
                            let mut reader = RFile::new(&mut e);
                            let mut program_reader = ProgramReader::new(&mut reader);
                            let mut program = Program::new();
                            match program.build_from_reader(&mut program_reader) {
                                Ok(_) => program,
                                Err(e) => {
                                    println!(
                                        "{}Error:{} Failed to read program {}[{:?}]{}",
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
            }; */

            let imported_commands = matches
                .values_of("insertCommands")
                .unwrap_or_default()
                .map(str::to_string)
                .collect::<Vec<_>>();
            debugger::debug(matches.is_present("jsonLog"), imported_commands);
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
            } else if matches.is_present("jsonLog") {
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
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
