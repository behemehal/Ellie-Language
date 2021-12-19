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
        println!("Usage: ellie [options] [file path | code]");
        println!("Options:");
        println!("\t--version                    || -v   : Show Version");
        println!("\t--help                       || -h   : Show Help");
        println!("\t--render-tokenized           || -rt  : Render tokenized code");
        println!("\t--render-parsed              || -rp  : Render parsed code");
        println!("\t--compile-workspace          || -cw  : Compile workspace for running later");
        println!("\t--exclude-std                || -es  : Exclude standard types from code");
        println!("\t--eval-code                  || -ec  : Evaluate code from parameters");
    } else {
        let args = env::args()
            .collect::<Vec<String>>()
            .drain(1..)
            .collect::<Vec<String>>();

        if args.is_empty() {
            println!("No file present\n-h for help");
        } else {
            let file_args = args
                .into_iter()
                .filter(|x| x.contains('.'))
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
                                        code: String::new(),
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
                                                        code: ext,
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
                            Err(e) => {
                                cli_utils::print_errors(&e, |path| {
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
                            Ok(_) => {
                                let mut parser = parser::Parser::new(pager.pages.clone(), None);

                                let std_pages: Vec<parser::ProcessedPage> = serde_json::from_str(
                                    ellie_core::builded_libraries::ELLIE_STANDARD_LIBRARY,
                                )
                                .unwrap();

                                if !env::args().any(|x| x == "-es" || x == "--exclude-std") {
                                    parser.import_processed_module(std_pages);
                                }

                                parser.parse();

                                if !parser.informations.has_no_warnings() {
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

                                if !parser.informations.has_no_errors() {
                                    cli_utils::print_errors(&parser.informations.errors, |path| {
                                        match cli_utils::read_file(&path) {
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
                                        }
                                    });
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
                                } else {
                                    println!(
                                        "\nCompiling {}succeeded{} with {}{} warnings{}.",
                                        cli_utils::Colors::Green,
                                        cli_utils::Colors::Reset,
                                        cli_utils::Colors::Yellow,
                                        parser.informations.warnings.len(),
                                        cli_utils::Colors::Reset,
                                    );

                                    if env::args()
                                        .any(|x| x == "-cp" || x == "--compile-workspace")
                                    {
                                        let bytes = bincode::serialize(&parser.processed_pages).unwrap();
                                        let output_file_name = Path::new(main_path)
                                            .file_name()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                            .to_owned();
                                        let output_file =
                                            format!("{}_workspace.bin", output_file_name);
                                        match fs::write(format!("./{}", output_file), bytes) {
                                            Ok(_) => {
                                                println!(
                                                    "\nCompiled workspace output successfully wrote to {}",
                                                    output_file
                                                );
                                            }
                                            Err(e) => {
                                                println!("Failed to write to file {}", e);
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
                        println!("Unable to read file ~{} [{}]", main_path.clone(), err)
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
