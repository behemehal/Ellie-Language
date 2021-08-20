use ellie_parser::parser;
use fs::File;
use path_absolutize::Absolutize;
use std::env;
use std::path::Path;
use std::thread;
use std::{fs, io::Read};
use websocket::{sync::Server, Message, OwnedMessage};
fn main() {
    ellie_lang::terminal_colors::title_set("Ellie");
    if env::args().any(|x| x == "-v" || x == "--version" || x == "-dv" || x == "--detailed-version")
    {
        if env::args().any(|x| x == "-dv" || x == "--detailed-version") {
            println!("Ellie v{} - Code: {}\n\nParser Version: {}\nRuntime Version: {}\nEllie RawByteCode Version: {}\n", ellie_lang::cli_constants::ELLIE_VERSION, ellie_lang::cli_constants::ELLIE_VERSION_NAME, ellie_lang::cli_constants::ELLIE_PARSER_VERSION, ellie_lang::cli_constants::ELLIE_RUNTIME_VERSION, ellie_lang::cli_constants::ELLIE_RAW_VERSION);
        } else {
            println!(
                "Ellie v{} - Code: {}",
                ellie_lang::cli_constants::ELLIE_VERSION,
                ellie_lang::cli_constants::ELLIE_VERSION_NAME
            );
        }
    } else if env::args().any(|x| x == "-h" || x == "--help") {
        println!("Usage: ellie [options] [file path | code]");
        println!("Options:");
        println!("\t--version                    || -v   : Show Version");
        println!("\t--help                       || -h   : Show Help");
        println!("\t--debug                      || -d   : Show debug headers");
        println!("\t--to-raw                     || -tr  : Compiles ellie to ellie raw");
        println!("\t--show-errors                || -se  : Linter code for errors");
        println!("\t--json-errors                || -je  : Linter code for errors as json");
        println!("\t--eval-code                  || -ec  : Evaluate code from parameters");
        println!("\t--parser-ws                  || -pw  : Visualize parsing process");
        println!("\t--parser-messages            || -pm  : Show parser messages");
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

            if ignore_errors && !File::open("./DEBUG_HEADERS.eidbg").is_ok() {
                std::println!(
                    "{}Cannot ignore errors, you are not in development directory{}",
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Red
                    ),
                    ellie_lang::terminal_colors::get_color(
                        ellie_lang::terminal_colors::Colors::Reset
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
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Yellow
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                            );
                        }

                        if visualize_code {
                            let port = if let Some(port_pos) =
                                env::args().position(|x| x == "--port" || x == "-p")
                            {
                                if env::args().len() > port_pos + 1 {
                                    let port_vec: Vec<String> =
                                        env::args().skip(port_pos + 1).collect(); //.nth(code_pos).unwrap();
                                    let port_string =
                                        ellie_lang::cli_utils::clean_up_escape(port_vec.join(" "));

                                    if let Ok(port) = port_string.clone().parse::<isize>() {
                                        port
                                    } else {
                                        println!("{}[Error]{}: Failed to get open port from parameters, supplied parameter ({}{}{}) is not a digit. -h to learn more",
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Red
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Red
                                            ),
                                            port_string,
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                        );
                                        -1
                                    }
                                } else {
                                    println!("{}[Error]{}: Failed to get open port from parameters, no data supplied. -h to learn more",
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Red
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                    );
                                    -1
                                }
                            } else {
                                9978
                            };

                            match Server::bind(format!("127.0.0.1:{}", port)) {
                                Ok(server) => {
                                    println!(
                                        "{}[Success]{}: Connect {}{}{} to visualize code exec",
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Green
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Cyan
                                        ),
                                        format!(
                                            "file://{}?s=127.0.0.1:{}",
                                            Path::new("../panel/panel.html")
                                                .absolutize()
                                                .unwrap()
                                                .to_str()
                                                .unwrap()
                                                .to_string(),
                                            port
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                    );
                                    for connection in server.filter_map(Result::ok) {
                                        thread::spawn(|| {
                                            let client = connection.accept().unwrap();
                                            println!(
                                                "{}[Connection]{}: Connection from {}{}{}, waiting for go signal",
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Cyan
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Yellow
                                                ),
                                                client.local_addr().unwrap(),
                                                ellie_lang::terminal_colors::get_color(
                                                    ellie_lang::terminal_colors::Colors::Reset
                                                ),
                                            );
                                            let (mut receiver, mut sender) =
                                                client.split().unwrap();
                                        });
                                    }
                                }
                                Err(e) => {
                                    println!("{}[Error: {}]{}: Failed to open port at {}{}{}, use --port or -p to alternate port.",
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Red
                                        ),
                                        e.to_string(),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Red
                                        ),
                                        port.to_string(),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                    );
                                }
                            }

                            println!("{}[Warning]{}: Visualizing code exec, waiting for a client to connect...",
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Yellow
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                            );
                            println!(
                                "{}[Connection]{}: Connection from {}{}{}, waiting for go signal",
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Cyan
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Yellow
                                ),
                                "192.168.1.1",
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                            );
                            println!(
                                "{}[Connected]{}: Parsing {}{}{}",
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Green
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Yellow
                                ),
                                file_arg.clone(),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                            );
                        } else {
                            let mut parser = parser::Parser::new(
                                code.clone(),
                                ellie_lang::cli_utils::resolve_import,
                                |e| {
                                    if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                                        println!(
                                            "{}[{}]{} {}[Parser Message]{}: {}[{:?}]{} - {}{}{}",
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Yellow
                                            ),
                                            e.id,
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Magenta
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Red
                                            ),
                                            e.message_type,
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
                                            ),
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Yellow
                                            ),
                                            e.message_data,
                                            ellie_lang::terminal_colors::get_color(
                                                ellie_lang::terminal_colors::Colors::Reset
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
                            parser.scope.scope_name = "file_arg".to_string();
                            let mapped = parser.map();

                            if !mapped.syntax_errors.is_empty() {
                                if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                    print!("*");
                                }
                                for error in
                                    &ellie_lang::cli_utils::zip_errors(mapped.syntax_errors.clone())
                                {
                                    if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                        println!(
                                            "+\n{:?}\n",
                                            serde_json::to_string(error).unwrap()
                                        );
                                    } else {
                                        match ellie_lang::cli_utils::read_file(&error.path.clone())
                                        {
                                            Ok(targeted_error_file) => {
                                                if error.pos.range_start.0 != error.pos.range_end.0
                                                {
                                                    std::println!(
                                                            "{}[Experimental]{}: Multi line error listing",
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
                                                        ellie_lang::cli_utils::get_line(
                                                            targeted_error_file.clone(),
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
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Reset
                                                        )
                                                    );
                                                }
                                            }
                                            Err(r) => {
                                                std::println!(
                                                        "{}[CLI ERROR]{}: Cannot read targeted file ~ {} {:#?}",
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Magenta
                                                        ),
                                                        ellie_lang::terminal_colors::get_color(
                                                            ellie_lang::terminal_colors::Colors::Reset
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
                                if env::args().any(|x| x == "-tr" || x == "--to-raw") {
                                    print!("/");
                                    for item in mapped.parsed.items {
                                        print!("-\n{:#?}\n", serde_json::to_string(&item).unwrap());
                                    }
                                    std::process::exit(0);
                                } else {
                                    print!(
                                        "Collected: {:#?}",
                                        mapped.parsed.items.into_iter().filter(|x| !matches!(
                                            x,
                                            ellie_parser::parser::Collecting::ImportItem(_)
                                        ))
                                    );
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
                        let code = ellie_lang::cli_utils::clean_up_escape(code_vec.join(" "));

                        if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                            println!("{}[Warning]{}: Parser messages arg applied to runtime, this will reduce performance",
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Yellow
                                ),
                                ellie_lang::terminal_colors::get_color(
                                    ellie_lang::terminal_colors::Colors::Reset
                                ),
                            );
                        }

                        let parser = parser::Parser::new(
                            code.clone(),
                            ellie_lang::cli_utils::resolve_import,
                            |e| {
                                if env::args().any(|x| x == "--parser-messages" || x == "-pm") {
                                    println!(
                                        "{}[{}]{} {}[Parser Message]{}: {}[{:?}]{} - {}{}{}",
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Yellow
                                        ),
                                        e.id,
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Magenta
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Red
                                        ),
                                        e.message_type,
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Yellow
                                        ),
                                        e.message_data,
                                        ellie_lang::terminal_colors::get_color(
                                            ellie_lang::terminal_colors::Colors::Reset
                                        ),
                                    )
                                }
                            },
                            ellie_core::defs::ParserOptions {
                                path: "<eval>".to_string(),
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
                            if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                print!("*");
                            }
                            for error in
                                &ellie_lang::cli_utils::zip_errors(mapped.syntax_errors.clone())
                            {
                                if env::args().any(|x| x == "-je" || x == "--json-errors") {
                                    println!("+\n{:?}\n", serde_json::to_string(error).unwrap());
                                } else {
                                    if error.pos.range_start.0 != error.pos.range_end.0 {
                                        println!(
                                            "{}[Experimental]{}: Multi line error listing",
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
                            std::process::exit(1);
                        } else if env::args().any(|x| x == "-rw" || x == "--raw-compile") {
                            println!("Pre-compiled raw generation not supported yet {:#?}", code);
                        } else if !env::args().any(|x| x == "-se" || x == "--show-errors") {
                            print!(
                                "Collected: {:#?}",
                                mapped.parsed.items.into_iter().filter(|x| !matches!(
                                    x,
                                    ellie_parser::parser::Collecting::ImportItem(_)
                                ))
                            );
                            std::process::exit(0);
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
