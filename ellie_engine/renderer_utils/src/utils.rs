use ellie_core::{defs, error, warning};

#[cfg(feature = "cli-utils")]
use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
    path::Path,
};
#[cfg(feature = "cli-utils")]
extern crate path_absolutize;

pub enum TextStyles {
    Bold,
    Dim,
    Italic,
    Underline,
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

/// ColorDisplay trait is a representation of a colorable text. It is used to create a interface for output of errors and warnings.
pub trait ColorDisplay: Copy {
    /// Return a string with color
    /// ## Parameters
    /// * `color` - color [`Colors`]
    /// ## Returns
    /// [`String`]
    fn color(&self, color: Colors) -> String;
    fn text_style(&self, text_style: TextStyles) -> String;
}

#[cfg(feature = "cli-utils")]
#[derive(Debug, Clone, Copy)]
pub struct CliColor;

#[cfg(feature = "cli-utils")]
impl ColorDisplay for CliColor {
    fn color(&self, color: Colors) -> String {
        let color_id = match color {
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
        format!("{}{}", '\u{001b}', color_id)
    }

    fn text_style(&self, text_style: TextStyles) -> String {
        let type_id = match text_style {
            TextStyles::Bold => "[1m",
            TextStyles::Dim => "[2m",
            TextStyles::Italic => "[3m",
            TextStyles::Underline => "[4m",
        };
        format!("{}{}", '\u{001b}', type_id)
    }
}

#[cfg(feature = "cli-utils")]
pub fn file_exists(path: String) -> bool {
    Path::new(&path).exists()
}

#[cfg(feature = "cli-utils")]
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

#[cfg(feature = "cli-utils")]
pub fn read_file_bin<P: AsRef<Path>>(file_dir: P) -> Result<Vec<u8>, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(file) => Ok(file.bytes().collect::<Result<Vec<u8>, _>>().unwrap()),
    }
}

#[cfg(feature = "cli-utils")]
pub fn hash_error(error: &error::Error) -> String {
    let mut hasher = DefaultHasher::new();
    format!("E{:?}", error).hash(&mut hasher);
    hasher.finish().to_string()
}

#[cfg(feature = "cli-utils")]
pub fn hash_warning(warning: &warning::Warning) -> String {
    let mut hasher = DefaultHasher::new();
    format!("W{:?}", warning).hash(&mut hasher);
    hasher.finish().to_string()
}

/// Check if given two errors are equal
/// ## Parameters
/// * `a` - First error [`Error`]
/// * `b` - Second error [`Error`]
/// ## Returns
/// If errors are equal, return [`true`]. Otherwise, return [`false`].
/// ## Todo
/// Implementing in PartialEq would be better.
pub fn is_errors_same(first: error::Error, second: error::Error) -> bool {
    first.code == second.code
        && first.message == second.message
        && first.pos.range_start.0 == second.pos.range_start.0
        && first.pos.range_start.1 == second.pos.range_start.1
}

/// Make escaped '\\t \\r \\n' to '\t \r \n'
/// ## Parameters
/// * `code` - Code to escape [`String`]
/// ## Returns
/// Escaped code [`String`]
pub fn clean_up_escape(code: String) -> String {
    code.replace("\\n", "\n")
        .replace("\\t", "\t")
        .replace("\\r", "\r")
}

/// Collides given errors if they are same.
/// ## Parameters
/// * `errors` - Errors to be collapsed [`Vec<error::Error>`]
/// ## Returns
/// Collapsed errors [`Vec<error::Error>`]
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

/// Draw error line with red color and normal color for the rest of the line
/// ## Parameters
/// * `line` - Code line [`String`]
/// * `pos` - Position of error [`defs::CursorPosition`]
/// * `color_output` - Color output [`ColorDisplay`]
/// ## Returns
/// [`String`]
pub fn draw_error<T: ColorDisplay>(
    line: String,
    pos: defs::CursorPosition,
    color_output: T,
) -> String {
    let mut draw = String::new();

    for (index, c) in line.chars().enumerate() {
        if index >= (if pos.1 != 0 { pos.1 - 1 } else { pos.1 }) {
            draw += &format!(
                "{}{}{}",
                color_output.color(Colors::Red),
                c,
                color_output.color(Colors::Reset),
            )
            .to_string();
        } else {
            draw += &format!(
                "{}{}{}",
                color_output.color(Colors::White),
                c,
                color_output.color(Colors::Reset),
            )
            .to_string();
        }
    }
    draw
}

/// Generate blank space
/// ## Parameters
/// * `count` - Count of blank spaces [`usize`]
/// ## Returns
/// [`String`] with blank spaces you provided
pub fn generate_blank(size: usize) -> String {
    let mut blank: String = String::new();
    for _ in 0..size + 1 {
        blank += &" ".to_string();
    }
    blank
}

/// Get specific line from code and render line number withit
/// ## Parameters
/// * `code` - Code [`String`]
/// * `line` - Line number [`usize`]
/// * `color_output` - Color output [`ColorDisplay`]
/// ## Returns
/// Colorized and line number filled [`String`]
pub fn _get_lines<T: ColorDisplay>(code: String, lines: defs::Cursor, color_output: T) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    let mut render = String::new();
    for i in lines.range_start.0..lines.range_end.0 + 1 {
        let t = format!(
            "{}{}{}{}{}|{} {}\n",
            color_output.color(Colors::Magenta),
            i + 1,
            color_output.color(Colors::Reset),
            generate_blank(v.len().to_string().len() - (i + 1).to_string().len()),
            color_output.color(Colors::Yellow),
            color_output.color(Colors::Reset),
            draw_error(
                v[i].to_string(),
                if lines.range_start.0 == i {
                    lines.range_start
                } else {
                    lines.range_end
                },
                color_output
            ),
        );
        render += &t;
    }
    render
}

/// Get line from code
pub fn get_line(code: String, line: usize) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    if line > v.len() {
        v[v.len() - 1].to_string()
    } else {
        v[line].to_string()
    }
}

/// Render code block with desired output type (Warning | Error)
/// ## Parameters
/// * item_path: File's path to render at begining [`String`]
/// * item_pos: Position of code block to render [`defs::Cursor`]
/// * code: File content [`String`]
/// * ref_message: A message to show at referenced code block [`String`]
/// * line_space: Count of line requested to be rendered at begining
/// * reference: Is this a reference highlight?
/// * is_error: Is this a error otherwise check reference else render as warning
/// * color_output: Color decor [`ColorDisplay`]
pub(crate) fn render_code_block<T: ColorDisplay>(
    item_path: String,
    item_pos: defs::Cursor,
    code: String,
    ref_message: String,
    line_space: usize,
    reference: bool,
    is_error: bool,
    color_output: T,
) -> String {
    let mut output = String::new();
    let multi_line = item_pos.range_start.0 != item_pos.range_end.0;
    output += &format!(
        "  {}[{}]{}{}╞ {}{}:{}{}{}\n",
        if reference {
            color_output.color(Colors::Green)
        } else if is_error {
            color_output.color(Colors::Red)
        } else {
            color_output.color(Colors::Yellow)
        },
        if reference { "@" } else { "~" },
        if line_space < 3 {
            String::new()
        } else {
            generate_blank(line_space - 3)
        },
        color_output.color(Colors::Reset),
        color_output.color(Colors::Green),
        item_path,
        color_output.color(Colors::Cyan),
        format!(
            "{}:{}{} > {}{}:{}",
            item_pos.range_start.0 + 1,
            item_pos.range_start.1 + 1,
            color_output.color(Colors::Red),
            color_output.color(Colors::Cyan),
            item_pos.range_end.0 + 1,
            item_pos.range_end.1 + 1
        ),
        color_output.color(Colors::Reset),
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

    if !reference {
        output += &format!(
            "{}{}{}  │ {}\n",
            color_output.color(Colors::Yellow),
            generate_blank(line_space),
            color_output.color(Colors::Reset),
            if line_start == 0 { "" } else { "..." }
        );
    }

    for i in line_start..line_end {
        if item_pos.range_start.0 == item_pos.range_end.0
            && i == item_pos.range_start.0
            && !multi_line
        {
            if reference {
                output += &format!(
                    "{}{}{}{} │ {} {} {}{}\n",
                    color_output.color(
                        if i >= item_pos.range_start.0 && i <= item_pos.range_end.0 {
                            Colors::Green
                        } else {
                            Colors::Yellow
                        }
                    ),
                    generate_blank((line_space - (i + 1).to_string().len()) + 1),
                    i + 1,
                    color_output.color(Colors::Reset),
                    get_line(code.clone(), i).replace("\t", "    "), //:/
                    color_output.color(Colors::Green),
                    ref_message,
                    color_output.color(Colors::Reset),
                );

                output += &format!(
                    "{} │ {}{}{}\n",
                    generate_blank(line_space + 1),
                    color_output.color(if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    }),
                    arrow(
                        (item_pos.range_start.1 + 1) as usize,
                        ((item_pos.range_end.1) - item_pos.range_start.1) + 1
                    ),
                    color_output.color(Colors::Reset),
                );
            } else {
                output += &format!(
                    "{}{}{}{} │ {}\n",
                    color_output.color(Colors::Yellow),
                    generate_blank((line_space - (i + 1).to_string().len()) + 1),
                    i + 1,
                    color_output.color(Colors::Reset),
                    get_line(code.clone(), i).replace("\t", "    "), //WTF? THIS IS THE ONLY SOLUTION
                );

                output += &format!(
                    "{} │ {}{}{}\n",
                    generate_blank(line_space + 1),
                    color_output.color(if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    }),
                    arrow(
                        (item_pos.range_start.1 + 1) as usize,
                        ((item_pos.range_end.1) - item_pos.range_start.1) + 1
                    ),
                    color_output.color(Colors::Reset),
                );
            }
        } else {
            output += &format!(
                "{}{}{}{} │ {}{}{}\n",
                color_output.color(
                    if i >= item_pos.range_start.0 && i <= item_pos.range_end.0 {
                        if reference {
                            Colors::Green
                        } else {
                            Colors::Red
                        }
                    } else {
                        Colors::Yellow
                    }
                ),
                generate_blank((line_space - (i + 1).to_string().len()) + 1),
                i + 1,
                color_output.color(Colors::Reset),
                if (i >= item_pos.range_start.0 && i <= item_pos.range_end.0) && multi_line {
                    color_output.color(if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    })
                } else {
                    color_output.color(Colors::White)
                },
                get_line(code.clone(), i),
                color_output.color(Colors::Reset),
            );

            if reference
                && i > item_pos.range_end.0
                && item_pos.range_end.0 != item_pos.range_start.0
            {
                output += &format!(
                    "{} │ {}{}{}\n",
                    generate_blank(line_space + 1),
                    color_output.color(if reference {
                        Colors::Green
                    } else if is_error {
                        Colors::Red
                    } else {
                        Colors::Yellow
                    }),
                    format!("^ {}", ref_message),
                    color_output.color(Colors::Reset),
                );
            }
        }
    }

    if reference {
        output += &format!(
            "{}{}{}  ├──\n",
            color_output.color(Colors::Yellow),
            generate_blank(line_space),
            color_output.color(Colors::Reset),
        );
    } else {
        output += &format!(
            "{}{}{}  │ {}\n",
            color_output.color(Colors::Yellow),
            generate_blank(line_space),
            color_output.color(Colors::Reset),
            if line_end <= code.lines().count() {
                "..."
            } else {
                "──"
            }
        );
    }
    output
}

/// Draw arrow for error
/// ## Parameters
/// * `line` - Code line [`String`]
/// ## Returns
/// [`String`] with arrow
pub(crate) fn arrow(line: usize, range: usize) -> String {
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

/// Generate vm error code from [`u8`]
/// ## Parameters
/// * `code` - Error code [`u8`]
/// ## Returns
/// [`String`] with error code
pub fn read_error_text(error: u8) -> &'static str {
    match error {
        0 => "FailedToAccessResource[0]",
        1 => "BrokenFile<IllegalOpCode>[1]",
        2 => "BrokenFile<??>[2]",
        3 => "NotExecutable[3]",
        _ => "UnknownErrorCode",
    }
}

/// Output given error list as string
/// ## Parameters
/// * errors: Vector of errors [`Vec<error::Error>`]
/// * file_reader: A file reader interface
/// * show_debug_lines: Shows debug lines in errors if supplied [`bool`]
/// * path_resolver: Path resolver interface
/// * color_output: Color decor [`ColorDisplay`]
/// ## Returns
/// [`String`] of lines has been rendered
/// ## Example
/// ```rust
/// use ellie_engine::terminal_utils::{print_errors, ColorDisplay, Colors, TextStyles};
/// use ellie_engine::ellie_core::defs::{Cursor, CursorPosition};
///
/// #[derive(Copy, Clone)]
/// struct NoColorTerminal;
///
/// impl ColorDisplay for NoColorTerminal {
///     fn color(&self, _: Colors) -> String {
///         //No modifier
///         String::new()
///     }
///
///     fn text_style(&self, _: TextStyles) -> String {
///         //No modifier
///         String::new()
///     }
/// }
///
/// let my_error = ellie_core::error::error_list::ERROR_S10.clone().build_without_debug(
///     vec![],
///     Cursor {
///         range_start: CursorPosition(0, 0),
///         range_end: CursorPosition(0, 10)
///     }
/// );
///
/// let error = print_errors(
///     &vec![ my_error ],
///     | path | {
///         "A total error".to_owned()
///     },
///    false,
///     | path | {
///         "C:VirtualPath".to_owned()
///     },
///     NoColorTerminal
/// );
/// println!("{}", error);
///
pub fn print_errors<E, F, T>(
    errors: &Vec<error::Error>,
    file_reader: E,
    show_debug_lines: bool,
    path_resolver: F,
    color_output: T,
) -> String
where
    E: FnOnce(String) -> String + Clone + Copy + Sized,
    F: FnOnce(String) -> String + Clone + Copy + Sized,
    T: ColorDisplay,
{
    let mut output = String::new();
    for error in errors {
        output += &format!(
            "\n{}{}[{:#04x}{}]{}: {}{}{}\n",
            color_output.color(Colors::Red),
            error.title,
            error.code,
            if show_debug_lines {
                format!(" - {}", error.debug_message)
            } else {
                "".to_string()
            },
            color_output.color(Colors::Reset),
            color_output.color(Colors::Cyan),
            error.builded_message.builded,
            color_output.color(Colors::Reset),
        );
        let file_content = file_reader(error.path.clone());
        let mut line_space = error.pos.range_start.0.to_string().len() + 1;
        if let Some(refr) = error.reference_block.clone() {
            let ref_file_content = file_reader(refr.1.clone());
            if line_space < refr.0.range_start.0.to_string().len() + 1 {
                line_space = refr.0.range_start.0.to_string().len() + 1;
            }
            output += &render_code_block(
                path_resolver(refr.1.clone()),
                refr.0,
                ref_file_content,
                error.reference_message.clone(),
                line_space,
                true,
                true,
                color_output,
            );
            output += "\n"
        }
        output += &render_code_block(
            path_resolver(error.path.clone()),
            error.pos,
            file_content,
            "".to_string(),
            line_space,
            false,
            true,
            color_output,
        );
        output += "\n";
        output += &format!(
            "{}{}[?]{} ╞ Check online error repo for more info {}{}{}\n",
            generate_blank(line_space - 2),
            color_output.color(Colors::Magenta),
            color_output.color(Colors::Reset),
            color_output.color(Colors::Green),
            format!(
                "https://www.ellie-lang.org/errorIndex.html#{:#04x}",
                error.code
            ),
            color_output.color(Colors::Reset),
        );

        if error.full_assist || error.semi_assist {
            if cfg!(feature = "ellie_assist") {
                output += &format!(
                    "{}{}[{}]{} ╞ {} assistment available type '{}ellie{} {}assist{} {}{}{}' for request assist\n",
                    generate_blank(line_space - 2),
                    color_output.color(Colors::Yellow),
                    if error.semi_assist {
                        "◆"
                    } else {
                        "✓"
                    },
                    color_output.color(Colors::Reset),
                    if error.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    color_output.color(Colors::Green),
                    color_output.color(Colors::Reset),
                    color_output.color(Colors::Yellow),
                    color_output.color(Colors::Reset),
                    color_output.color(Colors::Green),
                    "(0x00)",
                    color_output.color(Colors::Reset),
                );
            } else {
                output += &format!(
                    "{}{}[x]{} ╞ {} assistment available but {}ellie_assist{} feature is not enabled\n",
                    generate_blank(line_space - 2),
                    color_output.color( Colors::Yellow),
                    color_output.color( Colors::Reset),
                    if error.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    color_output.color( Colors::Red),
                    color_output.color( Colors::Reset),
                );
            }
        }

        if error.code == 0x00 && errors.len() > 2 {
            output += &format!(
                "\n{}{}{} other error omitted\n",
                color_output.color(Colors::Red),
                errors.len() - 2,
                color_output.color(Colors::Reset)
            );
            break;
        }
    }
    output
}

pub fn print_warnings<E, F, T: ColorDisplay>(
    warnings: &Vec<warning::Warning>,
    file_reader: E,
    path_resolver: F,
    color_output: T,
) -> String
where
    E: FnOnce(String) -> String + Clone + Copy + Sized,
    F: FnOnce(String) -> String + Clone + Copy + Sized,
{
    let mut output = String::new();
    for warning in warnings {
        output += &format!(
            "\n{}Warning[{}]{}: {}{}{}\n",
            color_output.color(Colors::Yellow),
            warning.title,
            color_output.color(Colors::Reset),
            color_output.color(Colors::Cyan),
            warning.builded_message.builded,
            color_output.color(Colors::Reset),
        );

        let file_content = file_reader(warning.path.clone());
        let mut line_space = warning.pos.range_end.0.to_string().len() + 1;

        if let Some(refr) = warning.reference_block.clone() {
            let ref_file_content = file_reader(refr.1.clone());
            if line_space < refr.0.range_start.0.to_string().len() + 1 {
                line_space = refr.0.range_start.0.to_string().len() + 1;
            }
            output += &render_code_block(
                path_resolver(refr.1.clone()),
                refr.0,
                ref_file_content,
                warning.reference_message.clone(),
                line_space,
                true,
                false,
                color_output,
            );
            output += "\n"
        }
        output += &render_code_block(
            path_resolver(warning.path.clone()),
            warning.pos,
            file_content,
            "".to_string(),
            line_space,
            false,
            false,
            color_output,
        );
        output += "\n";

        output += &format!(
            "{}{}[?]{} ╞ Check online standard rules repo for more info {}{}{}\n",
            generate_blank(line_space - 2),
            color_output.color(Colors::Magenta),
            color_output.color(Colors::Reset),
            color_output.color(Colors::Green),
            format!(
                "https://www.ellie-lang.org/standardRules.html#{:#04x}",
                warning.code
            ),
            color_output.color(Colors::Reset),
        );

        if warning.full_assist || warning.semi_assist {
            if cfg!(feature = "ellie_assist") {
                output += &format!(
                    "{}{}[{}]{} ╞ {} assistment available type '{}ellie{} {}assist{} {}{}{}' for request assist\n",
                    generate_blank(line_space - 2),
                    color_output.color(Colors::Yellow),
                    if warning.semi_assist {
                        "◆"
                    } else {
                        "✓"
                    },
                    color_output.color(Colors::Reset),
                    if warning.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    color_output.color(Colors::Green),
                    color_output.color(Colors::Reset),
                    color_output.color(Colors::Yellow),
                    color_output.color(Colors::Reset),
                    color_output.color(Colors::Green),
                    "(0x00)",
                    color_output.color(Colors::Reset),
                );
            } else {
                output += &format!(
                    "{}{}[x]{} ╞ {} assistment available but {}ellie_assist{} feature is not enabled\n",
                    generate_blank(line_space - 2),
                    color_output.color(Colors::Yellow),
                    color_output.color(Colors::Reset),
                    if warning.semi_assist {
                        "Semi"
                    } else {
                        "Full"
                    },
                    color_output.color(Colors::Red),
                    color_output.color(Colors::Reset),
                );
            }
        }
    }
    output
}
