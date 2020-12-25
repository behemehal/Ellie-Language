use crate::alloc::string::{String, ToString};
use crate::alloc::vec::Vec;
use crate::alloc::borrow::ToOwned;
use crate::collectors;

pub fn arrow(line: usize) -> String {
    let mut s = String::with_capacity(line);
    if line == 0 {
        s = "^".to_string();
    } else {
        for e in 0..line {
            if e == line - 1 {
                s.push_str("^");
            } else {
                s.push_str(" ");
            }
        }
    }
    return s;
}

/*

if let collectors::value_collector::ValueTypes:: = value {
*/


/*
pub fn compare_redefine_value(target: collectors::value_collector::ValueTypes, value: collectors::value_collector::ValueTypes, runtime: runtime::types::RuntimeOptions) -> collectors::value_collector::ValueTypes {
    if let collectors::value_collector::ValueTypes::Bool(_) = found.data.value {

    } else if let collectors::value_collector::ValueTypes::DotQuery(e) = value {

    } else if let collectors::value_collector::ValueTypes::Number(_) = value {

    } else if let collectors::value_collector::ValueTypes::Object(_) = value {

    } else if let collectors::value_collector::ValueTypes::Operator(_) = value {

    } else if let collectors::value_collector::ValueTypes::String(_) = value {
    
    } else if let collectors::value_collector::ValueTypes::Variable(e) = value {

    }
}
*/

pub fn compare_contains(e: String, q: String) -> Result<bool, usize> {
    let mut unfit = 1;
    let mut has_unfit = false;
    for (i, ch) in q.chars().enumerate() {
        if !e.contains(ch) {
            unfit = i;
            has_unfit = true;
            break;
        }
    };
    if has_unfit { Err(unfit) } else { Ok(true) }
}

pub fn scratch_start(q: String) -> String {
    let mut left = "".to_string();
    let mut start_scratch = false;
    for char in q.chars() {
        if char.to_string() != " " && !start_scratch {
            start_scratch = true;
            left = left.to_string() + &char.to_string()
        } else if start_scratch {
            left = left.to_string() + &char.to_string()
        }
    }
    return left;
}

pub fn get_letter(letter: String, index: usize, turn: bool) -> String {
    if turn { // Bir sonraki karakter
        if index == letter.len() {
            return "".to_string();
        } else {
            let sliced: Vec<char> = letter.chars().skip(index + 1).take(1).collect();
            return if sliced.len() == 0 { "".to_string() } else { sliced[0].to_string()};
        }
    } else {
        if index == 0 || index == 1 {
            return "".to_string();
        } else {
            let sliced: Vec<char> = letter.chars().skip(index - 1).take(1).collect();
            return if sliced.len() == 0 { "".to_string() } else { sliced[0].to_string()};
        }
    }
}

pub fn get_type_of_value_type(value: collectors::value_collector::ValueTypes) -> String {
    if let collectors::value_collector::ValueTypes::Variable(_) = value {
        return "variable".to_string();
    } else if let collectors::value_collector::ValueTypes::Number(_) = value {
        return "number".to_string();
    } else if let collectors::value_collector::ValueTypes::DotQuery(_) = value {
        return "dotquery".to_string();
    } else if let collectors::value_collector::ValueTypes::Operator(_) = value {
        return "Operator".to_string();
    } else if let collectors::value_collector::ValueTypes::Collective(_) = value {
        return "collective".to_string();
    } else if let collectors::value_collector::ValueTypes::String(_) = value {
        return "string".to_string();
    } else if let collectors::value_collector::ValueTypes::Bool(_) = value {
        return "bool".to_string();
    } else {
        return "false".to_string();
    }
}



pub fn clean_up(qe: String) -> String {
    let mut cleaned = "".to_string();
    for (_colmn_f, l_char) in qe.chars().enumerate() {
        let letter_char = &l_char.to_string();
        //let last_char = &get_letter(qe.to_string(), colmn, false).to_owned();
        //let next_char = &get_letter(qe.to_string(), colmn, true).to_owned();
        if letter_char != "\r" {
            cleaned = cleaned + letter_char;
        } else {
            cleaned = cleaned + "\n";
        }
    }
    return cleaned;
}

pub fn remove_all_white_spaces(text: String) -> String {
    let mut cleaned = "".to_string();
    for (colmn, l_char) in text.chars().enumerate() {
        let next_char = &get_letter(text.to_string(), colmn, true).to_owned();
        if (next_char != " " || next_char != "") && l_char.to_string() != " " {
            cleaned = cleaned + &l_char.to_string();
        }
    }
    return cleaned;
}

pub fn all_of_whitespaces(text: String) -> bool {
    let mut all_of_whitespaces = true;
    for e in text.chars() {
        if e.to_string() != " ".to_string() {
            all_of_whitespaces = false;
            break;
        }
    }
    all_of_whitespaces
}

pub fn get_line(file: String,linenm: usize) -> String {
    let lines = file.split("\n");
    let mut found = "".to_string();
    for (line_number_fixed, line) in lines.enumerate() {
        if linenm == 0 && line_number_fixed == 0 {
            found = line.to_string();
            break;
        }
        if line_number_fixed == linenm - 1 {
            found = line.to_string();
            break;
        }
    }
    return found;
}

/*
pub fn print_clear(val: crate::runtime::runtime_variable::ValueTypes) -> String {
    if let crate::runtime::runtime_variable::ValueTypes::TypeNumber(variable_inner_data) = val {
        return (if variable_inner_data.positive {"Positive"} else {"Negative"}).to_string() + (" Number: ") + &(variable_inner_data.value.to_string());
    } else if let crate::runtime::runtime_variable::ValueTypes::TypeString(variable_inner_data) = val {
        return "String: ".to_string() + &variable_inner_data.value;
    } else if let crate::runtime::runtime_variable::ValueTypes::TypeArray(variable_inner_data) = val {
        let mut value = (&"Array: [").to_string();
        //for (index, item) in variable_inner_data.value.iter().enumerate() {
        //    value = value + "\n    " + &(index).to_string() + " - " + &(item.value); 
        //}
        value = value + "\n]";
        return value;
    } else if let crate::runtime::runtime_variable::ValueTypes::TypeBool(variable_inner_data) = val {
        return "Boolean: ".to_string() + &variable_inner_data.value.to_string();
    } else if let crate::runtime::runtime_variable::ValueTypes::TypeFunction(variable_inner_data) = val {
        return "[".to_string() + &(if variable_inner_data.dynamic {"DynamicFunction"} else {"Function"}).to_string()  + ":" + &variable_inner_data.name + " > " + &variable_inner_data.type_return + "]";
        //println!("[{}: {} > {}]", 
        //    (if found_function_data.dynamic {"DynamicFunction"} else {"Function"}).to_string(),
        //    found_function_data.name,
        //    found_function_data.type_return
        //);
    } else {
        return "Undefined".to_string();
    }
}
*/