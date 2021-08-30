use std::path::Path;
use std::{fs::File, io::Read};
extern crate path_absolutize;

use crate::terminal_colors;
use ellie_core::{defs, error};
use ellie_parser::parser;
use path_absolutize::Absolutize;

pub struct EllieModuleResolver {
    pub main_path: String,
}

impl EllieModuleResolver {}

pub fn parse(contents: String, file_name: String) -> ellie_parser::parser::ParserResponse {
    std::println!(
        "{}[ParsingFile]{}: {}~{}{}",
        terminal_colors::get_color(terminal_colors::Colors::Cyan),
        terminal_colors::get_color(terminal_colors::Colors::Reset),
        terminal_colors::get_color(terminal_colors::Colors::Yellow),
        file_name,
        terminal_colors::get_color(terminal_colors::Colors::Reset),
    );

    let parser = parser::Parser::new(
        contents.clone(),
        resolve_import,
        |_| {},
        ellie_core::defs::ParserOptions {
            path: file_name.to_string(),
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
            "{}[ParsingSuccess]{}: {}~{}{}",
            terminal_colors::get_color(terminal_colors::Colors::Green),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
        );
    } else {
        println!(
            "{}[ParsingFailed]{}: {}~{}{}",
            terminal_colors::get_color(terminal_colors::Colors::Red),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            file_name,
            terminal_colors::get_color(terminal_colors::Colors::Reset)
        );
    }
    parsed
}

pub fn resolve_import(
    options: ellie_core::defs::ParserOptions,
    lib_name: String,
    nativeHeader: bool,
) -> ellie_parser::parser::ResolvedImport {
    let parent = &(Path::new(&options.path.clone())
        .parent()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string()
        + "/");
    let path = parent.clone()
        + &lib_name
        + if !Path::new(&lib_name).extension().is_some() {
            if nativeHeader {
                ".eih"
            } else {
                ".ei"
            }
        } else {
            ""
        };

    if lib_name == "ellie" {
        ellie_parser::parser::ResolvedImport {
            found: true,
            resolved_path: "<virtual>".to_string(),
            file_content: ellie_core::builded_libraries::ELLIE_STANDARD_LIBRARY.to_string(),
            ..Default::default()
        }
    } else {
        if Path::new(&path).absolutize().unwrap().to_str().unwrap()
            == Path::new(&options.path.clone())
                .absolutize()
                .unwrap()
                .to_str()
                .unwrap()
        {
            ellie_parser::parser::ResolvedImport {
                found: false,
                resolve_error: "Importing this file causes infinite loop".to_string(),
                ..Default::default()
            }
        } else {
            match read_file(Path::new(&path).absolutize().unwrap().to_str().unwrap()) {
                Ok(file) => ellie_parser::parser::ResolvedImport {
                    found: true,
                    file_content: file,
                    resolved_path: Path::new(&path)
                        .absolutize()
                        .unwrap()
                        .to_str()
                        .unwrap()
                        .to_string(),
                    ..Default::default()
                },
                Err(c) => ellie_parser::parser::ResolvedImport {
                    found: false,
                    resolve_error: format!(
                        "Cannot find module '{}' ({})",
                        Path::new(&path).absolutize().unwrap().to_str().unwrap(),
                        c
                    ),
                    ..Default::default()
                },
            }
        }
    }
}

pub fn is_errors_same(first: error::Error, second: error::Error) -> bool {
    first.code == second.code
        && first.message == second.message
        && first.pos.range_start.0 == second.pos.range_start.0
        && first.pos.range_start.1 == second.pos.range_start.1
}

pub fn clean_up_escape(code: String) -> String {
    code.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
}

pub fn zip_errors(errors: Vec<error::Error>) -> Vec<error::Error> {
    let mut clone_errors: Vec<error::Error> = errors.clone();
    let mut zipped_errors: Vec<error::Error> = Vec::new();
    for i in 0..clone_errors.len() {
        if i != 0 {
            if is_errors_same(clone_errors[i - 1].clone(), clone_errors[i].clone()) {
                let last_error = clone_errors.clone()[i - 1].clone();
                clone_errors[i].pos.range_start = last_error.pos.range_start;

                for field in 0..last_error.builded_message.fields.len() {
                    if last_error.builded_message.fields[field].value
                        != clone_errors[i].builded_message.fields[field].value
                    {
                        clone_errors[i].builded_message.fields[field].value =
                            last_error.builded_message.fields[field].value.clone()
                                + " "
                                + &clone_errors[i].builded_message.fields[field].value;
                    }
                }

                if i == errors.len() - 1
                    || !is_errors_same(clone_errors[i].clone(), clone_errors[i + 1].clone())
                {
                    clone_errors[i].builded_message = error::Error::build(
                        clone_errors[i].message.clone(),
                        clone_errors[i].builded_message.fields.clone(),
                    );
                    zipped_errors.push(clone_errors[i].clone())
                }
            } else {
                zipped_errors.push(clone_errors[i].clone())
            }
        } else if errors.len() > 1
            && !is_errors_same(clone_errors[0].clone(), clone_errors[1].clone())
            || errors.len() == 1
        {
            zipped_errors.push(clone_errors[0].clone());
        }
    }

    zipped_errors
}

pub fn draw_error(line: String, pos: defs::CursorPosition) -> String {
    let mut draw = String::new();

    for (index, c) in line.chars().enumerate() {
        if index >= (if pos.1 != 0 { pos.1 - 1 } else { pos.1 }) {
            draw += &format!(
                "{}{}{}",
                terminal_colors::get_color(terminal_colors::Colors::Red),
                c,
                terminal_colors::get_color(terminal_colors::Colors::Reset),
            )
            .to_string();
        } else {
            draw += &format!(
                "{}{}{}",
                terminal_colors::get_color(terminal_colors::Colors::White),
                c,
                terminal_colors::get_color(terminal_colors::Colors::Reset),
            )
            .to_string();
        }
    }
    draw
}

fn generate_blank(size: usize) -> String {
    let mut blank: String = String::new();
    for _ in 0..size + 1 {
        blank += &" ".to_string();
    }
    blank
}

pub fn get_lines(code: String, lines: defs::Cursor) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    let mut render = String::new();
    for i in lines.range_start.0..lines.range_end.0 + 1 {
        let t = format!(
            "{}{}{}{}{}|{} {}\n",
            terminal_colors::get_color(terminal_colors::Colors::Magenta),
            i + 1,
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            generate_blank(v.len().to_string().len() - (i + 1).to_string().len()),
            terminal_colors::get_color(terminal_colors::Colors::Yellow),
            terminal_colors::get_color(terminal_colors::Colors::Reset),
            draw_error(
                v[i].to_string(),
                if lines.range_start.0 == i {
                    lines.range_start
                } else {
                    lines.range_end
                }
            ),
        );
        render += &t;
    }
    render
}

pub fn get_line(code: String, line: usize) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    if line > v.len() {
        v[v.len() - 1].to_string()
    } else {
        v[line].to_string()
    }
}

pub fn read_file(file_dir: &str) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).expect("Unable to read");
            match String::from_utf8(file_content) {
                Ok(code_string) => Ok(code_string),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

pub fn arrow(line: usize, range: usize) -> String {
    let mut s = String::with_capacity(line);
    let mut range_arrows = String::with_capacity(range);
    for _ in 0..range {
        range_arrows.push('^')
    }
    if line == 0 {
        s = range_arrows;
    } else {
        for e in 0..line {
            if e == line - 1 {
                s.push_str(&range_arrows);
            } else {
                s.push(' ');
            }
        }
    }
    s
}
