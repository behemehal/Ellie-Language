use ellie_core::definite;
use ellie_tokenizer::tokenizer::{self, ResolvedImport, Tokenizer};
use fs::File;
use std::env;
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
            let file_arg_check = file_args.first();
            if file_arg_check != None {
                let file_arg = file_arg_check.unwrap();
                let mut file_content = Vec::new();
                let file_read = File::open(&file_arg.clone());
                if file_read.is_err() {
                    println!("File not found ~{}", &file_arg.clone());
                    std::process::exit(1);
                } else if let Ok(mut file) = file_read {
                    file.read_to_end(&mut file_content).expect("Unable to read");
                    let code_string = String::from_utf8(file_content);
                    if code_string.is_err() {
                        println!("Unable to read file ~{}", file_arg.clone())
                    } else if let Ok(code) = code_string {
                        println!("Code {:#?}", code);

                        let mut tokenizer = Tokenizer::new(code, |x| {
                            println!("Import: {:#?}", x);
                            ResolvedImport::default()
                        });
                        let collected = tokenizer.tokenize();

                        if let Ok(items) = collected {
                            println!("{:#?}", items);
                        } else if let Err(errors) = collected {
                            println!("{:#?}", errors);
                        }
                    }
                }
            }
        }
    }
}
