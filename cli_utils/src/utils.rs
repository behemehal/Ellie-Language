use ellie_core::{error, warning};
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
    ByteCodeDebug,
    Nop,
}

#[derive(PartialEq, Eq, Debug)]
pub enum CliOutputType {
    Json,
    ConsoleOutput,
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
