use ellie_core::definite;
use ellie_engine::cli_utils;
use ellie_tokenizer::tokenizer::{self, Pager, ResolvedImport, Tokenizer};
use fs::File;
use path_absolutize::Absolutize;
use std::collections::hash_map::DefaultHasher;
use std::env;
use std::hash::{Hash, Hasher};
use std::path::Path;
use std::sync::{Mutex, PoisonError};
use std::{fs, io::Read};

fn main() {
    println!("{}]0;{}{}", '\u{001b}', "Ellie", '\u{007}');
    if env::args().any(|x| x == "-rstd" || x == "--rebuild-std") {
        println!(
            "{}[Success]{} Rebuilding std library complete",
            ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Green),
            ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset)
        );
        std::process::exit(0);
    }
    if env::args().any(|x| x == "-v" || x == "--version" || x == "-dv" || x == "--detailed-version")
    {
        if env::args().any(|x| x == "-dv" || x == "--detailed-version") {
            println!(
                "Ellie v{} - Code: {}\n\nTokenizer Version: v{}\nParser Version: v{}\nRuntime Version: v{}\nCore version: v{}\nEllie Standard Types Version: v{}\nEllie Compatibility Hash: {:#04x}\n",
                ellie_engine::cli_constants::ELLIE_VERSION,
                ellie_engine::cli_constants::ELLIE_VERSION_NAME,
                ellie_engine::cli_constants::ELLIE_TOKENIZER_VERSION,
                ellie_engine::cli_constants::ELLIE_PARSER_VERSION,
                ellie_engine::cli_constants::ELLIE_RUNTIME_VERSION,
                ellie_engine::cli_constants::ELLIE_CORE_VERSION,
                ellie_engine::cli_constants::ELLIE_STD_VERSION,
                ellie_engine::cli_constants::ELLIE_COMPATIBILITY_HASH,
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

            //let map_errors_arg = env::args().any(|x| x == "--map-errors");
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

                                println!(
                                    "Filepath: {:#?},{:#?},{:#?}",
                                    file.clone(),
                                    path,
                                    file_name
                                );
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
                            },
                        );

                        #[derive(Debug, Clone)]
                        struct NPage {
                            pub hash: u64,
                            pub path: String,
                            pub dependents: Vec<u64>,
                            pub dependencies: Vec<u64>,
                        }

                        match pager.run() {
                            Err(e) => panic!("Failed to tokenize: {:#?}", e),
                            Ok(_) => println!(
                                "Tokenize succes: \n{:#?}",
                                pager
                                    .pages
                                    .into_iter()
                                    .map(|x| {
                                        NPage {
                                            hash: x.hash,
                                            path: x.path,
                                            dependents: x.dependents,
                                            dependencies: x.dependencies,
                                        }
                                    })
                                    .collect::<Vec<_>>()
                            ),
                        }
                    }
                    Err(err) => {
                        println!("Unable to read file ~{} [{}]", main_path.clone(), err)
                    }
                },
                None => {
                    println!("No file present\n-h for help");
                    std::process::exit(1);
                }
            }
        }
    }
}
