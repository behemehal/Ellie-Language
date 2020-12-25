use crate::alloc::string::{ToString, String};
use crate::alloc::vec::Vec;
use crate::alloc::borrow::ToOwned;

use crate::collectors;
use crate::utils;

pub fn map(code: String) -> collectors::Compiled {
    let lines = code.split("\n");
    let mut compiled = collectors::Compiled {
        items: Vec::new(),
        syntax_errors: Vec::new()
    };
    let mut comment_line = false;

    let mut variable_collect = false;
    let mut function_collect = false;
    let mut condition_collect = false;
    let mut caller_collect = false;

    let mut ignore_char = false;

    let mut modified_variable = collectors::VariableModifiedData {
        variable_name: "".to_string(),
        collect_name: true,
        variable_type: "".to_string(),
        collect_type: false,
        collect_value: "".to_string(),
        variable_value: "".to_string(),
        collect: true,
        position: collectors::variable::PositionOfElement {
            colmn: 0,
            line:0
        }
    };

    let mut modified_condition = collectors::ConditionModifiedData {
        collect: false,
        keyword: "".to_string(),
        collect_keyword: false,
        collect_condition: false,
        condition: "".to_string(),
        inner_code: "".to_string(),
        expect_semicolon: false,
        else_await_brace: false,
        collecting_inner_code: false,
        ignore_inner_brace: false,
        conditional_chain: Vec::new(),
        position: collectors::condition::PositionOfElement {
            colmn: 0,
            line:0
        }
    };

    let mut modified_function = collectors::FunctionModifiedData {
        collect: true,
        function_name: "".to_string(),
        collect_name: true,

        parameters: Vec::new(),
        collect_parameters: false,
        
        parameter_value: "".to_string(),
        parameter_text_collect: false,

        parameter_type: "".to_string(),
        parameter_type_collect: false,

        return_type: "".to_string(),
        collection_arrow: false,
        collect_return_type: false,

        inner_code: "".to_string(),
        collecting_inner_code: false,
        ignore_inner_brace: false,
        position: collectors::function::PositionOfElement {
            colmn: 0,
            line: 0
        }
    };

    let mut modified_caller = collectors::CallerModifiedData {
        collect: false,
        is_function: false,
        is_variable: false,
        is_return: false,
        collected: "".to_string(),
        return_keyword: false,
        returned: "".to_string(),
        function_name: "".to_string(),
        function_parameters: Vec::new(),
        variable_name: "".to_string(),
        variable_value: "".to_string(),
        quote_started: false,
        position: collectors::PositionOfElement {
            colmn: 0,
            line: 0
        }
    };

    for (line_number_fixed, line) in lines.enumerate() {
        let line_number = line_number_fixed + 1;
        comment_line = false;
        for (colmn, l_char) in line.chars().enumerate() {
            let letter_char = &l_char.to_string();
            let last_char = &utils::get_letter(line.to_string(), colmn, false).to_owned();
            let next_char = &utils::get_letter(line.to_string(), colmn, true).to_owned();
            let next_next_char = &utils::get_letter(line.to_string(), colmn + 1, true).to_owned();
      
            if comment_line {

            } else if letter_char == "/" && next_char == "/" {
                comment_line = true;
                break;
            } else if ignore_char {
                ignore_char = false;
            } else if !variable_collect && !function_collect && !condition_collect && !caller_collect {
                if letter_char == "v" && (last_char == "" || last_char == " " || last_char == ";") && (next_char == " " || line.to_string().len() == 2) {
                    variable_collect = true;
                    ignore_char = true; // Bir sonraki karakteri umursamıyoruz çünkü v sonra boşluk gelmeli!!!
                    modified_variable.position = collectors::variable::PositionOfElement {
                        colmn,
                        line: line_number
                    }
                } else if letter_char == "f" && next_char == "n" && (last_char == "" || last_char == " " || last_char == ";") && (next_next_char == " " || line.to_string().len() == 3) {
                    function_collect = true;
                    ignore_char = true;
                    modified_function.position = collectors::function::PositionOfElement {
                        colmn,
                        line: line_number
                    }
                } else if letter_char == "i" && next_char == "f" && (last_char == "" || last_char == " " || last_char == ";") && (next_next_char == " " || line.to_string().len() == 3) {
                    condition_collect = true;
                    ignore_char = true;
                    modified_condition.collect_condition = true;
                    modified_condition.keyword = "if".to_string();
                    modified_condition.position = collectors::condition::PositionOfElement {
                        colmn,
                        line: line_number
                    }
                } else if letter_char != "\r" && letter_char != " " && letter_char != "" {
                    caller_collect = true;
                    modified_caller.position.colmn = colmn;
                    modified_caller.position.line = line_number;
                    modified_caller.collected = letter_char.to_string();
                }
            } else {
                if variable_collect {
                    let finished = collectors::variable_collector(collectors::ModifiedDataTypes::Variable(modified_variable.clone()), letter_char, last_char, next_char, colmn, line_number);
                    if finished.complete {
                        if let collectors::Collected::Variable(data) = finished.collected {
                            compiled.items.push(collectors::CompiledItems::Variable(data));
                            modified_variable.variable_name = "".to_string();
                            modified_variable.collect_name = true;
                            modified_variable.variable_type = "".to_string();
                            modified_variable.collect_type = false;
                            modified_variable.collect_value = "".to_string();
                            modified_variable.variable_value = "".to_string();
                            modified_variable.collect = true;
                            modified_variable.position = collectors::variable::PositionOfElement {
                                colmn: 0,
                                line:0
                            }
                        } else {
                            "";
                        }
                        variable_collect = false;
                    } else {
                        if finished.has_syntax_error {
                            variable_collect = false;
                            compiled.syntax_errors.push(finished.syntax_error);
                        } else {
                            if let collectors::ModifiedDataTypes::Variable(data) = finished.modified_data.clone() {
                                modified_variable = data;
                            } else {
                                "";
                            }
                        }
                    }
                } else if function_collect {
                    let finished = collectors::function_collector(collectors::ModifiedDataTypes::Function(modified_function.clone()), letter_char, last_char, next_char, colmn, line_number);
                    if finished.complete {
                        if let collectors::Collected::Function(data) = finished.collected {
                            compiled.items.push(collectors::CompiledItems::Function(data));
                            modified_function.collect = true;
                            modified_function.function_name = "".to_string();
                            modified_function.collect_name = true;
                            modified_function.parameters = Vec::new();
                            modified_function.collect_parameters = false;
                            modified_function.parameter_value = "".to_string();
                            modified_function.parameter_text_collect = false;
                            modified_function.parameter_type = "".to_string();
                            modified_function.parameter_type_collect = false;
                            modified_function.return_type = "".to_string();
                            modified_function.collection_arrow = false;
                            modified_function.collect_return_type = false;
                            modified_function.inner_code = "".to_string();
                            modified_function.collecting_inner_code = false;
                            modified_function.ignore_inner_brace = false;
                            modified_function.position = collectors::function::PositionOfElement {
                                colmn: 0,
                                line: 0
                            };
                            function_collect = false;
                        } else {
                            "";
                        }
                    } else {
                        if finished.has_syntax_error {
                            function_collect = false;
                            compiled.syntax_errors.push(finished.syntax_error);
                        } else {
                            if let collectors::ModifiedDataTypes::Function(data) = finished.modified_data.clone() {
                                modified_function = data;
                            } else {
                                "";
                            }
                        }
                    }
                } else if condition_collect {
                    let finished = collectors::condition_collector(collectors::ModifiedDataTypes::Condition(modified_condition.clone()), letter_char, last_char, next_char, colmn, line_number);
                    if finished.complete {
                        if let collectors::Collected::Condition(data) = finished.collected {
                            compiled.items.push(collectors::CompiledItems::Condition(data));
                        } else {
                            "";
                        }
                        condition_collect = false;
                    } else {
                        if finished.has_syntax_error {
                            condition_collect = false;
                            compiled.syntax_errors.push(finished.syntax_error);
                        } else {
                            if let collectors::ModifiedDataTypes::Condition(data) = finished.modified_data.clone() {
                                modified_condition = data;
                            } else {
                                "";
                            }
                        }
                    }
                } else if caller_collect {
                    let finished = collectors::caller_collector(collectors::ModifiedDataTypes::Caller(modified_caller.clone()), letter_char, last_char, next_char, colmn, line_number);
                    if finished.complete {
                        if finished.has_syntax_error {
                            caller_collect = false;
                            compiled.syntax_errors.push(finished.syntax_error);
                        } else if let collectors::Collected::Caller(data) = finished.collected {
                            compiled.items.push(collectors::CompiledItems::Callers(data));
                        } else {
                            "";
                        }
                        caller_collect = false;
                        modified_caller = collectors::CallerModifiedData {
                            collect: false,
                            is_function: false,
                            is_variable: false,
                            is_return: false,
                            collected: "".to_string(),
                            return_keyword: false,
                            returned: "".to_string(),
                            function_name: "".to_string(),
                            function_parameters: Vec::new(),
                            variable_name: "".to_string(),
                            variable_value: "".to_string(),
                            quote_started: false,
                            position: collectors::PositionOfElement {
                                colmn: 0,
                                line: 0
                            }
                        };
                    } else {
                        if finished.has_syntax_error {
                            caller_collect = false;
                            compiled.syntax_errors.push(finished.syntax_error);
                        } else {
                            if let collectors::ModifiedDataTypes::Caller(data) = finished.modified_data.clone() {
                                modified_caller = data;
                            } else {
                                "";
                            }
                        }
                    }
                }
            }
        };
    };
    
    if caller_collect || variable_collect || function_collect || condition_collect {
        compiled.syntax_errors.push(collectors::SyntaxError {
            message: "SyntaxError: Unexpected end of input".to_string(),
            pos: collectors::PositionOfElement {
                line: if caller_collect { modified_caller.position.line } else if variable_collect { modified_variable.position.line } else if function_collect { modified_function.position.line } else if condition_collect { modified_condition.position.line } else { 0 },
                colmn: if caller_collect { modified_caller.position.colmn } else if variable_collect { modified_variable.position.colmn } else if function_collect { modified_function.position.colmn } else if condition_collect { modified_condition.position.colmn } else { 0 }
            }
        });
    }
    return compiled;
}