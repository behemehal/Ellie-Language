pub mod terminal_colors;

#[repr(C)]
pub struct ReliableNameRangeResponse {
    pub reliable: bool,
    pub at: usize,
    pub found: char,
}

#[repr(C)]
pub enum ReliableNameRanges {
    VariableName,
    Type,
    FunctionName,
}

#[no_mangle]
pub extern "C" fn is_opearators(value: &str) -> bool {
    let operators = "|&";
    operators.contains(&value)
}

#[no_mangle]
pub extern "C" fn reliable_name_range(
    range: ReliableNameRanges,
    value: String,
) -> ReliableNameRangeResponse {
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

#[no_mangle]
pub extern "C" fn get_letter(letter: String, index: usize, turn: bool) -> String {
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

#[no_mangle]
pub extern "C" fn get_line(code: String, line: usize) -> String {
    let v: Vec<&str> = code.split('\n').collect();
    v[line].to_string()
}

#[no_mangle]
pub extern "C" fn arrow(line: usize, range: usize) -> String {
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

#[no_mangle]
pub extern "C" fn trim_good(line: String) -> String {
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
