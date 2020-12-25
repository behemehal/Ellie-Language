fn fix_number(current: u32, minus: u32) -> bool {
    let c = Wrapping(current);
    let m = Wrapping(minus);
    return u32::MAX != ((c - m).0);
}

fn get_charity(text: &str, line: usize) -> String {
    return text.chars().nth(line).unwrap().to_string();
}

fn next_is_element(all: String, el: &str) -> bool  {
    let mut correct = true;
    #[allow(unused_variables)]
    for (colmn, letter_char) in all.split("").enumerate() {
        if letter_char == el {
            correct = true;
            break;
        } else if letter_char != " " {
            correct = false;
            break;
        }
    };
    return correct;
}

for (line_number, line) in lines.enumerate() {       

    for (colmn, letter_char) in line.split("").enumerate() {
        //let letter = letter_char.to_string();
        if letter_char == "v" && line[colmn..colmn+1].to_string() == " " { // new variable started v [name] : [type] = [value]
            variable.started = true;
            variable.line_started = line_number as i32;
        } else if variable.started {
            if (variable.started && variable.name_text == "") && letter_char == " " { // Writing variable name
                variable.name = true;
            } else if variable.ignore_qual {
                variable.ignore_qual = false
            } else if variable.name  && next_is_element(line[colmn..].to_string(), ":") { //(letter_char == " " || letter_char == ":") { // Writing variable name complete
                if variable.name_text == "" { // user passed without typing name
                    println!("{}:{}:{}", file_path[1], line_number + 1, colmn + 1);
                    println!("{}", line);
                    println!("{}", arrow(colmn));
                    println!("!Unexpected Token: {}", letter_char.to_string());
                    break;
                } else {
                    variable.name = false;
                    variable.typo = true;
                    if letter_char == " " {
                        variable.ignore_qual = true
                    }
                }
            } else if variable.name {
                if nameables.contains(letter_char) {
                    variable.name_text = variable.name_text + letter_char;
                } else if letter_char == ":" || letter_char == " " {
                    if letter_char == " " {
                        variable.ignore_qual = true
                    }
                    variable.name = false;
                    variable.typo = true;
                } else {
                    println!("{}:{}:{}", file_path[1], line_number + 1, colmn + 1);
                    println!("{}", line);
                    println!("{}", arrow(colmn));
                    println!("[59] Unexpected Token: {}", letter_char.to_string());
                    break;
                }
            } else if variable.typo && next_is_element(line[colmn-1..].to_string(), "=") {//(letter_char == " " || letter_char == "=") {
                if variable.type_text == "" {
                    println!("{}:{}:{}", file_path[1], line_number + 1, colmn + 1);
                    println!("{}", line);
                    println!("{}", arrow(colmn));
                    println!("Type Cannot Be Empty");
                    break;
                } else {
                    variable.typo = false;
                    if letter_char == " " {
                        variable.ignore_qual = true
                    }
                }
            } else if variable.typo {
                if valueables.contains(letter_char) {
                    variable.typo_started = true;
                    variable.type_text = variable.type_text + letter_char;
                } else if letter_char == "=" || variable.type_text != "" {
                    if available_types.iter().any(|&i| i==variable.type_text) {
                        variable.typo = false;
                        variable.typo_started = false;
                        variable.value = true;
                        if letter_char == " " {
                            variable.ignore_qual = true
                        }
                    } else {
                        println!("{}:{}:{}", file_path[1], line_number + 1, colmn + 1);
                        println!("{}", line);
                        println!("{}", arrow(colmn));
                        println!("Unknown Type: {}", variable.type_text);
                        break;
                    }
                    
                } else if variable.typo_started {
                    println!("{}:{}:{}", file_path[1], line_number + 1, colmn + 1);
                    println!("{}", line);
                    println!("{}", arrow(colmn));
                    println!("Unexpected Token: {}", letter_char.to_string());
                    break;
                }
            } else if variable.value {
                if letter_char == ";" {
                    let caught_variable = variable.clone();
                    variables.push(typos::get_variable(caught_variable));
                    variable = typos::VariableStarted {
                        started: false,
                        name: false,
                        typo: false,
                        typo_started: false,
                        value: false,
                        value_started: false,
                        name_text: "".to_string(),
                        type_text: "".to_string(),
                        value_text: "".to_string(),
                        ignore_qual: false,
                        string_started: false,
                        string_closed: false,
                        line_started: 0
                    };
                    break;
                } else if resulteables.contains(letter_char) {
                    variable.value_started = true;
                    variable.value_text = variable.value_text + letter_char; 
                } else if variable.value_started {
                    println!("{}:{}:{}", file_path[1], line_number + 1, colmn + 1);
                    println!("{}", line);
                    println!("{}", arrow(colmn));
                    println!("Unexpected Token: {} {}", letter_char.to_string(),  letter_char.to_string().len());
                    break;
                }
            }
        }
    }

    //match letter {
    //    "\n" => print!("line"),
    //    _ => print!("PANIC")
    //}
}