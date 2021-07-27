use crate::terminal_colors;

use ellie_core::{defs, error};
use ellie_parser::{self, parser};
use std::{env, fs::File, io::Read};

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

pub fn system_module_resolver(lib_name: String) -> Option<ellie_parser::parser::Parsed> {
    let core_resolver = |e: String| {
        println!(
            "{}[DEBUG:ParserInfo]{}: Import Request '{}{}{}'",
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Cyan),
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Reset),
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Yellow),
            e,
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Reset),
        );
        if e == "ellie"
            || e == "string"
            || e == "void"
            || e == "int"
            || e == "char"
            || e == "collective"
            || e == "bool"
            || e == "float"
            || e == "cloak"
            || e == "array"
        {
            if let Some(e) = crate::cli_utils::system_module_resolver(e.clone()) {
                parser::ResolvedImport {
                    found: true,
                    file_content: e,
                }
            } else {
                parser::ResolvedImport::default()
            }
        } else {
            println!(
                "{}[DEBUG:ParserInfo]{}: Import Rejected on: '{}'",
                crate::terminal_colors::get_color(crate::terminal_colors::Colors::Red),
                crate::terminal_colors::get_color(crate::terminal_colors::Colors::Reset),
                e
            );
            parser::ResolvedImport::default()
        }
    };

    let mut ellie_library_content = Vec::new();
    /*


                                 o_Oo_Oo_Oo_O
            o_Oo_Oo_Oo_O         o_O      o_O
            o_O      o_O         o_O      o_O
            o_O      o_O         o_O      o_O
            o_Oo_Oo_Oo_O         o_Oo_Oo_Oo_O

            o_Oo_Oo_Oo_Oo_Oo_Oo_Oo_Oo_Oo_Oo_O
            o_Oo_Oo_Oo_Oo_Oo_Oo_Oo_Oo_Oo_Oo_O

            If you want to use std you should replace this with path
    */
    let mut ellie_library = File::open(
        "C:\\Users\\ahmet\\Desktop\\Projects\\InBuild\\Ellie-Language\\lib\\".to_string()
            + &(lib_name + ".ei"),
    )
    .unwrap();
    ellie_library
        .read_to_end(&mut ellie_library_content)
        .expect("Unable to read");
    let ellie_code_string = String::from_utf8(ellie_library_content).unwrap();
    let child_parser: ellie_parser::parser::ParserResponse = ellie_parser::parser::Parser::new(
        ellie_code_string,
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
    )
    .map();
    if child_parser.syntax_errors.len() != 0 {
        println!(
            "{}[DEBUG:ParserInfo]{}: Import Failed",
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Red),
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Reset),
        );
        None
    } else {
        println!(
            "{}[DEBUG:ParserInfo]{}: Import Sucess",
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Green),
            crate::terminal_colors::get_color(crate::terminal_colors::Colors::Reset),
        );
        Some(child_parser.parsed)
    }
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
