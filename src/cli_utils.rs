use std::{fmt::Display, fs::File, io::Read, path::Path};

extern crate path_absolutize;

pub enum Colors {
    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,
    Reset,
}

impl Display for Colors {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let color_id = match self {
            Colors::Black => "[30m",
            Colors::Red => "[31m",
            Colors::Green => "[32m",
            Colors::Yellow => "[33m",
            Colors::Blue => "[34m",
            Colors::Magenta => "[35m",
            Colors::Cyan => "[36m",
            Colors::White => "[37m",
            Colors::Reset => "[0m",
        };
        return write!(f, "{}{}", '\u{001b}', color_id);
    }
}

use ellie_core::{defs, error};

pub struct EllieModuleResolver {
    pub main_path: String,
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
                    clone_errors[i] = clone_errors[i].clone().build(
                        clone_errors[i].builded_message.fields.clone(),
                        clone_errors[i].debug_message.clone(),
                        clone_errors[i].pos,
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
            draw += &format!("{}{}{}", Colors::Red, c, Colors::Reset,).to_string();
        } else {
            draw += &format!("{}{}{}", Colors::White, c, Colors::Reset,).to_string();
        }
    }
    draw
}

fn generate_blank(size: usize) -> String {
    let mut blank: String = String::new();
    for _ in 0..size + 1 {
        blank += &" ".to_owned();
    }
    blank
}

pub fn get_lines(code: String, lines: defs::Cursor) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    let mut render = String::new();
    for i in lines.range_start.0..lines.range_end.0 + 1 {
        let t = format!(
            "{}{}{}{}{}|{} {}\n",
            Colors::Magenta,
            i + 1,
            Colors::Reset,
            generate_blank(v.len().to_string().len() - (i + 1).to_string().len()),
            Colors::Yellow,
            Colors::Reset,
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

pub fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
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

pub fn print_errors(errors: Vec<error::Error>) {
    for error in errors {
        println!(
            "{}Error[{:#04x}-{}]{}: {}{}{}",
            Colors::Red,
            error.code,
            error.debug_message,
            Colors::Reset,
            Colors::Cyan,
            error.builded_message.builded,
            Colors::Reset,
        );
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
