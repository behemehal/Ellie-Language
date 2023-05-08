use core::ops::{Index, IndexMut};

use alloc::borrow::ToOwned;
use alloc::string::{String, ToString};
use alloc::vec;
use alloc::vec::Vec;
use rand;
use serde::{Deserialize, Serialize};

use crate::definite::types::operator::Operators;
use crate::definite::types::operator::{
    assignment_operator_to_string, comparison_operator_to_string, logical_operator_to_string,
    ArithmeticOperators, AssignmentOperators,
};
use crate::{defs, error};

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
        || value == "enum"
        || value == "enumField"
        || value == "var"
        || value == "v"
        || value == "c"
        || value == "const"
        || value == "d"
        || value == "co"
        || value == "constructor"
        || value == "import"
        || value == "g"
        || value == "getter"
        || value == "s"
        || value == "setter"
        || value == "new"
        || (value == "array" && !allow_core_naming)
        || (value == "collective" && !allow_core_naming)
        || (value == "cloak" && !allow_core_naming)
        || (value == "string" && !allow_core_naming)
        || (value == "char" && !allow_core_naming)
        || (value == "int" && !allow_core_naming)
        || (value == "float" && !allow_core_naming)
        || (value == "bool" && !allow_core_naming)
        || (value == "dyn" && !allow_core_naming)
        || (value == "void" && !allow_core_naming)
        || (value == "null" && !allow_core_naming)
        || (value == "nullAble" && !allow_core_naming)
}

pub fn generate_hash_usize() -> usize {
    rand::random::<usize>()
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
    Path,
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
        ReliableNameRanges::Path => {
            "QWERTYUIOPASDFGHJKLIZXCVBNMqwertyuıopasdfghjklizxcvbnm0123456789_@!?"
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

pub fn colapseable_operator(parent: Operators, child: Operators) -> bool {
    match parent {
        Operators::ComparisonType(_) => {
            match child {
                // 2 == 2 == 2 This is faulty chain for ellie but should collapse like this (2 == 2) == 2
                Operators::ComparisonType(_) => true,
                // 2 == 2 && 2 == 2 : (2 == 2) && (2 == 2)
                Operators::LogicalType(_) => true,
                // 2 == 1 + 1 : 2 == (1 + 1)
                Operators::ArithmeticType(_) => false,
                // v a : int = 2;
                // 4 == a *= 2 : 4 == (a *= 2)
                Operators::AssignmentType(_) => false,
                Operators::Null => unreachable!(),
            }
        }
        Operators::LogicalType(_) => {
            //This idiot should mind his own business
            match child {
                Operators::LogicalType(_) => true,
                Operators::Null => unreachable!(),
                _ => false,
            }
        }
        Operators::ArithmeticType(parent_inner) => {
            match child {
                // 2 + 2 == 4 : (2 + 2) == 4
                // steal 2 from ComparisonType
                Operators::ComparisonType(_) => true,
                // 2 + 2 && 1  : (2 + 2) && 1
                Operators::LogicalType(_) => true,
                // here is the problem
                Operators::ArithmeticType(child_inner) => {
                    match parent_inner {
                        ArithmeticOperators::Addition => {
                            match child_inner {
                                ArithmeticOperators::Addition => true,
                                ArithmeticOperators::Subtraction => true,

                                //Basic math
                                ArithmeticOperators::Multiplication => false,
                                ArithmeticOperators::Exponentiation => false,
                                ArithmeticOperators::Division => false,
                                ArithmeticOperators::Modulus => false,

                                ArithmeticOperators::Null => unreachable!(),
                            }
                        }
                        ArithmeticOperators::Subtraction => match child_inner {
                            ArithmeticOperators::Addition => false,
                            ArithmeticOperators::Subtraction => false,

                            //Basic math
                            ArithmeticOperators::Multiplication => true,
                            ArithmeticOperators::Exponentiation => true,
                            ArithmeticOperators::Division => true,
                            ArithmeticOperators::Modulus => true,

                            ArithmeticOperators::Null => unreachable!(),
                        },
                        ArithmeticOperators::Null => unreachable!(),
                        //other operators has high priorty
                        _ => match child_inner {
                            ArithmeticOperators::Addition => false,
                            ArithmeticOperators::Subtraction => false,

                            ArithmeticOperators::Multiplication => true,
                            ArithmeticOperators::Exponentiation => true,
                            ArithmeticOperators::Division => true,
                            ArithmeticOperators::Modulus => true,

                            ArithmeticOperators::Null => unreachable!(),
                        },
                    }
                }
                Operators::AssignmentType(_) => todo!(),
                Operators::Null => todo!(),
            }
        }
        Operators::AssignmentType(parent_inner) => {
            match child {
                // 2 + 2 == 4 : (2 + 2) == 4
                // steal 2 from ComparisonType
                Operators::ComparisonType(_) => true,
                // 2 + 2 && 1  : (2 + 2) && 1
                Operators::LogicalType(_) => true,
                // here is the problem
                Operators::ArithmeticType(child_inner) => {
                    match parent_inner {
                        AssignmentOperators::AdditionAssignment => {
                            match child_inner {
                                ArithmeticOperators::Addition => false,
                                ArithmeticOperators::Subtraction => false,

                                //Basic math
                                ArithmeticOperators::Multiplication => true,
                                ArithmeticOperators::Exponentiation => true,
                                ArithmeticOperators::Division => true,
                                ArithmeticOperators::Modulus => true,

                                ArithmeticOperators::Null => unreachable!(),
                            }
                        }
                        AssignmentOperators::SubtractionAssignment => match child_inner {
                            ArithmeticOperators::Addition => false,
                            ArithmeticOperators::Subtraction => false,

                            //Basic math
                            ArithmeticOperators::Multiplication => true,
                            ArithmeticOperators::Exponentiation => true,
                            ArithmeticOperators::Division => true,
                            ArithmeticOperators::Modulus => true,

                            ArithmeticOperators::Null => unreachable!(),
                        },
                        AssignmentOperators::Null => unreachable!(),
                        //other operators has high priorty
                        _ => match child_inner {
                            ArithmeticOperators::Addition => true,
                            ArithmeticOperators::Subtraction => true,

                            ArithmeticOperators::Multiplication => false,
                            ArithmeticOperators::Exponentiation => false,
                            ArithmeticOperators::Division => false,
                            ArithmeticOperators::Modulus => false,

                            ArithmeticOperators::Null => unreachable!(),
                        },
                    }
                }
                Operators::AssignmentType(_) => false,
                Operators::Null => unreachable!(),
            }
        }
        Operators::Null => unreachable!(),
    }
}

pub fn operator_priority(operator: &str) -> usize {
    match operator {
        "=" => 2,
        "!=" => 2,
        "==" => 2,
        ">" => 2,
        "<" => 2,
        ">=" => 2,
        "<=" => 2,

        "&&" => 3,
        "||" => 3,

        "+" => 1,
        "-" => 1,
        "*" => 1,
        "/" => 1,
        "%" => 1,
        _ => unreachable!(),
    }
}

pub fn operator_control(
    operator: Operators,
    first: String,
    second: String,
    path: String,
    pos: defs::Cursor,
) -> Option<crate::error::Error> {
    let first = first.as_str();
    let second = second.as_str();
    let operator = match operator {
        Operators::ComparisonType(operator) => match operator {
            crate::definite::types::operator::ComparisonOperators::Equal
            | crate::definite::types::operator::ComparisonOperators::NotEqual => {
                match (first, second) {
                    ("bool", "bool")
                    | ("string", "string")
                    | ("int", "int")
                    | ("float", "float")
                    | ("float", "double")
                    | ("double", "double")
                    | ("double", "float") => None,
                    _ => Some(comparison_operator_to_string(operator)),
                }
            }
            crate::definite::types::operator::ComparisonOperators::GreaterThan
            | crate::definite::types::operator::ComparisonOperators::LessThan
            | crate::definite::types::operator::ComparisonOperators::GreaterThanOrEqual
            | crate::definite::types::operator::ComparisonOperators::LessThanOrEqual => {
                match (first, second) {
                    ("int", "int")
                    | ("float", "float")
                    | ("float", "double")
                    | ("double", "float")
                    | ("double", "double") => None,
                    _ => Some(comparison_operator_to_string(operator)),
                }
            }
            crate::definite::types::operator::ComparisonOperators::Null => unreachable!(),
        },
        Operators::LogicalType(operator) => match operator {
            crate::definite::types::operator::LogicalOperators::And
            | crate::definite::types::operator::LogicalOperators::Or => match (first, second) {
                ("bool", "bool") => None,
                _ => Some(logical_operator_to_string(operator)),
            },
            crate::definite::types::operator::LogicalOperators::Null => unreachable!(),
        },
        Operators::ArithmeticType(operator) => match operator {
            crate::definite::types::operator::ArithmeticOperators::Addition => {
                match (first, second) {
                    ("int", "int") | ("int", "double") | ("int", "byte") | ("int", "float") => None,
                    ("float", "float") | ("float", "double") | ("float", "int") => None,
                    ("double", "double") | ("double", "int") | ("double", "float") => None,
                    ("byte", "byte") | ("byte", "int") => None,
                    ("string", "string")
                    | ("string", "int")
                    | ("string", "float")
                    | ("string", "double")
                    | ("string", "bool")
                    | ("string", "byte") => None,
                    _ => Some("Addition"),
                }
            }
            crate::definite::types::operator::ArithmeticOperators::Subtraction
            | crate::definite::types::operator::ArithmeticOperators::Multiplication
            | crate::definite::types::operator::ArithmeticOperators::Exponentiation
            | crate::definite::types::operator::ArithmeticOperators::Division
            | crate::definite::types::operator::ArithmeticOperators::Modulus => {
                match (first, second) {
                    ("int", "int") | ("int", "byte") => None,
                    ("float", "float") | ("float", "double") | ("float", "int") => None,
                    ("double", "double") | ("double", "int") | ("double", "float") => None,
                    ("byte", "byte") | ("byte", "int") => None,
                    _ => Some("Assignment"),
                }
            }
            crate::definite::types::operator::ArithmeticOperators::Null => unreachable!(),
        },
        Operators::AssignmentType(operator) => match operator {
            crate::definite::types::operator::AssignmentOperators::Assignment => None,
            crate::definite::types::operator::AssignmentOperators::AdditionAssignment => {
                match (first, second) {
                    ("int", "int")
                    | ("float", "float")
                    | ("float", "double")
                    | ("float", "int")
                    | ("float", "byte")
                    | ("double", "double")
                    | ("double", "float")
                    | ("double", "int")
                    | ("double", "byte")
                    | ("byte", "byte")
                    | ("byte", "int")
                    | ("string", "string")
                    | ("string", "int")
                    | ("string", "float")
                    | ("string", "double")
                    | ("string", "byte") => None,
                    _ => Some("AdditionAssignment"),
                }
            }
            crate::definite::types::operator::AssignmentOperators::SubtractionAssignment
            | crate::definite::types::operator::AssignmentOperators::MultiplicationAssignment
            | crate::definite::types::operator::AssignmentOperators::DivisionAssignment
            | crate::definite::types::operator::AssignmentOperators::ModulusAssignment
            | crate::definite::types::operator::AssignmentOperators::ExponentiationAssignment => {
                match (first, second) {
                    ("int", "int")
                    | ("float", "float")
                    | ("float", "double")
                    | ("float", "int") => None,
                    ("double", "double")
                    | ("double", "float")
                    | ("double", "int")
                    | ("byte", "byte")
                    | ("byte", "int") => None,
                    _ => Some(assignment_operator_to_string(operator)),
                }
            }
            crate::definite::types::operator::AssignmentOperators::Null => unreachable!(),
        },
        Operators::Null => unreachable!(),
    };
    match operator {
        Some(operator_string) => Some(error::error_list::ERROR_S52.clone().build_with_path(
            vec![
                error::ErrorBuildField {
                    key: "opType".to_owned(),
                    value: operator_string.to_string(),
                },
                error::ErrorBuildField {
                    key: "target".to_owned(),
                    value: first.to_owned(),
                },
                error::ErrorBuildField {
                    key: "value".to_owned(),
                    value: second.to_owned(),
                },
            ],
            alloc::format!("{}:{}:{}", file!().to_owned(), line!(), column!()),
            path,
            pos,
        )),
        None => None,
    }
}

pub fn is_operators_chainable(target: Operators, current: Operators) -> bool {
    match target {
        Operators::ComparisonType(_) => match current {
            Operators::LogicalType(_) => true,
            Operators::ArithmeticType(_) => true,
            Operators::AssignmentType(_) => true,
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
            Operators::AssignmentType(_) => true,
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
    pub page_hashs: (Vec<usize>, Vec<usize>),
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
    fn get_hash(&self) -> usize;
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
    pub fn find_page(&mut self, hash: usize) -> Option<&mut T> {
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
    pub fn find_page_and_idx(&mut self, hash: usize) -> Option<(&mut T, usize)> {
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
