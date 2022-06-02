use core::ops::{Index, IndexMut};

use alloc::string::{String, ToString};
use alloc::vec::Vec;
use rand;
use serde::{Deserialize, Serialize};

use crate::definite::types::operator::Operators;

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
        || value == '0'
        || value == '\\'
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

pub fn is_operators_chainable(target: Operators, current: Operators) -> bool {
    match target {
        Operators::ComparisonType(_) => match current {
            Operators::LogicalType(_) => true,
            _ => false,
        },
        Operators::LogicalType(_) => true,
        Operators::ArithmeticType(_) => match current {
            Operators::LogicalType(_) => true,
            Operators::ArithmeticType(_) => true,
            Operators::ComparisonType(_) => true,
            _ => false,
        },
        Operators::AssignmentType(_) => match current {
            Operators::LogicalType(_) => true,
            _ => false,
        },
        Operators::Null => false,
    }
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageExport<T> {
    pub pages: Vec<T>,
    pub page_hashs: (Vec<usize>, Vec<u64>),
}

impl<T> Index<usize> for PageExport<T> {
    type Output = T;
    fn index<'a>(&'a self, i: usize) -> &'a Self::Output {
        &self.pages[i]
    }
}

impl<T> IndexMut<usize> for PageExport<T> {
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        &mut self.pages[i]
    }
}

pub trait ExportPage {
    fn get_hash(&self) -> u64;
}

impl<T> PageExport<T>
where
    T: ExportPage + core::fmt::Debug,
{
    pub fn new() -> PageExport<T> {
        PageExport {
            pages: Vec::new(),
            page_hashs: (Vec::new(), Vec::new()),
        }
    }

    pub fn into_iter(self) -> impl Iterator<Item = T> {
        self.pages.into_iter()
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.pages.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.pages.iter_mut()
    }

    /// Push page with binary search in mind
    pub fn push_page(&mut self, page: T) {
        //self.page_hashs.0.push(self.pages.len());
        //self.page_hashs.1.push(page.get_hash());
        //match self.page_hashs.1.binary_search(&page.get_hash()) {
        //    Ok(e) => {
        //        panic!("Page hash already exists at index: {}", e);
        //    } // element already in vector @ `pos`
        //    Err(pos) => self.page_hashs.1.insert(pos, page.get_hash()),
        //}
        self.pages.push(page);
    }

    /// Extend pages
    pub fn extend_pages(&mut self, pages: Vec<T>) {
        for page in pages {
            self.push_page(page);
        }
    }

    pub fn nth_mut(&mut self, n: usize) -> Option<&mut T> {
        self.pages.get_mut(n)
    }

    pub fn nth(&self, n: usize) -> Option<&T> {
        self.pages.get(n)
    }

    /// Find page
    /// ## Arguments
    /// * `hash` - page hash
    /// ## Returns
    /// Option<&mut [`Page`]> //Page
    pub fn find_page(&mut self, hash: u64) -> Option<&mut T> {
        self.pages.iter_mut().find(|page| page.get_hash() == hash)
        //match self.page_hashs.1.iter().position(|x| x == &hash) {
        //    Some(index_pos) => {
        //        let page_index = self.page_hashs.0[index_pos];
        //        self.pages.iter_mut().nth(page_index)
        //    }
        //    None => None,
        //}
    }

    /// Find page
    /// ## Arguments
    /// * `hash` - page hash
    /// ## Returns
    /// Option<(&mut [`Page`], usize)> //Page and index
    pub fn find_page_and_idx(&mut self, hash: u64) -> Option<(&mut T, usize)> {
        let pos = self
            .pages
            .iter_mut()
            .position(|page| page.get_hash() == hash);
        match pos {
            Some(index_pos) => Some((self.pages.iter_mut().nth(index_pos).unwrap(), index_pos)),
            None => None,
        }
    }
}
