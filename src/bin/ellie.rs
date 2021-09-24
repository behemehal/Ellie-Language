use ellie_core::definite;
use ellie_parser::parser;
use fs::File;
use std::env;
use std::path::Path;
use std::{fs, io::Read};

fn main() {
    println!("{}]0;{}{}", '\u{001b}', "Ellie", '\u{007}');
    if env::args().any(|x| x == "-v" || x == "--version" || x == "-dv" || x == "--detailed-version")
    {
        if env::args().any(|x| x == "-dv" || x == "--detailed-version") {
            println!("Ellie v{} - Code: {}\n\nParser Version: {}\nRuntime Version: {}\nEllie RawByteCode Version: {}\n", ellie_engine::cli_constants::ELLIE_VERSION, ellie_engine::cli_constants::ELLIE_VERSION_NAME, ellie_engine::cli_constants::ELLIE_PARSER_VERSION, ellie_engine::cli_constants::ELLIE_RUNTIME_VERSION, ellie_engine::cli_constants::ELLIE_BYTE_CODE_VERSION);
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
        println!("\t--debug                      || -d   : Show debug headers");
        println!("\t--to-json                    || -tj  : Compiles ellie to ellie json");
        println!("\t--to-byte-code               || -tb  : Compiles ellie to byte code");
        println!("\t--show-errors                || -se  : Linter code for errors");
        println!("\t--json-errors                || -je  : Linter code for errors as json");
        println!("\t--eval-code                  || -ec  : Evaluate code from parameters");
        println!("\t--parser-ws                  || -pw  : Visualize parsing process");
        println!("\t--parser-messages            || -pm  : Show parser messages");
        println!("Disabled Options:");
        println!("\t--disable-webm               || -dw  : Disable web modules ");
        println!("\t--disable-gist               || -dg  : Disable gist modules ");
        if File::open("./DEBUG_HEADERS.eidbg").is_ok() {
            println!("Development Options:");
            println!("\t-i                                   : Ignore errors");
            println!("\t-e                                   : Show non-definite items");
            println!("\t-fi                                  : Filter out imports");
            println!("\t-dstd                                : Don't import std");
        }
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
            let eval_code = env::args().any(|x| x == "--eval-code" || x == "-ec");
            let visualize_code = env::args().any(|x| x == "--parser-ws" || x == "-pw");
            let ignore_errors = env::args().any(|x| x == "-i");
            let non_definite = env::args().any(|x| x == "-e");

            if ignore_errors && !File::open("./DEBUG_HEADERS.eidbg").is_ok() {
                std::println!(
                    "{}Cannot ignore errors, you are not in development directory{}",
                    ellie_engine::terminal_colors::get_color(
                        ellie_engine::terminal_colors::Colors::Red
                    ),
                    ellie_engine::terminal_colors::get_color(
                        ellie_engine::terminal_colors::Colors::Reset
                    ),
                );
                std::process::exit(2);
            } else if non_definite && !File::open("./DEBUG_HEADERS.eidbg").is_ok() {
                std::println!(
                    "{}Cannot show non-definite items, you are not in development directory{}",
                    ellie_engine::terminal_colors::get_color(
                        ellie_engine::terminal_colors::Colors::Red
                    ),
                    ellie_engine::terminal_colors::get_color(
                        ellie_engine::terminal_colors::Colors::Reset
                    ),
                );
                std::process::exit(2);
            }

            //let map_errors_arg = env::args().any(|x| x == "--map-errors");
            let file_arg_check = file_args.first();
            if file_arg_check != None && !eval_code {
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
                        if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                            println!("{}[Warning]{}: Parser messages arg applied to runtime, this will reduce performance",
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Yellow
                                ),
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Reset
                                ),
                            );
                        }

                        if visualize_code {
                            println!(
                                "{}[Not Ready]{}",
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Red
                                ),
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Reset
                                ),
                            );
                            std::process::exit(0);
                        } else {
                            let mut parser = parser::Parser::new(
                                code.clone(),
                                ellie_engine::cli_utils::resolve_import,
                                |e| {
                                    if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                                        println!(
                                            "{}[{}]{} {}[Parser Message]{}: {}[{:?}]{} - {}{}{}",
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Yellow
                                            ),
                                            e.id,
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Magenta
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Red
                                            ),
                                            e.message_type,
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Yellow
                                            ),
                                            e.message_data,
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                        )
                                    }
                                },
                                ellie_core::defs::ParserOptions {
                                    path: file_arg.to_string(),
                                    functions: true,
                                    break_on_error: false,
                                    loops: true,
                                    conditions: true,
                                    classes: true,
                                    enums: true,
                                    dynamics: true,
                                    import_std: !env::args().any(|x| x == "-dstd"),
                                    global_variables: true,
                                    getters: true,
                                    setters: true,
                                    line_ending: if env::consts::OS == "windows" {
                                        "\\r\\n".to_owned()
                                    } else {
                                        "\\n".to_owned()
                                    },
                                    collectives: true,
                                    variables: true,
                                    constants: true,
                                    parser_type: ellie_core::defs::ParserType::RawParser,
                                    allow_import: true,
                                },
                            );
                            parser.scope.scope_name = "ellie_core".to_owned();
                            let mapped = parser.map();

                            if !mapped.syntax_errors.is_empty() {
                                if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                    print!("*");
                                }
                                for error in &ellie_engine::cli_utils::zip_errors(
                                    mapped.syntax_errors.clone(),
                                ) {
                                    if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                        println!(
                                            "+\n{:?}\n",
                                            serde_json::to_string(error).unwrap()
                                        );
                                    } else {
                                        match ellie_engine::cli_utils::read_file(
                                            &error.path.clone(),
                                        ) {
                                            Ok(targeted_error_file) => {
                                                if error.pos.range_start.0 != error.pos.range_end.0
                                                {
                                                    std::println!(
                                                            "{}[Experimental]{}: Multi line error listing",
                                                            ellie_engine::terminal_colors::get_color(
                                                                ellie_engine::terminal_colors::Colors::Magenta
                                                            ),
                                                            ellie_engine::terminal_colors::get_color(
                                                                ellie_engine::terminal_colors::Colors::Reset
                                                            ),
                                                        );
                                                    println!(
                                                        "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                                        if debug_arg {
                                                            format!(
                                                                "{}({}) {}[{}]{} ",
                                                                ellie_engine::terminal_colors::get_color(
                                                                    ellie_engine::terminal_colors::Colors::Magenta
                                                                ),
                                                                error.scope,
                                                                ellie_engine::terminal_colors::get_color(
                                                                    ellie_engine::terminal_colors::Colors::Yellow
                                                                ),
                                                                error.debug_message,
                                                                ellie_engine::terminal_colors::get_color(
                                                                    ellie_engine::terminal_colors::Colors::Reset
                                                                )
                                                            )
                                                        } else {
                                                            "".to_owned()
                                                        },
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Red
                                                        ),
                                                        &error.code,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        ),
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Cyan
                                                        ),
                                                        error.title,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
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
                                                        ellie_engine::cli_utils::get_lines(
                                                            targeted_error_file.clone(),
                                                            error.pos
                                                        )
                                                    )
                                                } else {
                                                    println!(
                                                        "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                                        if debug_arg {
                                                            format!(
                                                                    "{}({}) {}[{}]{} ",
                                                                    ellie_engine::terminal_colors::get_color(
                                                                        ellie_engine::terminal_colors::Colors::Magenta
                                                                    ),
                                                                    error.scope,
                                                                    ellie_engine::terminal_colors::get_color(
                                                                        ellie_engine::terminal_colors::Colors::Yellow
                                                                    ),
                                                                    error.debug_message,
                                                                    ellie_engine::terminal_colors::get_color(
                                                                        ellie_engine::terminal_colors::Colors::Reset
                                                                    )
                                                                )
                                                        } else {
                                                            "".to_owned()
                                                        },
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Red
                                                        ),
                                                        &error.code,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        ),
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Cyan
                                                        ),
                                                        error.title,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        ),
                                                        error.builded_message.builded
                                                    );
                                                    println!(
                                                        "{}:{}:{}",
                                                        error.path,
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
                                                        ellie_engine::cli_utils::get_line(
                                                            targeted_error_file.clone(),
                                                            error.pos.range_start.0 as usize
                                                        ),
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Red
                                                        ),
                                                        ellie_engine::cli_utils::arrow(
                                                            (error.pos.range_start.1 + 1) as usize,
                                                            if error.pos.range_end.1
                                                                > error.pos.range_start.1
                                                            {
                                                                ((error.pos.range_end.1)
                                                                    - (error.pos.range_start.1))
                                                                    as usize
                                                            } else {
                                                                if line.len() - 1
                                                                    >= error.pos.range_start.1
                                                                    && error.pos.range_start.1
                                                                        > (line
                                                                            [error.pos.range_start.1])
                                                                            .len()
                                                                {
                                                                    error.pos.range_start.1 as usize
                                                                        - (line
                                                                            [error.pos.range_start.1])
                                                                            .len()
                                                                } else {
                                                                    error.pos.range_end.1
                                                                }
                                                            }
                                                        ),
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        )
                                                    );
                                                }
                                            }
                                            Err(r) => {
                                                std::println!(
                                                        "{}[CLI ERROR]{}: Cannot read targeted file ~ {} {:#?}",
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Magenta
                                                        ),
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        ),
                                                        error.path,
                                                        r
                                                    );
                                            }
                                        }
                                    }
                                }
                            }

                            if !env::args().any(|x| x == "-se" || x == "--show-errors")
                                && (mapped.syntax_errors.is_empty() || ignore_errors)
                            {
                                let collected_items = mapped
                                    .parsed
                                    .items
                                    .clone()
                                    .into_iter()
                                    .filter(|x| {
                                        if env::args().any(|x| x == "-fi") {
                                            !matches!(x, parser::Collecting::ImportItem(_))
                                        } else {
                                            true
                                        }
                                    })
                                    .map(|x| x.to_definite());
                                let collected_definite_items: Vec<definite::items::Collecting> =
                                    collected_items.collect();

                                if env::args().any(|x| x == "-tb" || x == "--to-byte-code") {
                                    let mut ellie_byte_conv =
                                        ellie_byte_code::converter::Converter::new(
                                            "ellie_core".to_owned(),
                                            ellie_byte_code::converter::ConverterOptions {
                                                apply_comments: true,
                                                lib_name: file_arg.to_string(),
                                            },
                                            false,
                                        );
                                    ellie_byte_conv.convert(mapped.parsed.to_definite());
                                    println!("-----RAW---\n{:#?}", ellie_byte_conv.clone());
                                    let path_to_w = format!(
                                        "./raw_{}.eiw",
                                        Path::new(&file_arg.to_string())
                                            .extension()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                    );
                                    if let Err(e) =
                                        fs::write(path_to_w.clone(), ellie_byte_conv.to_string())
                                    {
                                        println!(
                                                "{}[WriteError]{}: Cannot write raw file {}'{}'{}, {}{}{}",
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Yellow),
                                                path_to_w,
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                e.to_string(),
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
                                            )
                                    }
                                } else if env::args().any(|x| x == "-tj" || x == "--to-json") {
                                    print!("/");
                                    for item in collected_definite_items {
                                        print!("-\n{:#?}\n", serde_json::to_string(&item).unwrap());
                                    }
                                    std::process::exit(0);
                                } else {
                                    if env::args().any(|x| x == "-e") {
                                        print!(
                                            "Collected non-definite items: {:#?}",
                                            mapped.parsed.items
                                        );
                                    } else {
                                        print!(
                                            "Collected: definite items {:#?}",
                                            collected_definite_items
                                        );
                                    }
                                    std::process::exit(0);
                                }
                            }

                            if !mapped.syntax_errors.is_empty() {
                                std::process::exit(1);
                            }
                        }
                    }
                }
            } else {
                if eval_code {
                    let code_pos = env::args()
                        .position(|x| x == "--eval-code" || x == "-ec")
                        .unwrap()
                        + 1;
                    if env::args().len() > code_pos {
                        let code_vec: Vec<String> = env::args().skip(code_pos).collect(); //.nth(code_pos).unwrap();
                        let code = ellie_engine::cli_utils::clean_up_escape(code_vec.join(" "));

                        if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                            println!("{}[Warning]{}: Parser messages arg applied to runtime, this will reduce performance",
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Yellow
                                ),
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Reset
                                ),
                            );
                        }

                        if visualize_code {
                            println!(
                                "{}[Not Ready]{}",
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Red
                                ),
                                ellie_engine::terminal_colors::get_color(
                                    ellie_engine::terminal_colors::Colors::Reset
                                ),
                            );
                            std::process::exit(0);
                        } else {
                            let parser = parser::Parser::new(
                                code.clone(),
                                ellie_engine::cli_utils::resolve_import,
                                |e| {
                                    if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                                        println!(
                                            "{}[{}]{} {}[Parser Message]{}: {}[{:?}]{} - {}{}{}",
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Yellow
                                            ),
                                            e.id,
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Magenta
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Red
                                            ),
                                            e.message_type,
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Yellow
                                            ),
                                            e.message_data,
                                            ellie_engine::terminal_colors::get_color(
                                                ellie_engine::terminal_colors::Colors::Reset
                                            ),
                                        )
                                    }
                                },
                                ellie_core::defs::ParserOptions {
                                    path: "<eval>".to_owned(),
                                    functions: true,
                                    break_on_error: false,
                                    import_std: !env::args().any(|x| x == "-dstd"),
                                    loops: true,
                                    conditions: true,
                                    classes: true,
                                    enums: true,
                                    dynamics: true,
                                    global_variables: true,
                                    getters: true,
                                    setters: true,
                                    line_ending: if env::consts::OS == "windows" {
                                        "\\r\\n".to_owned()
                                    } else {
                                        "\\n".to_owned()
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
                                if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                    print!("*");
                                }
                                for error in &ellie_engine::cli_utils::zip_errors(
                                    mapped.syntax_errors.clone(),
                                ) {
                                    if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                        println!(
                                            "+\n{:?}\n",
                                            serde_json::to_string(error).unwrap()
                                        );
                                    } else {
                                        if error.pos.range_start.0 != error.pos.range_end.0 {
                                            println!(
                                                "{}[Experimental]{}: Multi line error listing",
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Magenta
                                                ),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Reset
                                                ),
                                            );
                                            println!(
                                                "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                                if debug_arg {
                                                    format!(
                                                        "{}({}) {}[{}]{} ",
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Magenta
                                                        ),
                                                        error.scope,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Yellow
                                                        ),
                                                        error.debug_message,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        )
                                                    )
                                                } else {
                                                    "".to_owned()
                                                },
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                &error.code,
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Reset
                                                ),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Cyan
                                                ),
                                                error.title,
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Reset
                                                ),
                                                error.builded_message.builded
                                            );
                                            println!(
                                                "{}:[{} ~ {}]:?",
                                                "eval",
                                                error.pos.range_start.0 + 1,
                                                error.pos.range_end.0 + 1
                                            );
                                            let mut pos = vec![error.pos.range_start];

                                            for _ in 1..error.pos.range_end.0 {
                                                pos.push(error.pos.range_end)
                                            }

                                            println!(
                                                "{}",
                                                ellie_engine::cli_utils::get_lines(
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
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Magenta
                                                        ),
                                                        error.scope,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Yellow
                                                        ),
                                                        error.debug_message,
                                                        ellie_engine::terminal_colors::get_color(
                                                            ellie_engine::terminal_colors::Colors::Reset
                                                        )
                                                    )
                                                } else {
                                                    "".to_owned()
                                                },
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                &error.code,
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Reset
                                                ),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Cyan
                                                ),
                                                error.title,
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Reset
                                                ),
                                                error.builded_message.builded
                                            );
                                            println!(
                                                "{}:{}:{}",
                                                "eval",
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
                                                ellie_engine::cli_utils::get_line(
                                                    code.clone(),
                                                    error.pos.range_start.0 as usize
                                                ),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                ellie_engine::cli_utils::arrow(
                                                    (error.pos.range_start.1 + 1) as usize,
                                                    if error.pos.range_end.1
                                                        > error.pos.range_start.1
                                                    {
                                                        ((error.pos.range_end.1)
                                                            - (error.pos.range_start.1))
                                                            as usize
                                                    } else {
                                                        if line.len() - 1 >= error.pos.range_start.1
                                                            && error.pos.range_start.1
                                                                > (line[error.pos.range_start.1])
                                                                    .len()
                                                        {
                                                            error.pos.range_start.1 as usize
                                                                - (line[error.pos.range_start.1])
                                                                    .len()
                                                        } else {
                                                            error.pos.range_end.1
                                                        }
                                                    }
                                                ),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Reset
                                                )
                                            );
                                        }
                                    }
                                }
                                std::process::exit(1);
                            }

                            if !env::args().any(|x| x == "-se" || x == "--show-errors")
                                && (mapped.syntax_errors.is_empty() || ignore_errors)
                            {
                                let collected_items = mapped
                                    .parsed
                                    .items
                                    .clone()
                                    .into_iter()
                                    .filter(|x| {
                                        if env::args().any(|x| x == "-fi") {
                                            !matches!(x, parser::Collecting::ImportItem(_))
                                        } else {
                                            true
                                        }
                                    })
                                    .map(|x| x.to_definite());
                                let collected_definite_items: Vec<definite::items::Collecting> =
                                    collected_items.collect();

                                if env::args().any(|x| x == "-tb" || x == "--to-byte-code") {
                                    let mut raw_conv = ellie_byte_code::converter::Converter::new(
                                        "ellie_core".to_owned(),
                                        ellie_byte_code::converter::ConverterOptions {
                                            apply_comments: true,
                                            lib_name: "<eval>".to_owned(),
                                        },
                                        false,
                                    );
                                    raw_conv.convert(mapped.parsed.to_definite());
                                    println!("-----RAW---\n{:#?}", raw_conv.clone());
                                    let path_to_w = format!(
                                        "./raw_{}.eiw",
                                        Path::new(&"eval".to_owned())
                                            .extension()
                                            .unwrap()
                                            .to_str()
                                            .unwrap()
                                    );
                                    if let Err(e) =
                                        fs::write(path_to_w.clone(), raw_conv.to_string())
                                    {
                                        println!(
                                                "{}[WriteError]{}: Cannot write byte code file {}'{}'{}, {}{}{}",
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Yellow),
                                                path_to_w,
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
                                                ellie_engine::terminal_colors::get_color(
                                                    ellie_engine::terminal_colors::Colors::Red
                                                ),
                                                e.to_string(),
                                                ellie_engine::terminal_colors::get_color(ellie_engine::terminal_colors::Colors::Reset),
                                            )
                                    }
                                } else if env::args().any(|x| x == "-tj" || x == "--to-json") {
                                    print!("/");
                                    for item in collected_definite_items {
                                        print!("-\n{:#?}\n", serde_json::to_string(&item).unwrap());
                                    }
                                    std::process::exit(0);
                                } else {
                                    if env::args().any(|x| x == "-e") {
                                        print!(
                                            "Collected non-definite items: {:#?}",
                                            mapped.parsed.items
                                        );
                                    } else {
                                        print!(
                                            "Collected: definite items {:#?}",
                                            collected_definite_items
                                        );
                                    }
                                    std::process::exit(0);
                                }
                            }

                            if !mapped.syntax_errors.is_empty() {
                                std::process::exit(1);
                            }
                        }
                    } else {
                        println!("Code not found");
                        std::process::exit(1);
                    }
                } else {
                    println!("No file present\n-h for help");
                    std::process::exit(1);
                }
            }
        }
    }
}
