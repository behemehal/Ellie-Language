use alloc::vec::Vec;
use alloc::string::{String, ToString};

pub mod terminal_colors;

pub struct ReliableNameRangeResponse {
    pub reliable: bool,
    pub at: usize,
    pub found: char,
}

pub enum ReliableNameRanges {
    VariableName,
    Type,
    FunctionName,
}

pub fn is_opearators(value: &str) -> bool {
    let operators = "|&";
    operators.contains(&value)
}

pub fn is_errors_same(first: crate::error::Error, second: crate::error::Error) -> bool {
    first.code == second.code && first.message == second.message && first.pos.range_start.0 == second.pos.range_start.0
}

pub fn zip_errors(errors: Vec<crate::error::Error>) -> Vec<crate::error::Error> {
    let mut clone_errors: Vec<crate::error::Error> = errors.clone();
    let mut zipped_errors : Vec<crate::error::Error> = Vec::new();

    for i in 0..clone_errors.len() {
        if i != 0 {
            if is_errors_same(clone_errors[i - 1].clone(), clone_errors[i].clone()) {
                let last_error = clone_errors.clone()[i - 1].clone();
                clone_errors[i].pos.range_start = last_error.pos.range_start;

                for field in 0..last_error.builded_message.fields.len() {
                    clone_errors[i].builded_message.fields[field].value = last_error.builded_message.fields[field].value.clone() + " " + &clone_errors[i].builded_message.fields[field].value;
                }

                if i == errors.len() - 1 || !is_errors_same(clone_errors[i].clone(), clone_errors[i + 1].clone()) {
                    clone_errors[i].builded_message =  crate::error::Error::build(clone_errors[i].message.clone(), clone_errors[i].builded_message.fields.clone());
                    zipped_errors.push(clone_errors[i].clone())
                }
            } else {
                zipped_errors.push(clone_errors[i].clone())
            }

        } else if errors.len() > 1 && !is_errors_same(clone_errors[0].clone(), clone_errors[1].clone()) || errors.len() == 1 {
            zipped_errors.push(clone_errors[0].clone());
        }
    }

    zipped_errors
}

pub fn reliable_name_range(range: ReliableNameRanges, value: String) -> ReliableNameRangeResponse {
    match range {
        ReliableNameRanges::VariableName => {
            let variable_range = "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789";
            let find = value.split("").position(|x| !variable_range.contains(&x));
            return ReliableNameRangeResponse {
                reliable: find == None,
                at: find.unwrap_or(0),
                found: value
                    .chars()
                    .nth(if let Some(e) = find { e - 1 } else { 0 })
                    .unwrap_or_default(),
            };
        }
        ReliableNameRanges::Type => {
            let variable_range =
                "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789<>";
            let find = value.split("").position(|x| !variable_range.contains(&x));
            return ReliableNameRangeResponse {
                reliable: find == None,
                at: find.unwrap_or(0),
                found: value
                    .chars()
                    .nth(if let Some(e) = find { e - 1 } else { 0 })
                    .unwrap_or_default(),
            };
        }
        ReliableNameRanges::FunctionName => {
            let variable_range = "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm";
            let find = value.split("").position(|x| !variable_range.contains(&x));
            return ReliableNameRangeResponse {
                reliable: find == None,
                at: find.unwrap_or(0),
                found: value
                    .chars()
                    .nth(if let Some(e) = find { e - 1 } else { 0 })
                    .unwrap_or_default(),
            };
        }
    }
}

pub fn get_letter(letter: String, index: usize, turn: bool) -> String {
    if turn {
        // Bir sonraki karakter
        if index == letter.len() {
            "".to_string()
        } else {
            let sliced: Vec<char> = letter.chars().skip(index + 1).take(1).collect();
            if sliced.is_empty() {
                "".to_string()
            } else {
                sliced[0].to_string()
            }
        }
    } else if index == 0 {
        "".to_string()
    } else {
        let sliced: Vec<char> = letter.chars().skip(index - 1).take(1).collect();
        if sliced.is_empty() {
            "".to_string()
        } else {
            sliced[0].to_string()
        }
    }
}

pub fn get_line(code: String, line: usize) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    v[line].to_string()
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

pub fn trim_good(line: String) -> String {
    let mut fixed = String::new();
    for i in 0..line.len() {
        let last = line.chars().nth(if i == 0 { 0 } else { i - 1 });
        let current = line.chars().nth(i).unwrap();
        if let Some(q) = last {
            if q != ' ' || current != ' ' || i == 0 {
                fixed += &current.to_string();
            }
        } else {
            fixed += &current.to_string();
        }
    }
    fixed
}

pub fn lower_first_char(line: String) -> String {
    let mut c = line.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}