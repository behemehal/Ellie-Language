use ellie_engine::{
    ellie_core::defs::{PlatformArchitecture, Version},
    ellie_parser,
    ellie_renderer_utils::{
        options, outputs,
        utils::{read_file_bin, CliColor, ColorDisplay, Colors, TextStyles},
    },
    engine_constants,
};
use std::path::Path;
mod compile_file;
mod tokenize_file;
mod view_module;

#[derive(Debug, Clone, PartialEq)]
pub enum OutputTypesSelector {
    /// Outputs module in binary format. This is the default export mode for modules.
    Bin,
    /// Not supported yet
    DependencyAnalysis,
    /// Compiled module as json
    Json,
    /// ByteCode binary format
    ByteCode,
    /// ByteCode assembly text
    ByteCodeAsm,
    /// ByteCode debug file
    ByteCodeDebug,
    /// No output
    Nop,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputTypes {
    Bin,
    DependencyAnalysis,
    Json,
    ByteCode,
    ByteCodeAsm,
    ByteCodeDebug,
    Nop,
}

fn main() {
    let version = env!("CARGO_PKG_VERSION");
    let app = options::generate_elliec_options();
    let matches = app.get_matches();
    let cli_color = &CliColor;

    match matches.subcommand() {
        Some(("tokenize", matches)) => {
            if !matches.is_present("allowPanics") {
                std::panic::set_hook(Box::new(|e| {
                    if e.to_string().contains("@Halt") {
                        println!(
                            "\n\n{}-----------------{}\n",
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}Compiler halted{}\n",
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
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=compiler,bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20elliec%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_color.text_style(TextStyles::Underline),cli_color.color(Colors::Green),line_and_col, line_and_col, engine_constants::ELLIE_ENGINE_VERSION, cli_color.color(Colors::Reset));
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
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
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
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

            let tokenizer_settings = tokenize_file::TokenizerSettings {
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

            tokenize_file::tokenize(
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
                            cli_color.color(Colors::Blue),
                            cli_color.color(Colors::Reset)
                        );
                        println!(
                            "{}{}Compiler halted{}\n",
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
                    println!("\n{}{}https://github.com/behemehal/Ellie-Language/issues/new?labels=bug,Internal%20Error&title=Ellie%20Internal%20Error-{}&body=%23%20Ellie%20Internal%20Error%0AGenerated%20by%20elliec%20located%20at%20{}%0AEllieVersion:{}%0A{}", cli_color.text_style(TextStyles::Underline),cli_color.color(Colors::Green),line_and_col, line_and_col, engine_constants::ELLIE_ENGINE_VERSION, cli_color.color(Colors::Reset));
                    println!(
                        "\n{}-----------------{}\n\n",
                        cli_color.color(Colors::Blue),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }));
            }
            let version = Version::build_from_string_checked(
                matches.value_of("binaryVersion").unwrap().to_string(),
            )
            .unwrap_or_else(|_| {
                println!(
                    "{}Error:{} Given binary version does not fit to versioning format",
                    cli_color.color(Colors::Red),
                    cli_color.color(Colors::Reset)
                );
                std::process::exit(1);
            });

            let output_type = match matches.value_of("outputType").unwrap() {
                "bin" => OutputTypesSelector::Bin,
                "json" => OutputTypesSelector::Json,
                "byteCode" => OutputTypesSelector::ByteCode,
                "byteCodeAsm" => OutputTypesSelector::ByteCodeAsm,
                "depA" => OutputTypesSelector::DependencyAnalysis,
                "nop" => OutputTypesSelector::Nop,
                _ => {
                    println!(
                        "{}Error:{} Given output type does not exist",
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
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
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
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
                        match read_file_bin(module_path) {
                            Ok(file_content) => {
                                match bincode::deserialize::<ellie_parser::parser::Module>(
                                    file_content.as_slice(),
                                ) {
                                    Ok(module) => {
                                        if code_path.is_none()
                                            || Path::new(&code_path.clone().unwrap()).is_dir()
                                        {
                                            let current_ellie_version = Version::build_from_string(
                                                engine_constants::ELLIE_ENGINE_VERSION.to_owned(),
                                            );
                                            if current_ellie_version != module.ellie_version {
                                                if matches.is_present("jsonLog") {
                                                    let mut cli_module_output =
                                                        outputs::LEGACY_MODULE.clone();
                                                    cli_module_output.extra.push(
                                                        outputs::CliOuputExtraData {
                                                            key: 0,
                                                            value: module.ellie_version.clone(),
                                                        },
                                                    )
                                                } else {
                                                    println!(
                                                    "\n{}Info{}: This module is legacy, used ellie_version: {}{}.{}.{}{} current ellie_version: {}{}.{}.{}{}",
                                                        cli_color.color(Colors::Cyan),
                                                        cli_color.color(Colors::Reset),
                                                        cli_color.color(Colors::Yellow),
                                                        module.ellie_version.major,
                                                        module.ellie_version.minor,
                                                        module.ellie_version.bug,
                                                        cli_color.color(Colors::Reset),
                                                        cli_color.color(Colors::Yellow),
                                                        current_ellie_version.major,
                                                        current_ellie_version.minor,
                                                        current_ellie_version.bug,
                                                        cli_color.color(Colors::Reset),
                                                    );
                                                }
                                            }
                                            parsed_modules.push((module, code_path));
                                        } else {
                                            println!(
                                                    "{}Error:{} Module code path '{}{}{}' does not exist",
                                                    cli_color.color(Colors::Red),
                                                    cli_color.color(Colors::Reset),
                                                    cli_color.color(Colors::Yellow),
                                                    path[1],
                                                    cli_color.color(Colors::Reset),
                                                );
                                            std::process::exit(1);
                                        }
                                    }
                                    Err(e) => {
                                        if matches.is_present("jsonLog") {
                                            let mut cli_module_output =
                                                outputs::READ_BINARY_MODULE_ERROR.clone();
                                            cli_module_output.extra.push(
                                                outputs::CliOuputExtraData {
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
                                                cli_color.color(Colors::Red),
                                                cli_color.color(Colors::Reset),
                                                cli_color.color(Colors::Yellow),
                                                module,
                                                cli_color.color(Colors::Reset),
                                                cli_color.color(Colors::Yellow),
                                                e,
                                                cli_color.color(Colors::Reset),
                                            );
                                        }
                                        std::process::exit(1);
                                    }
                                }
                            }
                            Err(e) => {
                                println!(
                                    "{}Error:{} Cannot read module file '{}{}{}' {}[{}]{}",
                                    cli_color.color(Colors::Red),
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Yellow),
                                    module,
                                    cli_color.color(Colors::Reset),
                                    cli_color.color(Colors::Red),
                                    e,
                                    cli_color.color(Colors::Reset),
                                );
                                std::process::exit(1);
                            }
                        };
                    } else {
                        println!(
                            "{}Error:{} Module '{}{}{}' does not exist",
                            cli_color.color(Colors::Red),
                            cli_color.color(Colors::Reset),
                            cli_color.color(Colors::Yellow),
                            path[0].trim(),
                            cli_color.color(Colors::Reset),
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
                    cli_color.color(Colors::Red),
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Yellow),
                    project_name,
                    cli_color.color(Colors::Reset),
                    cli_color.color(Colors::Cyan),
                    if matches.value_of("moduleName").is_none() {
                        " (Which is the name of the file, you can change project name with '--module-name' option)"
                    } else {
                        ""
                    },
                    cli_color.color(Colors::Reset),
                );
                std::process::exit(1);
            }

            let compiler_settings = compile_file::CliCompilerSettings {
                json_log: matches.is_present("jsonLog"),
                exclude_std: matches.is_present("excludeStd"),
                compiler_settings: ellie_engine::utils::CompilerSettings {
                    description: matches.value_of("description").unwrap().to_string(),
                    name: project_name,
                    is_lib: matches.is_present("isLib"),
                    version,
                    experimental_features: matches.is_present("experimentalFeatures"),
                    byte_code_architecture: match matches.value_of("targetArchitecture") {
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
                    file_name: Path::new(&target_path)
                        .file_name()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                },
                output_type,
                performance_info: matches.is_present("performanceInfo"),
                show_debug_lines: matches.is_present("showDebugLines"),
                warnings: !matches.is_present("disableWarnings"),
            };

            compile_file::compile(
                Path::new(&target_path),
                Path::new(&output_path),
                modules,
                compiler_settings,
            );
        }
        Some(("version", matches)) => {
            if matches.is_present("detailed") {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION_DETAILED.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: version.to_string(),
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
                        key: "tokenizer_version".to_string(),
                        value: engine_constants::ELLIE_TOKENIZER_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "parser_version".to_string(),
                        value: engine_constants::ELLIE_PARSER_VERSION.to_owned(),
                    });

                    output.extra.push(outputs::CliOuputExtraData {
                        key: "bytecode_version".to_string(),
                        value: engine_constants::ELLIE_BYTECODE_VERSION.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieC v{} ({}: {})\n\nEllie v{} - Code: {}\nBytecode Version: v{}\nTokenizer Version: v{}\nParser Version: v{}\nCore version: v{}\n",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE,
                        engine_constants::ELLIE_ENGINE_VERSION,
                        engine_constants::ELLIE_ENGINE_VERSION_NAME,
                        engine_constants::ELLIE_BYTECODE_VERSION,
                        engine_constants::ELLIE_TOKENIZER_VERSION,
                        engine_constants::ELLIE_PARSER_VERSION,
                        engine_constants::ELLIE_CORE_VERSION,
                    );
                }
            } else {
                if matches.is_present("jsonLog") {
                    let mut output = outputs::VERSION.clone();
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: version.to_string(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "git_hash".to_string(),
                        value: engine_constants::ELLIE_BUILD_GIT_HASH.to_owned(),
                    });
                    output.extra.push(outputs::CliOuputExtraData {
                        key: "build_date".to_string(),
                        value: engine_constants::ELLIE_BUILD_DATE.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "EllieC v{} ({} : {}) ",
                        version,
                        engine_constants::ELLIE_BUILD_GIT_HASH,
                        engine_constants::ELLIE_BUILD_DATE
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
                        cli_color.color(Colors::Red),
                        cli_color.color(Colors::Reset)
                    );
                    std::process::exit(1);
                }
            };

            view_module::parse(Path::new(&target_path), matches.is_present("jsonLog"));
        }
        _ => unreachable!("clap should ensure we don't get here"),
    }
}
