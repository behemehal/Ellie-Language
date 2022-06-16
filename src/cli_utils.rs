#[cfg(feature = "build-cli")]
use clap::ValueHint;
#[cfg(feature = "build-cli")]
use clap::{Arg, Command};

use ellie_core::{defs, error, warning};
use std::{
    collections::hash_map::DefaultHasher,
    fmt::Display,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
    path::Path,
};
extern crate path_absolutize;

pub enum TextStyles {
    Bold,
    Dim,
    Italic,
    Underline,
}

impl Display for TextStyles {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let type_id = match self {
            TextStyles::Bold => "[1m",
            TextStyles::Dim => "[2m",
            TextStyles::Italic => "[3m",
            TextStyles::Underline => "[4m",
        };
        write!(f, "{}{}", '\u{001b}', type_id)
    }
}

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
        write!(f, "{}{}", '\u{001b}', color_id)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum OutputTypes {
    Bin,
    DependencyAnalysis,
    Json,
    ByteCode,
    ByteCodeAsm,
    Nop,
}

#[derive(PartialEq, Eq, Debug)]
pub enum CliOutputType {
    Json,
    ConsoleOutput,
}

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

pub fn generate_blank(size: usize) -> String {
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

pub fn read_file<P: AsRef<Path>>(file_dir: P) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            match file.read_to_end(&mut file_content) {
                Ok(_) => match String::from_utf8(file_content) {
                    Ok(code_string) => Ok(code_string),
                    Err(e) => Err(e.to_string()),
                },
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

pub fn read_file_bin<P: AsRef<Path>>(file_dir: P) -> Result<Vec<u8>, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(file) => Ok(file.bytes().collect::<Result<Vec<u8>, _>>().unwrap()),
    }
}

pub fn hash_error(error: &error::Error) -> String {
    let mut hasher = DefaultHasher::new();
    format!("E{:?}", error).hash(&mut hasher);
    hasher.finish().to_string()
}

pub fn hash_warning(warning: &warning::Warning) -> String {
    let mut hasher = DefaultHasher::new();
    format!("W{:?}", warning).hash(&mut hasher);
    hasher.finish().to_string()
}

pub fn print_warnings<E, F>(warnings: &Vec<warning::Warning>, file_reader: E, path_resolver: F)
where
    E: FnOnce(String) -> String + Clone + Copy + Sized,
    F: FnOnce(String) -> String + Clone + Copy + Sized,
{
    for warning in warnings {
        println!(
            "\n{}Warning[{}]{}: {}{}{}\n",
            Colors::Yellow,
            warning.title,
            Colors::Reset,
            Colors::Cyan,
            warning.builded_message.builded,
            Colors::Reset,
        );

        let file_content = file_reader(warning.path.clone());
        let mut line_space = warning.pos.range_end.0.to_string().len() + 1;

        if let Some(refr) = warning.reference_block.clone() {
            let ref_file_content = file_reader(refr.1.clone());
            if line_space < refr.0.range_start.0.to_string().len() + 1 {
                line_space = refr.0.range_start.0.to_string().len() + 1;
            }
            render_code_block(
                path_resolver(refr.1.clone()),
                refr.0,
                ref_file_content,
                warning.reference_message.clone(),
                line_space,
                true,
                false,
            )
        }
        render_code_block(
            path_resolver(warning.path.clone()),
            warning.pos,
            file_content,
            "".to_owned(),
            line_space,
            false,
            false,
        );
        println!(
            "{}{}[?]{}╞ Check online standard rules repo for more info {}{}{}",
            generate_blank(line_space - 1),
            Colors::Magenta,
            Colors::Reset,
            Colors::Green,
            format!(
                "https://www.ellie-lang.org/standardRules.html#{:#04x}",
                warning.code
            ),
            Colors::Reset,
        );

        if warning.full_assist || warning.semi_assist {
            if cfg!(feature = "ellie_assist") {
                println!(
                    "{}{}[{}]{}╞ {} assistment available type '{}ellie{} {}--show-me-something{} {}{}{}' for request assist",
                    generate_blank(line_space - 1),
                    Colors::Yellow,
                    if warning.semi_assist {
                        "◆"
                    } else {
                        "✓"
                    },
                    Colors::Reset,
                    if warning.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    Colors::Green,
                    Colors::Reset,
                    Colors::Yellow,
                    Colors::Reset,
                    Colors::Green,
                    hash_warning(&warning),
                    Colors::Reset,
                );
            } else {
                println!(
                    "{}{}[x]{}╞ {} assistment available but {}ellie_assist{} feature is not enabled",
                    generate_blank(line_space - 1),
                    Colors::Yellow,
                    Colors::Reset,
                    if warning.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    Colors::Red,
                    Colors::Reset,
                );
            }
        }
    }
}

pub fn print_errors<E, F>(
    errors: &Vec<error::Error>,
    file_reader: E,
    show_debug_lines: bool,
    path_resolver: F,
) where
    E: FnOnce(String) -> String + Clone + Copy + Sized,
    F: FnOnce(String) -> String + Clone + Copy + Sized,
{
    for error in errors {
        println!(
            "\n{}Error[{:#04x}{}]{}: {}{}{}\n",
            Colors::Red,
            error.code,
            if show_debug_lines {
                format!(" - {}", error.debug_message)
            } else {
                "".to_owned()
            },
            Colors::Reset,
            Colors::Cyan,
            error.builded_message.builded,
            Colors::Reset,
        );
        let file_content = file_reader(error.path.clone());
        let mut line_space = error.pos.range_start.0.to_string().len() + 1;
        if let Some(refr) = error.reference_block.clone() {
            let ref_file_content = file_reader(refr.1.clone());
            if line_space < refr.0.range_start.0.to_string().len() + 1 {
                line_space = refr.0.range_start.0.to_string().len() + 1;
            }
            render_code_block(
                path_resolver(refr.1.clone()),
                refr.0,
                ref_file_content,
                error.reference_message.clone(),
                line_space,
                true,
                true,
            )
        }
        render_code_block(
            path_resolver(error.path.clone()),
            error.pos,
            file_content,
            "".to_owned(),
            line_space,
            false,
            true,
        );
        println!(
            "  {}[?]{}{}╞ Check online error repo for more info {}{}{}",
            Colors::Magenta,
            Colors::Reset,
            if line_space < 3 {
                String::new()
            } else {
                generate_blank(line_space - 3)
            },
            Colors::Green,
            format!(
                "https://www.ellie-lang.org/errorIndex.html#{:#04x}",
                error.code
            ),
            Colors::Reset,
        );

        if error.full_assist || error.semi_assist {
            if cfg!(feature = "ellie_assist") {
                println!(
                    "{}{}[{}]{}╞ {} assistment available type '{}ellie{} {}--show-me-something{} {}{}{}' for request assist",
                    generate_blank(line_space - 1),
                    Colors::Yellow,
                    if error.semi_assist {
                        "◆"
                    } else {
                        "✓"
                    },
                    Colors::Reset,
                    if error.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    Colors::Green,
                    Colors::Reset,
                    Colors::Yellow,
                    Colors::Reset,
                    Colors::Green,
                    hash_error(&error),
                    Colors::Reset,
                );
            } else {
                println!(
                    "{}{}[x]{}╞ {} assistment available but {}ellie_assist{} feature is not enabled",
                    generate_blank(line_space - 1),
                    Colors::Yellow,
                    Colors::Reset,
                    if error.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    Colors::Red,
                    Colors::Reset,
                );
            }
        }

        if error.code == 0x00 && errors.len() > 2 {
            println!(
                "\n{}{}{} other error omitted",
                Colors::Red,
                errors.len() - 2,
                Colors::Reset
            );
            break;
        }
    }
}

pub fn render_code_block(
    item_path: String,
    item_pos: defs::Cursor,
    code: String,
    ref_message: String,
    line_space: usize,
    reference: bool,
    is_error: bool,
) {
    let multi_line = item_pos.range_start.0 != item_pos.range_end.0;
    println!(
        "  {}[{}]{}{}╞ {}{}:{}{}{}",
        if reference {
            Colors::Green
        } else if is_error {
            Colors::Red
        } else {
            Colors::Yellow
        },
        if reference { "@" } else { "~" },
        if line_space < 3 {
            String::new()
        } else {
            generate_blank(line_space - 3)
        },
        Colors::Reset,
        Colors::Green,
        item_path,
        Colors::Cyan,
        format!(
            "{}:{}{} > {}{}:{}",
            item_pos.range_start.0 + 1,
            item_pos.range_start.1 + 1,
            Colors::Red,
            Colors::Cyan,
            item_pos.range_end.0 + 1,
            item_pos.range_end.1 + 1
        ),
        Colors::Reset,
    );

    let line_start = if item_pos.range_start.0 >= 2 {
        item_pos.range_start.0 - 2
    } else {
        0
    };

    let line_end = if (item_pos.range_end.0 + 2) <= code.lines().count() {
        item_pos.range_end.0 + 2
    } else {
        code.lines().count()
    };

    if multi_line {
        println!(
            "  {}[!]{}{}╞ (Beta) Mutli line rendering",
            if reference {
                Colors::Green
            } else if is_error {
                Colors::Red
            } else {
                Colors::Yellow
            },
            if line_space < 3 {
                String::new()
            } else {
                generate_blank(line_space - 3)
            },
            Colors::Reset,
        );
    }

    if !reference {
        println!(
            "{}{}{}  │ {}",
            Colors::Yellow,
            generate_blank(line_space),
            Colors::Reset,
            if line_start == 0 { "" } else { "..." }
        );
    }

    for i in line_start..line_end {
        if item_pos.range_start.0 == item_pos.range_end.0
            && i == item_pos.range_start.0
            && !multi_line
        {
            if reference {
                println!(
                    "{}{}{}{} │ {} {} {}{}",
                    if i >= item_pos.range_start.0 && i <= item_pos.range_end.0 {
                        Colors::Green
                    } else {
                        Colors::Yellow
                    },
                    generate_blank((line_space - (i + 1).to_string().len()) + 1),
                    i + 1,
                    Colors::Reset,
                    get_line(code.clone(), i).replace("\t", "    "), //:/
                    Colors::Green,
                    ref_message,
                    Colors::Reset,
                );

                println!(
                    "{} │ {}{}{}",
                    generate_blank(line_space + 1),
                    if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    },
                    arrow(
                        (item_pos.range_start.1 + 1) as usize,
                        ((item_pos.range_end.1) - item_pos.range_start.1) + 1
                    ),
                    Colors::Reset,
                );
            } else {
                println!(
                    "{}{}{}{} │ {}",
                    Colors::Yellow,
                    generate_blank((line_space - (i + 1).to_string().len()) + 1),
                    i + 1,
                    Colors::Reset,
                    get_line(code.clone(), i).replace("\t", "    "), //WTF? THIS IS THE ONLY SOLUTION
                );

                println!(
                    "{} │ {}{}{}",
                    generate_blank(line_space + 1),
                    if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    },
                    arrow(
                        (item_pos.range_start.1 + 1) as usize,
                        ((item_pos.range_end.1) - item_pos.range_start.1) + 1
                    ),
                    Colors::Reset,
                );
            }
        } else {
            println!(
                "{}{}{}{} │ {}{}{}",
                if i >= item_pos.range_start.0 && i <= item_pos.range_end.0 {
                    if reference {
                        Colors::Green
                    } else {
                        Colors::Red
                    }
                } else {
                    Colors::Yellow
                },
                generate_blank((line_space - (i + 1).to_string().len()) + 1),
                i + 1,
                Colors::Reset,
                if (i >= item_pos.range_start.0 && i <= item_pos.range_end.0) && multi_line {
                    if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    }
                } else {
                    Colors::White
                },
                get_line(code.clone(), i),
                Colors::Reset,
            );

            if reference
                && i > item_pos.range_end.0
                && item_pos.range_end.0 != item_pos.range_start.0
            {
                println!(
                    "{} │ {}{}{}",
                    generate_blank(line_space + 1),
                    if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    },
                    format!("^ {}", ref_message),
                    Colors::Reset,
                );
            }
        }
    }

    if reference {
        println!(
            "{}{}{}  ├──",
            Colors::Yellow,
            generate_blank(line_space),
            Colors::Reset,
        );
    } else {
        println!(
            "{}{}{}  │ {}",
            Colors::Yellow,
            generate_blank(line_space),
            Colors::Reset,
            if line_end <= code.lines().count() {
                "..."
            } else {
                "──"
            }
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

pub fn read_error_text(error: u8) -> &'static str {
    match error {
        0 => "FailedToAccessResource[0]",
        1 => "BrokenFile<IllegalOpCode>[1]",
        2 => "BrokenFile<??>[2]",
        3 => "NotExecutable[3]",
        _ => "UnknownErrorCode",
    }
}