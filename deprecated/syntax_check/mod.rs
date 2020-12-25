mod typos;
pub fn arrow(line: usize) -> String {
    let mut s = String::with_capacity(line);
    for e in 0..line {
        if e == line - 1 {
            s.push_str("^");
        } else {
            s.push_str(" ");
        }
    }
    return s;
}

fn get_letter(letter: String, index: usize, turn: bool) -> String {
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


pub fn collect(code: String) -> typos::Compiled {
    let file_path : Vec<String> = env::args().collect();
    let lines = code.split("\n");

    let mut comment_line = false;
    
    let mut error : Vec<typos::SyntaxError> = Vec::new();
    let mut variables : Vec<typos::Variable> = Vec::new();
    let mut functions : Vec<typos::Function> = Vec::new();
    let mut conditions : Vec<typos::Conditions> = Vec::new();
    let mut callers : Vec<typos::Caller> = Vec::new();

    let available_types = vec!["number", "bool", "string", "array"];

    let mut variable = typos::VariableStarted {
        started: false,
        name: false,
        typo: false,
        typo_started: false,
        value: false,
        value_started: false,
        name_text: "".to_string(),
        type_text: "".to_string(),
        value_text: "".to_string(),
        qual_exist: false,
        ignore_qual: false,
        string_started: false,
        string_closed: false,
        line_started: 0
    };

    let mut function = typos::FunctionStarted {
        started: false,
        name: false,
        code: "".to_string(),
        name_text: "".to_string(),
        paramater_text: "".to_string(),
        paramater_type_text: "".to_string(),
        parameter_started: false,
        has_paramater_type: true,
        has_paramater_text: false,
        parameters: Vec::new(),
        bracket_started: false,
        bracked_ended: false,
        return_started: false,
        return_text: "".to_string(),
        bracket_collect: false,
        ignore_brace: false,
        ignore_qual: false,
        line_started: 0,
    };

    let mut condition = typos::ConditionStarted {
        started: false,
        cond_type: "".to_string(),
        collecting_inner_code: false,
        await_if: false,
        collect_if: "".to_string(),
        condition_started: false,
        condition_brace_started: false,
        condition_brace: "".to_string(),
        inner_code: "".to_string(),
        chains: Vec::new(),
        ignore_qual: false,
        ignore_brace: false,
        if_started: 0,
        line_started: 0  
    };
    

    let unexpected_standalone_chars = ["v", "fn", ":", "=", "{", "}", ")", "("];
    let nameables = "qwertyuopasdfghjkklizxcvbnm";
    let _valueables = "qwertyuopasdfghjkklizxcvbnm[]\"'1234567890";
    let _resulteables = "qwertyuopasdfghjkklizxcvbnm[]\"'1234567890,";

    //fn next_is_element(all: String, el: &str) -> bool  {
    //    let mut correct = true;
    //    #[allow(unused_variables)]
    //    for (colmn, letter_char) in all.split("").enumerate() {
    //        if letter_char == el {
    //            correct = true;
    //            break;
    //        } else if letter_char != " " {
    //            correct = false;
    //            break;
    //        }
    //    };
    //    return correct;
    //}

    //fn slice_string(text: String, from: usize, to: usize) -> String {
    //    let sliced_vec : Vec<char> = text.chars().skip(from).take(to).collect();
    //    let s: String = sliced_vec.into_iter().collect();
    //    return s;
    //}

    //fn get_charity(text: &str, line: usize) -> String {
    //    return text.chars().nth(line).unwrap().to_string();
    //}

    fn clean_up(qe: String) -> String {
        let mut cleaned = "".to_string();
        for (_colmn_f, l_char) in qe.chars().enumerate() {
            //let colmn = colmn_f;
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

    fn remove_all_white_spaces(text: String) -> String {
        let mut cleaned = "".to_string();
        for (colmn, l_char) in text.chars().enumerate() {
            let next_char = &get_letter(text.to_string(), colmn, true).to_owned();
            if next_char != " " && l_char.to_string() != " " {
                cleaned = cleaned + &l_char.to_string();
            }
        }
        return cleaned;
    }

    for (line_number_fixed, line) in lines.enumerate() {
        let line_number = line_number_fixed + 1;
        comment_line = false;
        for (colmn, l_char) in line.chars().enumerate() {
            
            let letter_char = &l_char.to_string();
            let last_char = &get_letter(line.to_string(), colmn, false).to_owned();
            let next_char = &get_letter(line.to_string(), colmn, true).to_owned();

            if variable.started { //DONE
                if variable.ignore_qual {
                    variable.ignore_qual = false;
                } else if variable.value {
                    if letter_char == ";" {
                        variables.push(typos::Variable {
                            variable_type: variable.clone().type_text,
                            value: typos::get_variable(variable.clone()),
                            line: typos::Line {
                                line: usize::try_from(variable.clone().line_started).unwrap(),
                                colmn: 0
                            }
                        });
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
                            qual_exist: false,
                            ignore_qual: false,
                            string_started: false,
                            string_closed: false,
                            line_started: 0
                        };
                    } else {
                        if (!variable.string_started && variable.string_closed) && unexpected_standalone_chars.iter().any(|&i| i==letter_char) && next_char == " " {
                            error.push(typos::SyntaxError {
                                line: line.to_string(),
                                file: file_path[1].clone(),
                                pos: typos::Line {
                                    line: line_number,
                                    colmn: colmn
                                },
                                message: "[154] Unexpected Token: ".to_string() + &letter_char.to_string()
                            })
                        } else {
                            if letter_char == "\"" {
                                variable.string_started = true;
                            }
                            variable.value_text = variable.value_text + letter_char; 
                        }
                    }
                } else if variable.typo {
                    if letter_char == "=" {
                        if available_types.iter().any(|&i| i==variable.type_text) {
                            variable.value = true;
                        } else {
                            let new_colmn;
                            if colmn > variable.type_text.len() {
                                new_colmn = colmn - variable.type_text.len(); 
                            } else {
                                new_colmn = variable.type_text.len() - colmn;
                            }
                            error.push(typos::SyntaxError {
                                line: line.to_string(),
                                file: file_path[1].clone(),
                                pos: typos::Line {
                                    line: line_number,
                                    colmn: new_colmn - 1
                                },
                                message: "[175] Unknown Type: ".to_string() + &variable.type_text.to_string()
                            });
                        }
                    } else if nameables.contains(letter_char) {
                        variable.type_text = variable.type_text + letter_char
                    } else if last_char != ":" && letter_char == " " && nameables.contains(next_char) {
                        error.push(typos::SyntaxError {
                            line: line.to_string(),
                            file: file_path[1].clone(),
                            pos: typos::Line {
                                line: line_number + 1,
                                colmn: colmn + 1
                            },
                            message: "[188] Unexpected Token: ".to_string() + &letter_char.to_string()
                        })
                    }
                
                } else if variable.name {
                    if letter_char == ":" {
                        if variable.name_text.contains(" ") {
                            //let index = variable.name_text.find(" ").unwrap();
                            error.push(typos::SyntaxError {
                                line: line.to_string(),
                                file: file_path[1].clone(),
                                pos: typos::Line {
                                    line: line_number,
                                    colmn: colmn
                                },
                                message: "[203] Unexpected Token: ".to_string() + &letter_char.to_string()
                            })
                        } else {
                            variable.typo = true;
                        }
                    } else if nameables.contains(letter_char) {
                        variable.name_text = variable.name_text + letter_char
                    } else if (nameables.contains(last_char) && nameables.contains(next_char)) && letter_char == " " {
                        error.push(typos::SyntaxError {
                            line: line.to_string(),
                            file: file_path[1].clone(),
                            pos: typos::Line {
                                line: line_number,
                                colmn: colmn
                            },
                            message: "[218] Unexpected Token: ".to_string() + &letter_char.to_string()
                        })
                    }
                }       
            } else if function.started { //DONE
                if function.ignore_qual {
                    function.ignore_qual = false;
                } else if function.bracket_started {

                    if function.bracket_collect {
                        if letter_char == "}" && !function.ignore_brace {
                            if next_char != ";" {
                                error.push(typos::SyntaxError {
                                    line: line.to_string(),
                                    file: file_path[1].clone(),
                                    pos: typos::Line {
                                        line: line_number,
                                        colmn: colmn
                                    },
                                    message: "[268] SyntaxError: Expected ';'".to_string()
                                });
                            } else {
                                let cleaned_code = clean_up(function.code.clone());
                                let b: Vec<char> = cleaned_code.chars().skip(1).collect();
                                let fixed_code: String = if cleaned_code.starts_with("\n") { b.into_iter().collect() } else { cleaned_code };

                                let compiled_code = collect(clean_up(fixed_code));
                                if compiled_code.errors.len() != 0 {
                                    for err in compiled_code.errors {
                                        error.push(err);
                                    }
                                } else {
                                    functions.push(typos::Function {
                                        name: function.name_text.clone(),
                                        parameters: function.parameters.clone(),
                                        code: compiled_code,
                                        line: typos::Line {
                                            line: line_number,
                                            colmn: colmn
                                        }
                                    });
                                    function = typos::FunctionStarted {
                                        started: false,
                                        name: false,
                                        code: "".to_string(),
                                        name_text: "".to_string(),
                                        paramater_text: "".to_string(),
                                        paramater_type_text: "".to_string(),
                                        parameter_started: false,
                                        has_paramater_type: true,
                                        has_paramater_text: false,
                                        parameters: Vec::new(),
                                        bracket_started: false,
                                        bracked_ended: false,
                                        return_started: false,
                                        return_text: "".to_string(),
                                        bracket_collect: false,
                                        ignore_brace: false,
                                        ignore_qual: false,
                                        line_started: 0,
                                    };
                                }
                                function.bracket_started = false;
                            }
                        } else {
                            function.code = function.code + letter_char;
                        }
                        
                        if letter_char == "{" {
                            function.ignore_brace = true;
                        }
                        if letter_char == "}" {
                            function.ignore_brace = false;
                        }
                        
                    } else if letter_char == "{" && !function.bracket_collect {
                        function.bracket_collect = true;
                    }
                } else if function.parameter_started {
                    if !function.has_paramater_type {
                        if letter_char == " " {
                        } else if letter_char == ")" {
                            if available_types.iter().any(|&i| i == function.paramater_type_text) {
                                function.parameters.push(typos::Parameter {
                                    name: function.paramater_text.clone(),
                                    typ: function.paramater_type_text.clone()
                                });
                                function.has_paramater_type = true;
                                function.paramater_type_text = "".to_string();
                                function.paramater_text = "".to_string();
                                function.parameter_started = true;
                                function.bracket_started = true;
                            } else {
                                error.push(typos::SyntaxError {
                                    line: line.to_string(),
                                    file: file_path[1].clone(),
                                    pos: typos::Line {
                                        line: line_number,
                                        colmn: colmn - 1
                                    },
                                    message: "[268] Unknown Type: ".to_string() + &function.paramater_type_text.to_string()
                                });
                            }
                        } else if letter_char == "," {
                            println!("We got a variable user wants to add more!");
                            function.has_paramater_text = false;
                            function.has_paramater_type = true;
                        } else if nameables.contains(letter_char) {
                            function.paramater_type_text = function.paramater_type_text + letter_char;
                        } else if last_char != "}" && letter_char != ";" {
                            error.push(typos::SyntaxError {
                                line: line.to_string(),
                                file: file_path[1].clone(),
                                pos: typos::Line {
                                    line: line_number,
                                    colmn: colmn
                                },
                                message: "[378] Unexpected Token: ".to_string() + &letter_char.to_string()
                            })
                        }
                    } else if !function.has_paramater_text {
                        if letter_char == ")" {
                            function.parameter_started = true;
                            function.bracket_started = true;
                            if !function.has_paramater_text && function.paramater_text != "" {
                                error.push(typos::SyntaxError {
                                    line: line.to_string(),
                                    file: file_path[1].clone(),
                                    pos: typos::Line {
                                        line: line_number,
                                        colmn: colmn
                                    },
                                    message: "[286] Unexpected Token: ".to_string() + &letter_char.to_string()
                                })
                            }
                        } else if letter_char == "," {

                        } else if letter_char == ":" {
                            function.has_paramater_type = false;
                        } else if nameables.contains(letter_char) {
                            function.paramater_text = function.paramater_text + letter_char;
                        } else if ( last_char != ":" && last_char != ",") || letter_char == " " && nameables.contains(next_char) {
                            error.push(typos::SyntaxError {
                                line: line.to_string(),
                                file: file_path[1].clone(),
                                pos: typos::Line {
                                    line: line_number,
                                    colmn: colmn
                                },
                                message: "[305] Unexpected Token: ".to_string() + &letter_char.to_string()
                            })
                        }
                    }
                } else if function.name {
                    if letter_char == " " {

                    } else if nameables.contains(letter_char) {
                        function.name_text = function.name_text + letter_char;
                    } else if letter_char == "(" { // && last_char != "n"
                        if function.name_text == "" {
                            let new_colmn;
                            if colmn > variable.type_text.len() {
                                new_colmn = colmn - function.paramater_type_text.len(); 
                            } else {
                                new_colmn = function.paramater_type_text.len() - colmn;
                            }
                            
                            error.push(typos::SyntaxError {
                                line: line.to_string(),
                                file: file_path[1].clone(),
                                pos: typos::Line {
                                    line: line_number,
                                    colmn: new_colmn - 1
                                },
                                message: "[393] Expected Identifier: ".to_string() + &function.paramater_type_text.to_string()
                            });
                        }
                        function.name = false;
                        function.parameter_started = true;
                    }
                }
            } else if condition.started {
                let cleaned_cond_type = remove_all_white_spaces(condition.cond_type.clone());
                if condition.ignore_qual {
                    if cleaned_cond_type == "else" ||  cleaned_cond_type == "elsi" {
                        condition.ignore_brace = false;
                    }
                    condition.ignore_qual = false;
                } else {

                    if condition.condition_brace_started {
                        if !condition.ignore_brace && letter_char == "{" && (condition.cond_type == "else" && (last_char != " " || last_char != "")) {
                            let new_colmn;
                            if colmn > function.paramater_type_text.len() {
                                new_colmn = colmn - function.paramater_type_text.len(); 
                            } else {
                                new_colmn = function.paramater_type_text.len() - colmn;
                            }
                            error.push(
                                typos::SyntaxError {
                                    line: line.to_string(),
                                    file: file_path[1].clone(),
                                    pos: typos::Line {
                                        line: line_number,
                                        colmn: new_colmn - 1
                                    },
                                    message: "[486] SyntaxError: Unexpected Token".to_string()
                                }
                            );
                        } else if !condition.ignore_brace && letter_char == "}" {
                            println!("END OF IF");
                            condition.condition_brace_started = false;
                            let cloned_cond = condition.clone();
                            condition.collecting_inner_code = false;
                            condition.chains.push(
                                typos::Condition {
                                    condition_type: remove_all_white_spaces(cloned_cond.cond_type),
                                    given_conditions: remove_all_white_spaces(cloned_cond.condition_brace),
                                    code: cloned_cond.inner_code,
                                    start_line: condition.line_started,
                                    end_line: condition.if_started as i32
                                }
                            );
                            condition.ignore_brace = true;
                            condition.cond_type = "".to_string();
                        } else {
                            if letter_char == "{" {
                                condition.ignore_brace = true;
                            } else if letter_char == "}" {
                                condition.ignore_brace = false;
                            }
                            condition.inner_code = condition.inner_code + letter_char;
                        }
                    } else if !condition.condition_started {

                        if letter_char == " " {
                            println!("condition_started");
                            condition.condition_started = true;
                        } else {
                            let new_colmn;
                            if colmn > function.paramater_type_text.len() {
                                new_colmn = colmn - function.paramater_type_text.len(); 
                            } else {
                                new_colmn = function.paramater_type_text.len() - colmn;
                            }
                            error.push(
                                typos::SyntaxError {
                                    line: line.to_string(),
                                    file: file_path[1].clone(),
                                    pos: typos::Line {
                                        line: line_number,
                                        colmn: new_colmn - 1
                                    },
                                    message: "[427] SyntaxError: Expected whitespace".to_string()
                                }
                            );
                        } 
                    } else if letter_char == "{" { //&& !condition.ignore_brace ?????????
                        if condition.condition_brace == "" {
                            let new_colmn;
                            if colmn > function.paramater_type_text.len() {
                                new_colmn = colmn - function.paramater_type_text.len(); 
                            } else {
                                new_colmn = function.paramater_type_text.len() - colmn;
                            }
                            error.push(
                                typos::SyntaxError {
                                    line: line.to_string(),
                                    file: file_path[1].clone(),
                                    pos: typos::Line {
                                        line: line_number,
                                        colmn: new_colmn - 1
                                    },
                                    message: "[532] SyntaxError: Expected Condition".to_string()
                                }
                            );
                        } else {
                            condition.collecting_inner_code = true;
                            condition.condition_brace_started = true;
                        }
                    } else {
                        condition.condition_brace = condition.condition_brace + letter_char;
                        println!("collect brace cond: {}", letter_char);
                    }
                }
            }

            if !comment_line && letter_char != "\\" && next_char != "\\" && !condition.collecting_inner_code {
                
                if letter_char == ";" && condition.started && condition.ignore_brace {
                    conditions.push(
                        typos::Conditions {
                            chains: condition.chains,
                            start_line: condition.line_started
                        }
                    );
                    condition = typos::ConditionStarted {
                        started: false,
                        cond_type: "".to_string(),
                        await_if: false,
                        collecting_inner_code: false,
                        collect_if: "".to_string(),
                        condition_started: false,
                        condition_brace_started: false,
                        condition_brace: "".to_string(),
                        inner_code: "".to_string(),
                        chains: Vec::new(),
                        ignore_qual: false,
                        ignore_brace: false,
                        if_started: 0,
                        line_started: 0  
                    }
                } else if condition.started && condition.ignore_brace && condition.await_if {
                    //println!("Collecting else if or else: {}", removeAllWhiteSpaces(condition.cond_type.clone()));
                    let cleaned_cond_type = remove_all_white_spaces(condition.cond_type.clone());
                    if (cleaned_cond_type == "else" && cleaned_cond_type.contains("{")) || cleaned_cond_type == "elsi" {
                        if  condition.ignore_brace {
                            if cleaned_cond_type == "else" {
                                println!("Start Collect");
                                condition.condition_brace = "".to_string();
                                condition.collecting_inner_code = true;
                                condition.condition_brace_started = true;
                                condition.started = true;
                                if next_char == "{" {
                                    condition.ignore_brace = true;
                                } else {
                                    condition.ignore_brace = false;    
                                }
                                condition.ignore_qual = true;
                                condition.await_if = false;
                                condition.inner_code = "".to_string();
                                condition.if_started = line_number as i32;
                            } else if cleaned_cond_type == "elsi" {
                                //Disabled for now

                                condition.condition_brace = "".to_string();
                                condition.line_started = line_number as i32;
                                condition.started = true;
                                condition.ignore_qual = false;
                                condition.ignore_brace = false;
                                condition.if_started = line_number as i32;
                            }
                        } else {
                            condition.ignore_brace = true;
                        }
                        println!("it confirmed its a {}", remove_all_white_spaces(condition.cond_type.clone()));
                    } else if (letter_char == "\\" && next_char == "n") || letter_char == "i" || letter_char == " " || letter_char == "s" || letter_char == "l" || letter_char == "e" || letter_char == "f" {
                        condition.cond_type = condition.cond_type + letter_char;
                        condition.if_started = line_number as i32;
                        condition.condition_brace_started = false;
                    } else {

                    }
                } else if condition.started && condition.ignore_brace && letter_char == "e" {
                    println!("This might be else if or else we are waiting correct letters");
                    condition.cond_type = "e".to_string();
                    condition.await_if = true;
                    //condition
                } else if letter_char == "i" && next_char == "f" {
                    if !condition.started {
                        condition.cond_type = "if".to_string();
                        condition.line_started = line_number as i32;
                        condition.started = true;
                        condition.ignore_qual = true;
                        condition.if_started = line_number as i32;
                    }
                } else  if last_char == "}" && condition.started {
                    println!("a new if chain started");
                } else if !variable.started && !function.started && !condition.started {
                    if letter_char == "v" && (last_char == "" || last_char == " " || last_char == ";" || last_char == "}") && (next_char == " " || line.to_string().len() == 2) {
                        variable.started = true;
                        variable.ignore_qual = true;
                        variable.name = true;
                        variable.line_started = line_number as i32;
                    } else if letter_char == "f" && next_char == "n" && (last_char == " " || last_char == ";" || last_char == "}" || last_char == "") {
                        function.started = true;
                        function.ignore_qual = true;
                        function.name = true;
                        function.line_started = line_number as i32;
                    }
                }
            } else {
                comment_line = true;
            }
        }
    }

    if variable.started {
        println!("{}:{}:{}", file_path[1], variable.line_started + 1, 0);
        println!("Error");
    }

    return typos::Compiled {
        variables,
        functions,
        conditions,
        callers,
        errors: error
    }
}

pub fn syntax_check(code: String) -> typos::SyntaxError {
    let file_path : Vec<String> = env::args().collect();
    let lines = code.split("\n");
    let mut pos = typos::Line {
        line: 0,
        colmn: 0
    };
    let mut lineq = "".to_string();
    let mut brace_open: bool = false;
    let mut quest_open: bool = false;
    let mut _expected_c : bool = false;
    for (line_number, line) in lines.enumerate() {
        for (colmn, letter_char) in line.split("").enumerate() {
            let _last_char = &get_letter(line.to_string(), colmn, false).to_owned();
            let _next_char = &get_letter(line.to_string(), colmn, true).to_owned();
            if letter_char == "\"" {
                quest_open = !quest_open;
                pos.colmn = colmn + 1;
                pos.line = line_number;
                lineq = line.to_string();
            }
            if letter_char == "{" && !quest_open {
                brace_open = true;
                pos.colmn = colmn + 1;
                pos.line = line_number;
                lineq = line.to_string();
            } else if letter_char == "}" && !quest_open{
                brace_open = false;
            }

            //if letter_char == "}" && next_char != ";" {
            //    expectedC = true;
            //    pos.colmn = i32::try_from(colmn).unwrap();
            //    pos.line = i32::try_from(line_number).unwrap();
            //    lineq = line.to_string();
            //}
        }
    }
    if brace_open {
        return typos::SyntaxError {
            line: lineq,
            file: file_path[1].clone(),
            pos,
            message: "[627] SyntaxError: Unexpected end of input".to_string()
        };
    } else if quest_open {
        return typos::SyntaxError {
            line: lineq,
            file: file_path[1].clone(),
            pos,
            message: "[634] SyntaxError: Unexpected end of input".to_string()
        };
    } else {
        return typos::SyntaxError {
            line: lineq,
            file: file_path[1].clone(),
            pos,
            message: "false".to_string()
        };
    }
}

