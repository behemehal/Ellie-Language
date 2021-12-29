use ellie_engine::cli_outputs;
use ellie_engine::cli_utils;
use ellie_parser::parser;
use ellie_tokenizer::tokenizer::{self, ResolvedImport};
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;

fn main() {
    println!("{}]0;{}{}", '\u{001b}', "Ellie", '\u{007}');
    if env::args().any(|x| x == "-v" || x == "--version" || x == "-dv" || x == "--detailed-version")
    {
        if env::args().any(|x| x == "-dv" || x == "--detailed-version") {
            println!(
                "Ellie v{} - Code: {}\n\nTokenizer Version: v{}\nParser Version: v{}\nRuntime Version: v{}\nCore version: v{}\nEllie Standard Types Version: v{}\n",
                ellie_engine::cli_constants::ELLIE_VERSION,
                ellie_engine::cli_constants::ELLIE_VERSION_NAME,
                ellie_engine::cli_constants::ELLIE_TOKENIZER_VERSION,
                ellie_engine::cli_constants::ELLIE_PARSER_VERSION,
                ellie_engine::cli_constants::ELLIE_RUNTIME_VERSION,
                ellie_engine::cli_constants::ELLIE_CORE_VERSION,
                ellie_engine::cli_constants::ELLIE_STD_VERSION,
            );
        } else {
            println!(
                "Ellie v{} - Code: {}",
                ellie_engine::cli_constants::ELLIE_VERSION,
                ellie_engine::cli_constants::ELLIE_VERSION_NAME
            );
        }
    } else if env::args().any(|x| x == "-h" || x == "--help") {
        println!("Ellie compiler");
        println!("Usage: elliec [options] [file path | code]");
        println!("Options:");
        println!("\t--version                    || -v   : Show version");
        println!("\t--version                    || -dv  : Show version with internal components");
        println!("\t--help                       || -h   : Show help");
        println!("\t--output                     || -o   : [Required] Desired code output; bin, json, depA");
        println!("\t--output-path                || -op  : [Required] Compiled output path");
        println!("\t--set-version                || -sv  : [Required] Set version for workspace");
        println!("\t--json-output                || -jo  : Make error and warning output as json");
        println!("\t--disable-warnings           || -dw  : Disable warnings");
    } else {
        let args = env::args()
            .collect::<Vec<String>>()
            .drain(1..)
            .collect::<Vec<String>>();

        let mut cli_output_type = cli_utils::CliOutputType::ConsoleOutput;
        let mut disable_warnings = false;
        if args.is_empty() {
            println!("No file present\n-h for help");
            std::process::exit(1);
        } else if !env::args().any(|x| x == "-o" || x == "--output") {
            println!("No ouput present\n-h for help");
            std::process::exit(1);
        } else if !env::args().any(|x| x == "-op" || x == "--output-path") {
            println!("No ouput path present\n-h for help");
            std::process::exit(1);
        } else if !env::args().any(|x| x == "-sv" || x == "--set-version") {
            println!("No version name is set\n-h for help");
            std::process::exit(1);
        } else {
            let output_arg = env::args()
                .position(|x| x == "-o" || x == "--output")
                .unwrap();

            let output_path_arg = env::args()
                .position(|x| x == "-op" || x == "--output-path")
                .unwrap();

            let version_arg = env::args()
                .position(|x| x == "-sv" || x == "--set-version")
                .unwrap();

            if output_arg == env::args().len() {
                println!("No ouput type present\n-h for help");
                std::process::exit(1);
            }

            if output_path_arg == env::args().len() {
                println!("No ouput name present\n-h for help");
                std::process::exit(1);
            }

            if version_arg == env::args().len() {
                println!("No version data presented to be set as workspace version\n-h for help");
                std::process::exit(1);
            }

            let output_type = env::args().nth(output_arg + 1).unwrap();
            let output_path = env::args().nth(output_path_arg + 1).unwrap();
            let version_str = env::args().nth(version_arg + 1).unwrap();
            let given_workspace_version = match ellie_core::defs::Version::build_from_string_checked(
                version_str,
            ) {
                Ok(version) => version,
                Err(_) => {
                    println!("Presented version data is not correct, expected version type: '1.1.1'; Minor.Major.Bug\n-h for help");
                    std::process::exit(1);
                }
            };

            let prefered_output_type = if output_type == "bin"
                || output_type == "json"
                || output_type == "depA"
            {
                match output_type.as_str() {
                    "bin" => cli_utils::OutputTypes::Bin,
                    "json" => cli_utils::OutputTypes::Json,
                    "depA" => cli_utils::OutputTypes::DependencyAnalysis,
                    _ => unreachable!(),
                }
            } else {
                println!("Unknown ouput type present\nAvailable:\n\tbin, json, depA\n-h for help");
                std::process::exit(1);
            };

            let file_args = args
                .into_iter()
                .enumerate()
                .filter(|(index, path)| {
                    Path::new(path).is_file() && index != &(output_path_arg)
                })
                .map(|x| x.1)
                .collect::<Vec<String>>();

            match file_args.first() {
                Some(main_path) => match cli_utils::read_file(main_path) {
                    Ok(file_content) => {
                        let mut pager = tokenizer::Pager::new(
                            file_content,
                            Path::new(main_path).to_str().unwrap().to_string(),
                            |path, file_name| {
                                let path = Path::new(&path)
                                    .parent()
                                    .unwrap()
                                    .to_str()
                                    .unwrap()
                                    .to_string();
                                let file = if cli_utils::file_exists(
                                    path.clone() + "/" + &file_name.clone(),
                                ) {
                                    Some(path.clone() + "/" + &file_name.clone())
                                } else if cli_utils::file_exists(
                                    path.clone() + "/" + &file_name.clone() + ".ei",
                                ) {
                                    Some(path.clone() + "/" + &file_name.clone() + ".ei")
                                } else {
                                    None
                                };

                                if file_name == "ellie" {
                                    ResolvedImport {
                                        found: true,
                                        matched: tokenizer::ImportType::Module(tokenizer::Module {
                                            hash: 343,
                                            initial_page: 343,
                                            version:
                                                ellie_core::builded_libraries::ELLIE_STD_VERSION
                                                    .clone(),
                                            name: "ellie".to_owned(),
                                        }),
                                        hash: 343,
                                        path: "<ellie_virtual>".to_string(),
                                        ..Default::default()
                                    }
                                } else {
                                    match file {
                                        Some(file) => {
                                            let file = Path::new(&file).absolutize().unwrap();
                                            match cli_utils::read_file(
                                                &file.to_str().unwrap().to_string(),
                                            ) {
                                                Ok(ext) => {
                                                    let mut hasher = DefaultHasher::new();
                                                    ext.hash(&mut hasher);
                                                    ResolvedImport {
                                                        found: true,
                                                        matched: ellie_tokenizer::tokenizer::ImportType::Code(ext),
                                                        hash: hasher.finish(),
                                                        path: file.to_str().unwrap().to_string(),
                                                        ..Default::default()
                                                    }
                                                }
                                                Err(err) => ResolvedImport {
                                                    found: false,
                                                    resolve_error: err,
                                                    ..Default::default()
                                                },
                                            }
                                        }
                                        None => ResolvedImport {
                                            found: false,
                                            ..Default::default()
                                        },
                                    }
                                }
                            },
                            None,
                        );

                        match pager.run() {
                            Err(pager_errors) => {
                                if cli_output_type == cli_utils::CliOutputType::Json {
                                    for error in pager_errors.clone() {
                                        let mut output = cli_outputs::COMPILER_ERROR.clone();
                                        output.extra.push(cli_outputs::CliOuputExtraData {
                                            key: "error".to_string(),
                                            value: error,
                                        });

                                        println!("{}", serde_json::to_string(&output).unwrap());
                                    }
                                } else {
                                    cli_utils::print_errors(&pager_errors, |path| {
                                        match cli_utils::read_file(&path) {
                                            Ok(e) => e,
                                            Err(err) => {
                                                println!(
                                                    "Cannot read file '{}' {}[{}]{}",
                                                    path,
                                                    cli_utils::Colors::Red,
                                                    err,
                                                    cli_utils::Colors::Reset
                                                );
                                                std::process::exit(1);
                                            }
                                        }
                                    });
                                }
                            }
                            Ok(_) => {
                                let mut parser = parser::Parser::new(
                                    pager.pages.clone(),
                                    None,
                                    given_workspace_version,
                                );

                                if !env::args().any(|x| x == "-es" || x == "--exclude-std") {
                                    let ellie_module: parser::Module = serde_json::from_str(
                                        ellie_core::builded_libraries::ELLIE_STANDARD_LIBRARY,
                                    )
                                    .unwrap();

                                    parser.import_module(ellie_module)
                                    //parser.import_processed_module(std_pages);
                                }

                                if env::args().any(|x| x == "-jo" || x == "--json-output") {
                                    cli_output_type = cli_utils::CliOutputType::Json;
                                }

                                if env::args().any(|x| x == "-dw" || x == "--disable-warnings") {
                                    disable_warnings = true;
                                }

                                let workspace = parser.parse(
                                    Path::new(main_path)
                                        .file_name()
                                        .unwrap()
                                        .to_str()
                                        .unwrap()
                                        .to_owned(),
                                );

                                if !parser.informations.has_no_warnings() && !disable_warnings {
                                    if cli_output_type == cli_utils::CliOutputType::Json {
                                        for warning in parser.informations.warnings.clone() {
                                            println!(
                                                "{}",
                                                serde_json::to_string(&warning).unwrap()
                                            );
                                        }
                                    } else {
                                        cli_utils::print_warnings(
                                            &parser.informations.warnings,
                                            |path| match cli_utils::read_file(&path) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    println!(
                                                        "Cannot read file '{}' {}[{}]{}",
                                                        path,
                                                        cli_utils::Colors::Red,
                                                        err,
                                                        cli_utils::Colors::Reset
                                                    );
                                                    std::process::exit(1);
                                                }
                                            },
                                        );
                                    }
                                }

                                if !parser.informations.has_no_errors() {
                                    if cli_output_type == cli_utils::CliOutputType::Json {
                                        for error in parser.informations.errors.clone() {
                                            println!("{}", serde_json::to_string(&error).unwrap());
                                        }
                                    } else {
                                        cli_utils::print_errors(
                                            &parser.informations.errors,
                                            |path| match cli_utils::read_file(&path) {
                                                Ok(e) => e,
                                                Err(err) => {
                                                    println!(
                                                        "Failed to ouput error. Cannot read file '{}' {}[{}]{}",
                                                        path,
                                                        cli_utils::Colors::Red,
                                                        err,
                                                        cli_utils::Colors::Reset
                                                    );
                                                    std::process::exit(1);
                                                }
                                            },
                                        );
                                    }

                                    if parser.informations.warnings.len() == 0 {
                                        if cli_output_type == cli_utils::CliOutputType::Json {
                                            let mut output = cli_outputs::COMPILE_FAILED_WITH_ERRORS_WITH_NO_WARNINGS.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
                                                key: "errors".to_string(),
                                                value: parser
                                                    .informations
                                                    .errors
                                                    .len()
                                                    .clone()
                                                    .to_string(),
                                            });
                                            println!("{}", serde_json::to_string(&output).unwrap())
                                        } else {
                                            println!(
                                                "\nCompiling {}failed{} with {}{} errors{}",
                                                cli_utils::Colors::Red,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Red,
                                                parser.informations.errors.len(),
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    } else {
                                        if cli_output_type == cli_utils::CliOutputType::Json {
                                            let mut output = cli_outputs::COMPILE_FAILED_WITH_ERRORS_WITH_WARNINGS.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
                                                key: "errors".to_string(),
                                                value: parser
                                                    .informations
                                                    .errors
                                                    .len()
                                                    .clone()
                                                    .to_string(),
                                            });
                                            output.extra.push(cli_outputs::CliOuputExtraData {
                                                key: "warnings".to_string(),
                                                value: parser.informations.errors.len().to_string(),
                                            });
                                            println!("{}", serde_json::to_string(&output).unwrap())
                                        } else {
                                            println!("\nCompiling {}failed{} with {}{} errors{} and {}{} warnings{}.",
                                                cli_utils::Colors::Red,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Red,
                                                parser.informations.errors.len(),
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Yellow,
                                                parser.informations.warnings.len(),
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    }
                                } else {
                                    if parser.informations.warnings.len() == 0 {
                                        if cli_output_type == cli_utils::CliOutputType::Json {
                                            println!(
                                                "{}",
                                                serde_json::to_string(
                                                    &cli_outputs::COMPILE_SUCCESS_WITH_NO_WARNINGS
                                                        .clone()
                                                )
                                                .unwrap()
                                            )
                                        } else {
                                            println!(
                                                "\nCompiling {}succeeded{}.",
                                                cli_utils::Colors::Green,
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    } else {
                                        if cli_output_type == cli_utils::CliOutputType::Json {
                                            let mut output =
                                                cli_outputs::COMPILE_SUCCESS_WITH_WARNINGS.clone();
                                            output.extra.push(cli_outputs::CliOuputExtraData {
                                                key: "warnings".to_string(),
                                                value: parser.informations.errors.len().to_string(),
                                            });
                                            println!("{}", serde_json::to_string(&output).unwrap())
                                        } else {
                                            println!(
                                                "\nCompiling {}succeeded{} with {}{} warnings{}.",
                                                cli_utils::Colors::Green,
                                                cli_utils::Colors::Reset,
                                                cli_utils::Colors::Yellow,
                                                parser.informations.warnings.len(),
                                                cli_utils::Colors::Reset,
                                            );
                                        }
                                    }

                                    match prefered_output_type {
                                        cli_utils::OutputTypes::Bin => {
                                            let bytes = bincode::serialize(&workspace).unwrap();

                                            match Path::new(&output_path).absolutize() {
                                                Ok(resolved_path) => {
                                                    if let Err(write_error) =
                                                        fs::write(resolved_path, bytes)
                                                    {
                                                        if cli_output_type
                                                            == cli_utils::CliOutputType::Json
                                                        {
                                                            let mut output =
                                                                cli_outputs::WRITE_FILE_ERROR
                                                                    .clone();

                                                            output.extra.push(
                                                                cli_outputs::CliOuputExtraData {
                                                                    key: "path".to_string(),
                                                                    value: format!(
                                                                        "{:?}",
                                                                        write_error
                                                                    ),
                                                                },
                                                            );

                                                            println!(
                                                                "{}",
                                                                serde_json::to_string(&output)
                                                                    .unwrap()
                                                            )
                                                        } else {
                                                            println!(
                                                                "\nFailed to write output. [{}{:?}{}]",
                                                                cli_utils::Colors::Red,
                                                                write_error,
                                                                cli_utils::Colors::Reset,
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(path_error) => {
                                                    if cli_output_type
                                                        == cli_utils::CliOutputType::Json
                                                    {
                                                        let mut output =
                                                            cli_outputs::PATH_ERROR.clone();

                                                        output.extra.push(
                                                            cli_outputs::CliOuputExtraData {
                                                                key: "path".to_string(),
                                                                value: format!("{:?}", path_error),
                                                            },
                                                        );

                                                        println!(
                                                            "{}",
                                                            serde_json::to_string(&output).unwrap()
                                                        )
                                                    } else {
                                                        println!(
                                                                "\nFailed to find output path. [{}{:?}{}]",
                                                                cli_utils::Colors::Red,
                                                                path_error,
                                                                cli_utils::Colors::Reset,
                                                            );
                                                    }
                                                }
                                            }
                                        }
                                        cli_utils::OutputTypes::DependencyAnalysis => todo!(),
                                        cli_utils::OutputTypes::Json => {
                                            let json = serde_json::to_string(&workspace).unwrap();

                                            match Path::new(&output_path).absolutize() {
                                                Ok(resolved_path) => {
                                                    if let Err(write_error) =
                                                        fs::write(resolved_path, json)
                                                    {
                                                        if cli_output_type
                                                            == cli_utils::CliOutputType::Json
                                                        {
                                                            let mut output =
                                                                cli_outputs::WRITE_FILE_ERROR
                                                                    .clone();

                                                            output.extra.push(
                                                                cli_outputs::CliOuputExtraData {
                                                                    key: "path".to_string(),
                                                                    value: format!(
                                                                        "{:?}",
                                                                        write_error
                                                                    ),
                                                                },
                                                            );

                                                            println!(
                                                                "{}",
                                                                serde_json::to_string(&output)
                                                                    .unwrap()
                                                            )
                                                        } else {
                                                            println!(
                                                                "\nFailed to write output. [{}{:?}{}]",
                                                                cli_utils::Colors::Red,
                                                                write_error,
                                                                cli_utils::Colors::Reset,
                                                            );
                                                        }
                                                    }
                                                }
                                                Err(path_error) => {
                                                    if cli_output_type
                                                        == cli_utils::CliOutputType::Json
                                                    {
                                                        let mut output =
                                                            cli_outputs::PATH_ERROR.clone();

                                                        output.extra.push(
                                                            cli_outputs::CliOuputExtraData {
                                                                key: "path".to_string(),
                                                                value: format!("{:?}", path_error),
                                                            },
                                                        );

                                                        println!(
                                                            "{}",
                                                            serde_json::to_string(&output).unwrap()
                                                        )
                                                    } else {
                                                        println!(
                                                                "\nFailed to find output path. [{}{:?}{}]",
                                                                cli_utils::Colors::Red,
                                                                path_error,
                                                                cli_utils::Colors::Reset,
                                                            );
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    if env::args().any(|x| x == "-rt" || x == "--render-tokenized")
                                    {
                                        let json = serde_json::to_string(&pager.pages).unwrap();
                                        let output_file_name = Path::new(main_path)
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned();
                                        let output_file =
                                            format!("{}_tokenized.json", output_file_name);
                                        match fs::write(format!("./{}", output_file), json) {
                                            Ok(_) => {
                                                println!(
                                                    "\nTokenized output successfully wrote to {}",
                                                    output_file
                                                );
                                            }
                                            Err(e) => {
                                                println!("\nFailed to write to file {}", e);
                                            }
                                        }
                                    }

                                    if env::args().any(|x| x == "-rp" || x == "--render-parsed") {
                                        let json =
                                            serde_json::to_string(&parser.processed_pages).unwrap();
                                        let output_file_name = Path::new(main_path)
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned();
                                        let output_file =
                                            format!("{}_parsed.json", output_file_name);
                                        match fs::write(format!("./{}", output_file), json) {
                                            Ok(_) => {
                                                println!(
                                                    "\nParsed output successfully wrote to {}",
                                                    output_file
                                                );
                                            }
                                            Err(e) => {
                                                println!("Failed to write to file {}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    Err(err) => {
                        println!("Unable to read file ~{} [{}]", main_path.clone(), err);
                        std::process::exit(1);
                    }
                },
                None => {
                    if env::args().any(|x| x == "-ec" || x == "--eval-code") {
                        println!("Evaluating code is not yet supported");
                    } else {
                        println!("No file present\n-h for help");
                    }
                    std::process::exit(1);
                }
            }
        }
    }
}
