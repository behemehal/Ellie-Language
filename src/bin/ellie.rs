use ellie_parser::parser;
use fs::File;
use std::env;
use std::{fs, io::Read};

fn main() {
    if env::args().any(|x| x == "-v" || x == "--version") {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        println!("Ellie v{} - Code: Millennium", VERSION);
    } else if env::args().any(|x| x == "-h" || x == "--help") {
        println!("Usage: ellie [options] [file path]");
        println!("Options:");
        println!("\t--version                    || -v   : Show Version");
        println!("\t--help                       || -h   : Show Help");
        println!("\t--debug                      || -d   : Show debug headers");
        println!(
            "\t--experimental-error-listing || -xe  : Use experimental error listing in terminal"
        );
        println!("\t--to-raw                     || -tr  : Compiles ellie to ellie raw");
        println!("\t--raw-compile                || -rw  : Compiles as ellie raw");
        println!("\t--show-errors                || -se  : Linter code for errors");
        println!("\t--json-errors                || -je  : Linter code for errors as json");
        println!("\t--parser-ws                  || -pw  : Visualize parsing process");
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
            let debug_arg = env::args().any(|x| x == "--debug" || x == "-d");

            //let map_errors_arg = env::args().any(|x| x == "--map-errors");
            let file_arg_check = file_args.first();
            if file_arg_check != None {
                let file_arg = file_arg_check.unwrap();
                //let file = Path::new(&file_arg.clone());
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
                        let core_resolver = |e| {
                            println!(
                                "{}[ParserInfo]{}: Import Request '{}{}{}'",
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Cyan
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Yellow
                                ),
                                e,
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                            );
                            if e == "ellie"
                                || e == "string"
                                || e == "void"
                                || e == "int"
                                || e == "char"
                            {
                                if let Some(e) = ellie_lang::cli_utils::system_module_resolver(e) {
                                    parser::ResolvedImport {
                                        found: true,
                                        file_content: e,
                                    }
                                } else {
                                    parser::ResolvedImport::default()
                                }
                            } else {
                                parser::ResolvedImport::default()
                            }
                        };

                        let parser = parser::Parser::new(
                            code.clone(),
                            core_resolver,
                            ellie_core::defs::ParserOptions {
                                functions: true,
                                break_on_error: false,
                                loops: true,
                                conditions: true,
                                classes: true,
                                dynamics: true,
                                global_variables: true,
                                line_ending: if env::consts::OS == "windows" {
                                    "\\r\\n".to_string()
                                } else {
                                    "\\n".to_string()
                                },
                                collectives: true,
                                variables: true,
                                constants: true,
                                parser_type: ellie_core::defs::ParserType::RawParser,
                                allow_import: true,
                            },
                        );
                        let mapped = parser.map();

                        if !mapped.syntax_errors.is_empty() {
                            if env::args()
                                .any(|x| x == "-xe" || x == "--experimental-error-listing")
                            {
                                for error in
                                    &ellie_lang::cli_utils::zip_errors(mapped.syntax_errors)
                                {
                                    if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                        println!(
                                            "+\n{:#?}\n",
                                            serde_json::to_string(error).unwrap()
                                        );
                                    } else {
                                        if error.pos.range_start.0 != error.pos.range_end.0 {
                                            std::println!(
                                                "{}[Experimental]{}: Multiline error listing",
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Magenta
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                            );
                                            println!(
                                                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                                if debug_arg {
                                                    format!(
                                                        "{}({}) {}[{}]{} ",
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Magenta
                                                        ),
                                                        error.scope,
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Yellow
                                                        ),
                                                        error.debug_message,
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Reset
                                                        )
                                                    )
                                                } else {
                                                    "".to_string()
                                                },
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Red
                                                ),
                                                &error.code,
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Cyan
                                                ),
                                                error.title,
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                                error.builded_message.builded
                                            );
                                            println!(
                                                "{}:[{} ~ {}]:?",
                                                file_arg.clone(),
                                                error.pos.range_start.0 + 1,
                                                error.pos.range_end.0 + 1
                                            );
                                            let mut pos = vec![error.pos.range_start];

                                            for _ in 1..error.pos.range_end.0 {
                                                pos.push(error.pos.range_end)
                                            }

                                            println!(
                                                "{}",
                                                ellie_lang::cli_utils::get_lines(
                                                    code.clone(),
                                                    error.pos
                                                )
                                            )
                                        } else {
                                            println!(
                                                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                                if debug_arg {
                                                    format!(
                                                        "{}({}) {}[{}]{} ",
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Magenta
                                                        ),
                                                        error.scope,
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Yellow
                                                        ),
                                                        error.debug_message,
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Reset
                                                        )
                                                    )
                                                } else {
                                                    "".to_string()
                                                },
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Red
                                                ),
                                                &error.code,
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Cyan
                                                ),
                                                error.title,
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                                error.builded_message.builded
                                            );
                                            println!(
                                                "{}:{}:{}",
                                                file_arg.clone(),
                                                error.pos.range_start.0 + 1,
                                                error.pos.range_start.1 + 1
                                            );
                                            let line: Vec<&str> = code
                                                .split(if env::consts::OS == "windows" {
                                                    "\\r\\n"
                                                } else {
                                                    "\\n"
                                                })
                                                .collect();
                                            println!(
                                                "{}\n{}{}{}",
                                                ellie_lang::cli_utils::get_line(
                                                    code.clone(),
                                                    error.pos.range_start.0 as usize
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Red
                                                ),
                                                ellie_lang::cli_utils::arrow(
                                                    (error.pos.range_start.1 + 1) as usize,
                                                    if error.pos.range_end.1
                                                        > error.pos.range_start.1
                                                    {
                                                        ((error.pos.range_end.1)
                                                            - (error.pos.range_start.1))
                                                            as usize
                                                    } else {
                                                        error.pos.range_start.1 as usize
                                                            - (line[error.pos.range_start.1]).len()
                                                    }
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                )
                                            );
                                        }
                                    }
                                }
                            } else {
                                for error in &mapped.syntax_errors {
                                    if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                        println!(
                                            "+\n{:#?}\n",
                                            serde_json::to_string(error).unwrap()
                                        );
                                    } else {
                                        println!(
                                            "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                            if debug_arg {
                                                format!(
                                                    "{}[{}]{} ",
                                                    ellie_lang::terminal_colors::get_color(
                                                        ellie_lang::terminal_colors::Colors::Yellow
                                                    ),
                                                    error.debug_message,
                                                    ellie_lang::terminal_colors::get_color(
                                                        ellie_lang::terminal_colors::Colors::Reset
                                                    )
                                                )
                                            } else {
                                                "".to_string()
                                            },
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Red
                                            ),
                                            &error.code,
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Cyan
                                            ),
                                            error.title,
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                            error.builded_message.builded
                                        );
                                        println!(
                                            "{}:{}:{}",
                                            file_arg.clone(),
                                            error.pos.range_start.0 + 1,
                                            error.pos.range_start.1 + 1
                                        );
                                        println!(
                                            "{}\n{}{}{}",
                                            ellie_lang::cli_utils::get_line(
                                                code.clone(),
                                                error.pos.range_start.0 as usize
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Red
                                            ),
                                            ellie_lang::cli_utils::arrow(
                                                (error.pos.range_start.1 + 1) as usize,
                                                if error.pos.range_end.1 > error.pos.range_start.1 {
                                                    ((error.pos.range_end.1)
                                                        - (error.pos.range_start.1))
                                                        as usize
                                                } else {
                                                    std::println!(
                                                        "{}[ParserWarning]{}: Multiline error show is not supported, you may want to use --experimental-error-listing : {}https://github.com/behemehal/Ellie-Language/issues/17{}",
                                                        ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Red),
                                                        ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Reset),
                                                        ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Cyan),
                                                        ellie_lang::terminal_colors::get_color(ellie_lang::terminal_colors::Colors::Reset),
                                                    );
                                                    error.pos.range_start.1 as usize
                                                }
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            )
                                        );
                                    }
                                }
                            }
                            std::process::exit(1);
                        } else if env::args().any(|x| x == "-rw" || x == "--raw-compile") {
                            println!("Pre-compiled raw generation not supported yet {:#?}", code);
                        } else if !env::args().any(|x| x == "-se" || x == "--show-errors") {
                            print!("Collected: {:#?}", mapped);
                            std::process::exit(0);
                        }
                    }
                }
            } else {
                println!("No file present\n-h for help");
                std::process::exit(1);
            }
        }
    }
}
