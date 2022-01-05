use clap::ValueHint;
use clap::{App, AppSettings, Arg};
use ellie_engine::{cli_outputs, cli_utils};
use std::path::Path;

fn main() {
    println!("\x1B]0;{}\x07", "Ellie Compiler");

    let app = App::new("EllieCompiler")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommand(
            App::new("compile")
                .about("Compile option")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("disableWarnings")
                        .help("Disable warnings")
                        .short('d')
                        .long("-disable-warnings"),
                )
                .arg(
                    Arg::new("insertModule")
                        .help("Insert a module from binary")
                        .short('i')
                        .long("--insert-module")
                        .takes_value(true)
                        .multiple_values(true)
                        .value_hint(ValueHint::FilePath),
                )
                .arg(
                    Arg::new("binaryVersion")
                        .help("Binary version")
                        .short('b')
                        .long("--binary-version")
                        .default_value("1.0.0")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("description")
                        .help("Description of module")
                        .short('c')
                        .long("--module-description")
                        .default_value("A ellie package")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("moduleName")
                        .help("Name of module")
                        .short('m')
                        .long("--module-name")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("outputPath")
                        .help("Output path to write")
                        .short('p')
                        .long("--output-path")
                        .takes_value(true)
                        .value_hint(ValueHint::DirPath),
                )
                .arg(
                    Arg::new("outputType")
                        .help("Output type")
                        .short('o')
                        .long("--output-type")
                        .takes_value(true)
                        .default_value("bin"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target file to compile")
                        .takes_value(true)
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            App::new("viewModule")
                .about("Analyze given module information")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(
                    Arg::new("target")
                        .help("Target module to analyze")
                        .required(true)
                        .value_hint(ValueHint::FilePath),
                ),
        )
        .subcommand(
            App::new("version")
                .about("Get version")
                .arg(
                    Arg::new("jsonLog")
                        .help("Output json log")
                        .short('j')
                        .long("-json-log"),
                )
                .arg(Arg::new("detailed").short('d').long("--detailed-version")),
        );

    let matches = app.get_matches();
    match matches.subcommand() {
        Some(("compile", matches)) => {
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
                    let path = Path::new(module);

                    //If module path is file
                    if path.is_file() {
                        //If module path is file
                        match cli_utils::read_file_bin(path) {
                            Ok(file_content) => {
                                match bincode::deserialize::<ellie_parser::parser::Module>(
                                    file_content.as_slice(),
                                ) {
                                    Ok(module) => {
                                        let current_ellie_version =
                                            ellie_core::defs::Version::build_from_string(
                                                ellie_engine::engine_constants::ELLIE_VERSION
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
                                        parsed_modules.push(module);
                                    }
                                    Err(e) => {
                                        if matches.is_present("jsonLog") {
                                            let mut cli_module_output =
                                                cli_outputs::READ_BINARY_MODULE_ERROR.clone();
                                            cli_module_output.extra.push(
                                                cli_outputs::CliOuputExtraData { key: 0, value: 0 },
                                            );
                                            println!(
                                                "{}",
                                                serde_json::to_string_pretty(&cli_module_output)
                                                    .unwrap()
                                            );
                                        } else {
                                            println!(
                                                "{}Error{}: Failed to decode module '{}{}{}' [{}{}{}]].",
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
                            module,
                            cli_utils::Colors::Reset,
                        );
                        std::process::exit(1);
                    }
                }
                parsed_modules
            } else {
                vec![]
            };

            let compiler_settings = ellie_engine::compile_file::CompilerSettings {
                json_log: matches.is_present("jsonLog"),
                description: matches.value_of("description").unwrap().to_string(),
                name: matches
                    .value_of("moduleName")
                    .unwrap_or(&target_path)
                    .to_string(),
                version,
                output_type: matches.value_of("outputType").unwrap().to_string(),
                warnings: !matches.is_present("disableWarnings"),
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
                        value: ellie_engine::engine_constants::ELLIE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_VERSION_NAME.to_owned(),
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
                        key: "runtime_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_RUNTIME_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "core_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_CORE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "std_version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_STD_VERSION.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}\n\nTokenizer Version: v{}\nParser Version: v{}\nRuntime Version: v{}\nCore version: v{}\nEllie Standard Types Version: v{}\n",
                        ellie_engine::engine_constants::ELLIE_VERSION,
                        ellie_engine::engine_constants::ELLIE_VERSION_NAME,
                        ellie_engine::engine_constants::ELLIE_TOKENIZER_VERSION,
                        ellie_engine::engine_constants::ELLIE_PARSER_VERSION,
                        ellie_engine::engine_constants::ELLIE_RUNTIME_VERSION,
                        ellie_engine::engine_constants::ELLIE_CORE_VERSION,
                        ellie_engine::engine_constants::ELLIE_STD_VERSION,
                    );
                }
            } else {
                if matches.is_present("jsonLog") {
                    let mut output = cli_outputs::VERSION.clone();
                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "version".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_VERSION.to_owned(),
                    });

                    output.extra.push(cli_outputs::CliOuputExtraData {
                        key: "code".to_string(),
                        value: ellie_engine::engine_constants::ELLIE_VERSION_NAME.to_owned(),
                    });
                    println!("{}", serde_json::to_string(&output).unwrap());
                } else {
                    println!(
                        "Ellie v{} - Code: {}",
                        ellie_engine::engine_constants::ELLIE_VERSION,
                        ellie_engine::engine_constants::ELLIE_VERSION_NAME
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
