use ellie::{mapper, utils};
use fs::File;
use std::env;
use std::{fs, io::Read};

fn main() {
    if env::args().any(|x| x == "-v") || env::args().any(|x| x == "--version") {
        const VERSION: &'static str = env!("CARGO_PKG_VERSION");
        println!("v{}", VERSION);
    } else if env::args().any(|x| x == "-h") || env::args().any(|x| x == "--help") {
        println!("Usage: ellie [options] [file path]");
        println!("Options:");
        println!("\t--version        || -v : Show Version");
        println!("\t--help           || -h : Show Help");
        println!("\t--raw-compile    || -rw  : Pre-compile program but dont run");
        println!("\t--enable-fs      || -df  : Enable file system access");
        println!("\t--enable-fs      || -df  : Enable file system access");
        println!("\t--enable-network || -en  : Enable network access");
    } else {
        let args = env::args()
            .collect::<Vec<String>>()
            .drain(1..)
            .collect::<Vec<String>>();

        if args.len() == 0 {
            println!("No file present\n-h for help");
        } else {
            let file_args = args
                .clone()
                .into_iter()
                .filter(|x| x.contains("."))
                .collect::<Vec<String>>();
            let debug_arg = env::args().any(|x| x == "--debug");
            //let map_errors_arg = env::args().any(|x| x == "--map-errors");
            let file_arg_check = file_args.first();
            if file_arg_check != None {
                let file_arg = file_arg_check.unwrap();
                //let file = Path::new(&file_arg.clone());
                let mut file_content = Vec::new();
                let file = File::open(&file_arg.clone());
                if file.is_err() {
                    println!("File not found ~{}", &file_arg.clone());
                } else {
                    file.unwrap()
                        .read_to_end(&mut file_content)
                        .expect("Unable to read");
                    let code_string = String::from_utf8(file_content);
                    if code_string.is_err() {
                        println!("Unable to read file ~{}", file_arg.clone())
                    } else {
                        let code = code_string.unwrap();
                        let mut mapper = mapper::Mapper::new(
                            code.clone(),
                            mapper::defs::MapperOptions {
                                functions: true,
                                break_on_error: false,
                                loops: true,
                                global_variables: true,
                                collectives: true,
                                variables: true,
                            },
                        );
                        let mapped = mapper.Map();
                        if mapped.syntax_errors.len() != 0 {
                            //let serialized = serde_json::to_string(&mapped.syntax_errors).unwrap();
                            //println!("serialized = {}", serialized);
                            for error in &mapped.syntax_errors {
                                println!(
                                    "{}{}Error[{:#04x}]{} - {}{}{}: {}",
                                    if debug_arg {
                                        format!(
                                            "{}[{}]{} ",
                                            utils::terminal_colors::get_color(
                                                utils::terminal_colors::Colors::Yellow
                                            ),
                                            error.debug_message,
                                            utils::terminal_colors::get_color(
                                                utils::terminal_colors::Colors::Reset
                                            )
                                        )
                                    } else {
                                        "".to_string()
                                    },
                                    utils::terminal_colors::get_color(
                                        utils::terminal_colors::Colors::Red
                                    ),
                                    &error.code,
                                    utils::terminal_colors::get_color(
                                        utils::terminal_colors::Colors::Reset
                                    ),
                                    utils::terminal_colors::get_color(
                                        utils::terminal_colors::Colors::Cyan
                                    ),
                                    error.title,
                                    utils::terminal_colors::get_color(
                                        utils::terminal_colors::Colors::Reset
                                    ),
                                    error.builded_message
                                );
                                println!(
                                    "{}:{}:{}",
                                    file_arg.clone(),
                                    error.pos.range_start.0 + 1,
                                    error.pos.range_start.1 + 1
                                );
                                println!(
                                    "{}\n{}{}{}",
                                    utils::get_line(code.clone(), error.pos.range_start.0 as usize),
                                    utils::terminal_colors::get_color(
                                        utils::terminal_colors::Colors::Red
                                    ),
                                    utils::arrow(
                                        (error.pos.range_start.1 + 1) as usize,
                                        ((error.pos.range_end.1) - (error.pos.range_start.1))
                                            as usize
                                    ),
                                    utils::terminal_colors::get_color(
                                        utils::terminal_colors::Colors::Reset
                                    )
                                );
                            }
                        } else {
                            if env::args().any(|x| x == "-rw")
                                || env::args().any(|x| x == "--raw-compile")
                            {
                                //let mut wraw = File::create("compiled.wraw").expect("Unable to create file");
                                //let serialized = serde_json::to_string(&point).unwrap();
                                //for i in &syntax.clone().items {
                                //    write!(wraw, "{:?}", i);
                                //}
                                println!(
                                    "Pre-compiled raw generation not supported yet {:#?}",
                                    code.clone()
                                );
                            } else {
                                print!("Collected: {:#?}", mapped);
                            }
                        }
                    }
                }
            } else {
                println!("No file present\n-h for help");
            }
        }
    }
}
