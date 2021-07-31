use ellie_core;
use ellie_parser::parser;
use regex::Regex;
#[path = "src/terminal_colors.rs"]
mod terminal_colors;
use serde_json;
use std::{
    fs::{self, File},
    io::Read,
};

fn read_file(file_dir: &str) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).expect("Unable to read");
            let code_string = String::from_utf8(file_content);
            Ok(code_string.unwrap())
        }
    }
}

fn resolve_import(lib_name: String) -> ellie_parser::parser::ResolvedImport {
    std::println!(
        "{}[ReadingFile]{}: {}~./lib/{}.ei{}",
        terminal_colors::get_color(terminal_colors::Colors::Magenta),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        lib_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );
    match read_file(&("./lib/".to_string() + &lib_name + &".ei".to_string())) {
        Ok(e) => ellie_parser::parser::ResolvedImport {
            found: true,
            file_content: parse(e, lib_name).parsed,
        },
        Err(err) => {
            std::println!(
                "{}[Fail]{}: Cannot read file {}~./lib/{}.ei{}\n{:#?}",
                terminal_colors::get_color(terminal_colors::Colors::Red),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                lib_name,
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                err
            );
            ellie_parser::parser::ResolvedImport::default()
        }
    }
}

fn parse(contents: String, file_name: String) -> ellie_parser::parser::ParserResponse {
    std::println!(
        "{}[ParsingFile]{}: {}~./lib/{}.ei{}",
        terminal_colors::get_color(terminal_colors::Colors::Cyan),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        file_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );
    let parser = parser::Parser::new(
        contents.clone(),
        resolve_import,
        ellie_core::defs::ParserOptions {
            functions: true,
            break_on_error: false,
            loops: true,
            conditions: true,
            classes: true,
            dynamics: true,
            global_variables: true,
            line_ending: "\\n".to_string(),
            collectives: true,
            variables: true,
            constants: true,
            parser_type: ellie_core::defs::ParserType::RawParser,
            allow_import: true,
        },
    );
    let parsed = parser.map();

    if parsed.syntax_errors.len() == 0 {
        std::println!(
            "{}[ParsingSuccess]{}: {}~./lib/{}.ei{}",
            terminal_colors::get_color(terminal_colors::Colors::Green),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    } else {
        std::println!(
            "{}[ParsingFailed]{}: {}~./lib/{}.ei{}",
            terminal_colors::get_color(terminal_colors::Colors::Red),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    }
    parsed
}

fn main() {
    let ellie_lib = resolve_import("ellie".to_string());
    let ellie_version: Vec<ellie_parser::syntax::file_key::FileKey> = ellie_lib
        .file_content
        .clone()
        .items
        .into_iter()
        .filter_map(|x| {
            if let ellie_parser::parser::Collecting::FileKey(e) = x {
                if e.data.key_name == "version" {
                    Some(e.data)
                } else {
                    None
                }
            } else {
                None
            }
        })
        .collect();

    if ellie_version.len() == 0 {
        println!(
            "\nCompiling Ellie Failed, No version key found in {}ellie.ei{}",
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    } else {
        let version_key = &ellie_version[0].value;
        if let ellie_parser::syntax::types::Types::String(e) = version_key {
            let version_regex = Regex::new(r"(\^|\~?)(\d|x|\*)+\.(\d|x|\*)+\.(\d|x|\*)+").unwrap();

            if version_regex.is_match(&e.value) {
                println!(
                    "\nCompiling Ellie standard library {}v{}{} complete",
                    terminal_colors::get_color(terminal_colors::Colors::Yellow),
                    e.value,
                    terminal_colors::get_color(terminal_colors::Colors::Reset),
                );
                let j = serde_json::to_string(&ellie_lib.file_content).unwrap();
                fs::write(
                    "./core/src/builded_libraries.rs",
                    format!("pub static ELLIE_STANDARD_LIBRARY : &str = {:#?};", j),
                )
                .unwrap();
            } else {
                println!(
                    "\nCompiling Ellie Failed, Version key found in {}ellie.ei{} does not fit to versioning rules.",
                    terminal_colors::get_color(terminal_colors::Colors::Yellow),
                    terminal_colors::get_color(terminal_colors::Colors::Reset),
                );
            }
        } else {
            println!(
                "\nCompiling Ellie Failed, Version key found in {}ellie.ei{} was '{}' but 'string' expected.",
                terminal_colors::get_color(terminal_colors::Colors::Yellow),
                terminal_colors::get_color(terminal_colors::Colors::Reset),
                version_key.get_type(),
            );
        }
    }
}
