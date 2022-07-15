use ellie_engine::{cli_options, cli_outputs, cli_utils};
use std::path::Path;

#[derive(Debug, Clone)]
struct EllieError {}

fn main() {
    let app = cli_options::generate_elliec_options();
    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("tokenize", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}Compiler halted{}\n",
                            cli_utils::Colors::Yellow,
                            cli_utils::TextStyles::Bold,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}{}",
                            cli_utils::Colors::Blue,
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split("@")
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_utils::Colors::Red
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        return;
                    }

                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_utils::Colors::Red,
                        cli_utils::TextStyles::Bold,
                        cli_utils::Colors::Red
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_utils::Colors::Green,
                        cli_utils::Colors::Reset,
                        cli_utils::Colors::Yellow,
                        cli_utils::Colors::Reset,
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=compiler,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20elliec%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_utils::TextStyles::Underline,cli_utils::Colors::Green,line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, cli_utils::Colors::Reset);
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }));
            }

            let target_path = {
                let path = Path::new(matches.value_of("target").unwrap().clone());
                if path.exists() {
                    matches.value_of("target").unwrap().to_string()
                } else {
                    println!(
                        "{}Error:{} Target path does not exist",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            };

            let project_name = {
                let file_name = Path::new(&target_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap();

                if file_name.contains(".") {
                    file_name.split(".").next().unwrap().to_string()
                } else {
                    file_name.to_string()
                }
            };

            let tokenizer_settings = ellie_engine::tokenize_file::TokenizerSettings {
                json_log: matches.is_present("jsonLog"),
                name: project_name,
                file_name: Path::new(&target_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                show_debug_lines: matches.is_present("showDebugLines"),
            };

            ellie_engine::tokenize_file::tokenize(
                Path::new(&target_path),
                Path::new(
                    &Path::new(&target_path)
                        .parent()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                ),
                tokenizer_settings,
            );
        }
        Some(("compile", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}Compiler halted{}\n",
                            cli_utils::Colors::Yellow,
                            cli_utils::TextStyles::Bold,
                            cli_utils::Colors::Reset
                        );
                        println!(
                            "{}{}{}",
                            cli_utils::Colors::Blue,
                            e.to_string().split("@Halt:").collect::<Vec<&str>>()[1]
                                .split("@")
                                .collect::<Vec<&str>>()[0]
                                .trim(),
                            cli_utils::Colors::Red
                        );
                        println!(
                            "\n{}-----------------{}\n\n",
                            cli_utils::Colors::Blue,
                            cli_utils::Colors::Reset
                        );
                        return;
                    }
                    println!(
                        "\n\n{}-----------------{}\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
                    );
                    println!(
                        "{}{}Oh no! A internal error occured;{}",
                        cli_utils::Colors::Red,
                        cli_utils::TextStyles::Bold,
                        cli_utils::Colors::Red
                    );
                    println!(
                        "{}Can you please share this error with us? This can help us solve issue faster. All you have to do is follow the link below {}[{}CTRL + Mouse Left Click might help{}]",
                        cli_utils::Colors::Green,
                        cli_utils::Colors::Reset,
                        cli_utils::Colors::Yellow,
                        cli_utils::Colors::Reset,
                    );

                    let line_and_col = if let Some(real_loc) = e.location() {
                        format!("{}:{}", real_loc.line(), real_loc.column())
                    } else {
                        "?:?".to_string()
                    };
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20elliec%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_utils::TextStyles::Underline,cli_utils::Colors::Green,line_and_col, line_and_col, ellie_engine::engine_constants::ELLIE_ENGINE_VERSION, cli_utils::Colors::Reset);
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_utils::Colors::Blue,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }));
            }
            let version = ellie_core::defs::Version::build_from_string_checked(
                matches.value_of("binaryVersion").unwrap().to_string(),
            )
            .unwrap_or_else(|_| {
                println!(
                    "{}Error:{} Given binary version does not fit to versioning format",
                    cli_utils::Colors::Red,
                    cli_utils::Colors::Reset
                );
                std::process::exit(1);
            });

            let output_type = match matches.value_of("outputType").unwrap() {
                "bin" => cli_utils::OutputTypes::Bin,
                "json" => cli_utils::OutputTypes::Json,
                "byteCode" => cli_utils::OutputTypes::ByteCode,
                "byteCodeAsm" => cli_utils::OutputTypes::ByteCodeAsm,
                "depA" => cli_utils::OutputTypes::DependencyAnalysis,
                "nop" => cli_utils::OutputTypes::Nop,
                _ => {
                    println!(
                        "{}Error:{} Given output type does not exist",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            };

            let target_path = {
                let path = Path::new(matches.value_of("target").unwrap().clone());
                if path.exists() {
                    if path.is_file() {
                        matches.value_of("target").unwrap().to_string()
                    } else {
                        println!(
                            "{}Error:{} Given path is not a file",
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset
                        );
                        std::process::exit(1);
                    }
                } else {
                    println!(
                        "{}Error:{} Target path does not exist",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            };

            let output_path = if let Some(output) = matches.value_of("outputPath") {
                let path = Path::new(output.clone());

                //Check the output path is exists or check path is file and parent directory exists
                if path.exists()
                    || (path.is_file() && path.parent().is_some()
                        || path.parent().unwrap().exists())
                {
                    output.to_string()
                } else {
                    println!(
                        "{}Error:{} Output path does not exist",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            } else {
                Path::new(&target_path)
                    .parent()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            };

            let modules = if let Some(modules) = matches.values_of("insertModule") {
                let mut parsed_modules = vec![];

                //Iter through all modules
                for module in modules {
                    let path = module.trim().split("=").collect::<Vec<_>>();

                    let module_path = Path::new(path[0].trim());
                    let code_path = if path.len() > 1 {
                        Some(path[1].trim().to_string())
                    } else {
                        None
                    };

                    //If module path is file
                    if module_path.is_file() {
                        //If module path is file
                        match cli_utils::read_file_bin(module_path) {
                            Ok(file_content) => {
                                match bincode::deserialize::<ellie_parser::parser::Module>(
                                    file_content.as_slice(),
                                ) {
                                    Ok(module) => {
                                        if code_path.is_none()
                                            || Path::new(&code_path.clone().unwrap()).is_dir()
                                        {
                                            let current_ellie_version =
                                                ellie_core::defs::Version::build_from_string(
                                                    ellie_engine::engine_constants::ELLIE_ENGINE_VERSION
                                                        .to_owned(),
                                                );
                                            if current_ellie_version != module.ellie_version {
                                                if matches.is_present("jsonLog") {
                                                    let mut cli_module_output =
                                                        crate::cli_outputs::LEGACY_MODULE.clone();
                                                    cli_module_output.extra.push(
                                                        cli_outputs::CliOuputExtraData {
                                                            key: 0,
                                                            value: module.ellie_version.clone(),
                                                        },
                                                    )
                                                } else {
                                                    println!(
                                                    "\n{}Info{}: This module is legacy, used ellie_version: {}{}.{}.{}{} current ellie_version: {}{}.{}.{}{}",
                                                        cli_utils::Colors::Cyan,
                                                        cli_utils::Colors::Reset,
                                                        cli_utils::Colors::Yellow,
                                                        module.ellie_version.major,
                                                        module.ellie_version.minor,
                                                        module.ellie_version.bug,
                                                        cli_utils::Colors::Reset,
                                                        cli_utils::Colors::Yellow,
                                                        current_ellie_version.major,
                                                        current_ellie_version.minor,
                                                        current_ellie_version.bug,
                                                        cli_utils::Colors::Reset,
                                                    );
                                                }
                                            }
                                            parsed_modules.push((module, code_path));
                                        } else {
                                            println!(
                                                    "{}Error:{} Module code path '{}{}{}' does not exist",
                                                    cli_utils::Colors::Red,
                                                    cli_utils::Colors::Reset,
                                                    cli_utils::Colors::Yellow,
                                                    path[1],
                                                    cli_utils::Colors::Reset,
                                                );
                                            std::process::exit(1);
                                        }
                                    }
                                    Err(e) => {
                                        if matches.is_present("jsonLog") {
                                            let mut cli_module_output =
                                                cli_outputs::READ_BINARY_MODULE_ERROR.clone();
                                            cli_module_output.extra.push(
                                                cli_outputs::CliOuputExtraData {
                                                    key: "file".to_string(),
                                                    value: module_path
                                                        .to_str()
                                                        .unwrap()
                                                        .to_string(),
                                                },
                                            );
                                            println!(
                                                "{}",
                                                serde_json::to_string_pretty(&cli_module_output)
                                                    .unwrap()
                                            );
                                        } else {
                                            println!(
                                                "{}Error{} 0x2: Failed to decode module '{}{}{}' [{}{}{}]].",
                                                cli_utils::Colors::Red,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Yellow,
                                                module,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Yellow,
                                                e,
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Cannot read module file '{}{}{}' {}[{}]{}",
                                    cli_utils::Colors::Red,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Yellow,
                                    module,
                                    cli_utils::Colors::Reset,
                                    cli_utils::Colors::Red,
                                    e,
                                    cli_utils::Colors::Reset,
                                );
                                std::process::exit(1);
                            }
                        };
                    } else {
                        println!(
                            "{}Error:{} Module '{}{}{}' does not exist",
                            cli_utils::Colors::Red,
                            cli_utils::Colors::Reset,
                            cli_utils::Colors::Yellow,
                            path[0].trim(),
                            cli_utils::Colors::Reset,
                        );
                        std::process::exit(1);
                    }
                }
                parsed_modules
            } else {
                vec![]
            };

            let project_name = match matches.value_of("moduleName") {
                Some(e) => e.to_string(),
                None => {
                    let file_name = Path::new(&target_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap();

                    if file_name.contains(".") {
                        file_name.split(".").next().unwrap().to_string()
                    } else {
                        file_name.to_string()
                    }
                }
            };

            if project_name.contains(" ")
                || project_name.contains("/")
                || project_name.contains(".")
            {
                println!(
                    "{}Error:{} Wrong project name '{}{}{}'{}{}{}",
                    cli_utils::Colors::Red,
                    cli_utils::Colors::Reset,
                    cli_utils::Colors::Yellow,
                    project_name,
                    cli_utils::Colors::Reset,
                    cli_utils::Colors::Cyan,
                    if matches.value_of("moduleName").is_none() {
                        " (Which is the name of the file, you can change project name with '--module-name' option)"
                    } else {
                        ""
                    },
                    cli_utils::Colors::Reset,
                );
                std::process::exit(1);
            }

            let compiler_settings = ellie_engine::compile_file::CompilerSettings {
                json_log: matches.is_present("jsonLog"),
                description: matches.value_of("description").unwrap().to_string(),
                name: project_name,
                is_lib: matches.is_present("isLib"),
                version,
                output_type,
                experimental_features: matches.is_present("experimentalFeatures"),
                exclude_stdlib: matches.is_present("excludeStd"),
                performance_info: matches.is_present("performanceInfo"),
                file_name: Path::new(&target_path)
                    .file_name()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string(),
                show_debug_lines: matches.is_present("showDebugLines"),
                warnings: !matches.is_present("disableWarnings"),
                byte_code_architecture: match matches.value_of("targetArchitecture") {
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
                                cli_utils::Colors::Red,
                                cli_utils::Colors::Reset,
                                cli_utils::Colors::Yellow,
                                e,
                                cli_utils::Colors::Reset,
                            );
                            std::process::exit(1);
                        }
                    }
                    None => unreachable!(),
                },
            };

            ellie_engine::compile_file::compile(
                Path::new(&target_path),
                Path::new(&output_path),
                modules,
                compiler_settings,
            );
        }
        Some(("version", matches)) => {
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    let mut output = cli_outputs::VERSION_DETAILED.clone();
                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "tokenizer_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_TOKENIZER_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "parser_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_PARSER_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "bytecode_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_BYTECODE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}\n\nBytecode Version: v{}\nTokenizer Version: v{}\nParser Version: v{}\nCore version: v{}\n",
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION,
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME,
                        ellie_engine::engine_constants::ELLIE_BYTECODE_VERSION,
                        ellie_engine::engine_constants::ELLIE_TOKENIZER_VERSION,
                        ellie_engine::engine_constants::ELLIE_PARSER_VERSION,
                        ellie_engine::engine_constants::ELLIE_CORE_VERSION,
                    );
                }
            } else {
                if matches.is_present("jsonLog") {
                    let mut output = cli_outputs::VERSION.clone();
                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}",
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION,
                        ellie_engine::engine_constants::ELLIE_ENGINE_VERSION_NAME
                    );
                }
            }
        }
        Some(("viewModule", matches)) => {
            let target_path = {
                let path = Path::new(matches.value_of("target").unwrap().clone());
                if path.exists() {
                    matches.value_of("target").unwrap().to_string()
                } else {
                    println!(
                        "{}Error:{} Target path does not exist",
                        cli_utils::Colors::Red,
                        cli_utils::Colors::Reset
                    );
                    std::process::exit(1);
                }
            };

            ellie_engine::view_module::parse(
                Path::new(&target_path),
                matches.is_present("jsonLog"),
            );
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
