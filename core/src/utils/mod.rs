use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rand;

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

pub fn is_operators(value: &str) -> bool {
    let operators = "|&";
    operators.contains(&value)
}

pub fn is_reserved(value: &str) -> bool {
    value == "fn"
        || value == "class"
        || value == "if"
        || value == "else"
        || value == "v"
        || value == "c"
        || value == "d"
        || value == "co"
        || value == "import"
        || value == "get"
        || value == "set"
        || value == "new"
}

pub fn generate_hash() -> String {
    alloc::format!(
        "{:02x?}",
        (0..24)
            .map(|_| { rand::random::<u8>() })
            .collect::<Vec<u8>>()
    )
    .replace(" ", "")
    .replace(",", "")
    .replace("]", "")
    .replace("[", "")
}

pub fn reliable_name_range(range: ReliableNameRanges, value: String) -> ReliableNameRangeResponse {
    match range {
        ReliableNameRanges::VariableName => {
            let variable_range =
                "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789_";
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
            let variable_range = "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxc_vbnm";
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

pub fn trim_good(line: String) -> String {
    let mut fixed = String::new();
    for i in 0..line.len() {
        let last = line.chars().nth(if i == 0 { 0 } else { i - 1 });
        if let Some(current) = line.chars().nth(i) {
            if let Some(q) = last {
                if q != ' ' || current != ' ' || i == 0 {
                    fixed += &current.to_string();
                }
            } else {
                fixed += &current.to_string();
            }
        }
    }
    fixed
}

pub fn upper_first_char(line: String) -> String {
    let mut c = line.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

pub fn lower_first_char(line: String) -> String {
    let mut c = line.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

pub enum FoundExtended {
    Reference,
    BracketReference,
    LogicalOperator,
    ComparisonOperator,
    ArithmeticOperator,
    AssignmentOperator,
    FunctionCall,
}

pub fn is_extended(letter_char: &str, next_char: &str) -> Option<FoundExtended> {
    pub fn is_logical_operator(value: &str) -> bool {
        value == "&&" || value == "||"
    }

    pub fn is_comparison_operator(value: &str) -> bool {
        value == "=="
            || value == "!="
            || value == ">"
            || value == "<"
            || value == ">="
            || value == "<="
    }

    pub fn is_arithmetic_operator(value: &str) -> bool {
        value == "+"
            || value == "-"
            || value == "*"
            || value == "**"
            || value == "/"
            || value == "%"
    }

    pub fn is_assignment_operator(value: &str) -> bool {
        value == "="
            || value == "+="
            || value == "-="
            || value == "*="
            || value == "/="
            || value == "%="
            || value == "**="
    }

    if letter_char == "." {
        Some(FoundExtended::Reference)
    } else if letter_char == "[" {
        Some(FoundExtended::BracketReference)
    } else if is_logical_operator(letter_char)
        || is_logical_operator(&(letter_char.to_string() + &next_char))
    {
        Some(FoundExtended::LogicalOperator)
    } else if is_comparison_operator(letter_char)
        || is_comparison_operator(&(letter_char.to_string() + &next_char))
    {
        Some(FoundExtended::ComparisonOperator)
    } else if is_assignment_operator(letter_char)
        || is_assignment_operator(&(letter_char.to_string() + &next_char))
    {
        Some(FoundExtended::AssignmentOperator)
    } else if is_arithmetic_operator(letter_char)
        || is_arithmetic_operator(&(letter_char.to_string() + &next_char))
    {
        Some(FoundExtended::ArithmeticOperator)
    } else if letter_char == "(" {
        Some(FoundExtended::FunctionCall)
    } else {
        None
    }
}
