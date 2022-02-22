use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rand;

/// Response of [`reliable_name_range`] function
/// ## Fields
/// * `reliable` - Is char reliable
/// * `at` - Column of char in character set if its reliable
/// * `found` - Data set column index
pub struct ReliableNameRangeResponse {
    pub reliable: bool,
    pub at: usize,
    pub found: char,
}

pub fn is_escape(value: char) -> bool {
    value == '\''
        || value == '"'
        || value == 'n'
        || value == 'r'
        || value == 't'
        || value == 'b'
        || value == 'f'
        || value == 'v'
        || value == '0'
}

pub fn is_reserved(value: &str, allow_core_naming: bool) -> bool {
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
        
        || (value == "array" && !allow_core_naming)
        || (value == "collective" && !allow_core_naming)
        || (value == "cloak" && !allow_core_naming)
        || (value == "vector" && !allow_core_naming)
        || (value == "string" && !allow_core_naming)
        || (value == "char" && !allow_core_naming)
        || (value == "int" && !allow_core_naming)
        || (value == "float" && !allow_core_naming)
        || (value == "bool" && !allow_core_naming)
        || (value == "dyn" && !allow_core_naming)
        || (value == "void" && !allow_core_naming)
        || (value == "null" && !allow_core_naming)
        || (value == "nullAble" && !allow_core_naming)
        || (value == "function" && !allow_core_naming)
        || (value == "panic" && !allow_core_naming)
        || (value == "createThread" && !allow_core_naming)
        || (value == "cursorPosition" && !allow_core_naming)
        || (value == "error" && !allow_core_naming)
        || (value == "rawMemoryData" && !allow_core_naming)
}

pub fn generate_hash_u64() -> u64 {
    rand::random::<u64>()
}

pub fn generate_hash() -> String {
    alloc::format!(
        "{:?}",
        (0..24)
            .map(|_| { rand::random::<u8>() })
            .collect::<Vec<u8>>()
    )
    .replace(" ", "")
    .replace(",", "")
    .replace("]", "")
    .replace("[", "")
}

/// ReliableNameRanges is a enum indicates which charachter set is to be used
/// VariableName: QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789_
/// Type: QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789
pub enum ReliableNameRanges {
    VariableName,
    Type,
}

/// ReliableNameRange is a function that returns [`ReliableNameRangeResponse`]
/// ## Arguments
/// * `range` - [`ReliableNameRanges`]
/// * `value` - Char to be checked
pub fn reliable_name_range(range: ReliableNameRanges, value: char) -> ReliableNameRangeResponse {
    let variable_range = match range {
        ReliableNameRanges::VariableName => {
            "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789_"
        }
        ReliableNameRanges::Type => {
            "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789"
        }
    };

    let find = variable_range.chars().position(|x| x == value);
    return ReliableNameRangeResponse {
        reliable: find != None,
        at: find.unwrap_or(0),
        found: variable_range
            .chars()
            .nth(find.unwrap_or(0))
            .unwrap_or_default(),
    };
}

/// Trims text from left and right
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

/// Makes text's first letter uppercase
pub fn upper_first_char(line: String) -> String {
    let mut c = line.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

/// Makes text's first letter lowerCase
pub fn lower_first_char(line: String) -> String {
    let mut c = line.chars();
    match c.next() {
        None => String::new(),
        Some(f) => f.to_lowercase().collect::<String>() + c.as_str(),
    }
}

/// Check if the given char is a start of a operator
pub fn is_operator_start(letter_char: char) -> bool {
    letter_char == '&'
        || letter_char == '|'
        || letter_char == '+'
        || letter_char == '-'
        || letter_char == '='
        || letter_char == '!'
        || letter_char == '>'
        || letter_char == '<'
        || letter_char == '*'
        || letter_char == '/'
        || letter_char == '%'
}

#[derive(Debug, Clone)]
pub enum FoundExtended {
    LogicalOperator,
    ComparisonOperator,
    ArithmeticOperator,
    AssignmentOperator,
}

/// Resolve given string to [`FoundExtended`]
pub fn resolve_operator(operator: &str) -> Option<FoundExtended> {
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

    if is_logical_operator(operator) {
        Some(FoundExtended::LogicalOperator)
    } else if is_comparison_operator(operator) {
        Some(FoundExtended::ComparisonOperator)
    } else if is_assignment_operator(operator) {
        Some(FoundExtended::AssignmentOperator)
    } else if is_arithmetic_operator(operator) {
        Some(FoundExtended::ArithmeticOperator)
    } else {
        None
    }
}
