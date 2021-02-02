use crate::alloc::string::{ToString, String};
use crate::alloc::vec::Vec;
pub mod return_collector;
pub mod variable_definier;
pub mod value_collector;
pub mod function_caller;
pub mod condition;
pub mod variable;
pub mod function;


#[derive(Debug, Clone , PartialEq, Eq)]
pub enum Callers {
    VariableDefinier(variable_definier::VariableDefinier),
    FunctionCaller(function_caller::FunctionCaller),
    ReturnCaller(return_collector::ReturnCaller)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum CompiledItems {
    Variable(variable::Variable),
    Condition(condition::Condition),
    Function(function::Function),
    Callers(Callers)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Compiled {
    pub items: Vec<CompiledItems>,
    pub syntax_errors: Vec<SyntaxError>,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct PositionOfElement {
    pub colmn: usize,
    pub line: usize
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct VariableStarted {
    pub keyword_collected: bool,
    pub collect_name: bool,
    pub collect_type: bool,
    pub collect_value: bool,
    pub collect: String,
    pub position: PositionOfElement
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct SyntaxError {
    pub pos: PositionOfElement,
    pub message: String
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct VariableModifiedData {
    pub variable_name: String,
    pub collect_name: bool,
    pub variable_type: String,
    pub collect_type: bool,
    pub variable_value: String,
    pub collect_value: String,
    pub collect: bool,
    pub position: variable::PositionOfElement,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ConditionModifiedData {
    pub collect: bool,
    pub expect_semicolon: bool,
    
    pub keyword: String,
    pub collect_keyword: bool,

    pub collect_condition: bool,
    pub condition: String,

    pub inner_code: String,
    pub else_await_brace: bool,
    pub collecting_inner_code: bool,
    pub ignore_inner_brace: bool,

    pub conditional_chain: Vec<condition::ConditionChain>,
    pub position: condition::PositionOfElement,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct CallerModifiedData {
    pub collect: bool,
    pub is_function: bool,
    pub is_variable: bool,
    pub is_return: bool,
    pub collected: String,

    pub return_keyword: bool,
    pub returned: String,

    pub variable_name: String,
    pub variable_value: String,

    pub function_name: String,
    pub function_parameters: Vec<String>,

    pub quote_started: bool,
    pub position: PositionOfElement,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct FunctionModifiedData {
    pub collect: bool,
    pub function_name: String,
    pub collect_name: bool,
    pub parameters: Vec<function::Parameter>,
    pub collect_parameters: bool,

    pub parameter_value: String,
    pub parameter_text_collect: bool,

    pub parameter_type: String,
    pub parameter_type_collect: bool,

    pub return_type: String,
    pub collection_arrow: bool,
    pub collect_return_type: bool,

    pub inner_code: String,
    pub collecting_inner_code: bool,
    pub ignore_inner_brace: bool,
    
    pub position: function::PositionOfElement,
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum ModifiedDataTypes {
    Variable(VariableModifiedData),
    Function(FunctionModifiedData),
    Condition(ConditionModifiedData),
    Caller(CallerModifiedData),
    Null(bool)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum Collected {
    Variable(variable::Variable),
    Function(function::Function),
    Condition(condition::Condition),
    Caller(Callers),
    Null(bool)
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct CollectorResponse {
    pub complete: bool,
    pub has_syntax_error: bool,
    pub syntax_error: SyntaxError,
    pub collected: Collected,
    pub modified_data: ModifiedDataTypes
}

pub fn variable_collector(modified_data: ModifiedDataTypes,letter_char: &String, last_char: &String, next_char: &String, colmn: usize, line: usize) -> CollectorResponse {
    let nameables = "qwertyuopasdfghjkklizxcvbnm";

    let mut has_syntax_error = false;
    let mut syntax_error : SyntaxError = SyntaxError {
        pos: PositionOfElement {
            line: 0,
            colmn: 0
        },
        message: "".to_string()
    };

    if let ModifiedDataTypes::Variable(qdata) = modified_data {
        let mut data = qdata;
        if data.collect_type {
            if (letter_char == " " && next_char == " ") || (letter_char == " " && next_char == "=") {

            } else if letter_char == "=" {
                data.variable_type = data.collect_value;
                data.collect_value = "".to_string();
                data.collect_type = false;
            } else if nameables.contains(letter_char) {
                data.collect_value = data.collect_value + letter_char;
            } else {
                if (last_char != ":" && letter_char != " ") || (!nameables.contains(letter_char) && data.collect_value != "") {
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[124] SyntaxError: Unexpected Token ".to_string() + "'" + letter_char + "'"
                    }
                }
            }
        } else if data.collect_name { // Variable ismi daha alınmadı
            if letter_char == " " && (next_char == " " || next_char == ":") {
            } else if letter_char == ":" {
                data.variable_name = data.collect_value;
                data.collect_name = false;
                data.collect_type = true;
                data.collect_value = "".to_string();
            } else if nameables.contains(letter_char) { //Char variable ismi olabilir o yüzden topla
                data.collect_value = data.collect_value + letter_char;
            } else {
                has_syntax_error = true;
                syntax_error = SyntaxError {
                    pos: PositionOfElement {
                        colmn: colmn + 1,
                        line
                    },
                    message: "[109] SyntaxError: Unexpected identifier ".to_string() + "'" + next_char + "'"
                }
            }
        } else {
            if letter_char == ";" {
                data.collect = false;
            } else {
                data.variable_value = data.variable_value + letter_char;
            }
        }

        if data.collect {
            return CollectorResponse {
                complete: false,
                has_syntax_error,
                syntax_error,
                collected: Collected::Variable(
                    variable::Variable {
                        type_of: "String".to_string(),
                        name: "variable_name".to_string(),
                        value: value_collector::ValueTypes::None,
                        pos: variable::PositionOfElement {
                            line: 0,
                            colmn: 0
                        }
                }),
                modified_data: ModifiedDataTypes::Variable(data)
            };
        } else {
            let collected_value = value_collector::valueCollector(&data.variable_value, 0, 0);
            let errors = collected_value.clone().error;
            if errors.len() == 0 {
                return CollectorResponse {
                    complete: true,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Variable(
                        variable::Variable {
                            type_of: data.variable_type,
                            name: data.variable_name,
                            value: collected_value.data.clone(),
                            pos: data.position
                    }),
                    modified_data: ModifiedDataTypes::Null(false)
                };
            } else {
                has_syntax_error = true;
                syntax_error = errors.first().unwrap().to_owned();
                return CollectorResponse {
                    complete: false,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Null(false),
                    modified_data: ModifiedDataTypes::Null(false)
                };
            }

        }
    } else {
        return CollectorResponse {
            complete: false,
            has_syntax_error,
            syntax_error,
            collected: Collected::Null(false),
            modified_data: ModifiedDataTypes::Null(false)
        };
    }
}

pub fn function_collector(modified_data: ModifiedDataTypes,letter_char: &String, _last_char: &String, next_char: &String, colmn: usize, line: usize) -> CollectorResponse {
    let nameables = "qwertyuopasdfghjkklizxcvbnm";
    let mut has_syntax_error = false;
    let mut syntax_error : SyntaxError = SyntaxError {
        pos: PositionOfElement {
            line: 0,
            colmn: 0
        },
        message: "".to_string()
    };

    if let ModifiedDataTypes::Function(qdata) = modified_data {
        let mut data = qdata;
        
        if data.collecting_inner_code {
            if letter_char == "}" && !data.ignore_inner_brace {
                data.collect = false;
                data.collecting_inner_code = false;
                data.ignore_inner_brace = true;
            } else {
                data.inner_code = data.inner_code + letter_char;
                if letter_char == "{" {
                    data.ignore_inner_brace = true;
                } else if letter_char == "}" {
                    data.ignore_inner_brace = false;
                }
            }
        } else if data.collect_return_type {
            if data.return_type == "" && letter_char == " " {

            } else if data.return_type != "" && letter_char == " " && (next_char != " " && next_char != "{") {
                /*
                    Bu syntax catcher şunu yapıyor: 
                    fn ae (e: string) > e e
                                          ^
                    Ama bunda hata çıkarmamalı
                    fn ae (e: string) > e {
                                          ^
                */
                has_syntax_error = true;
                syntax_error = SyntaxError {
                    pos: PositionOfElement {
                        colmn: colmn + 1,
                        line
                    },
                    message: "[234] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'"
                }
            } else if letter_char == "{" {
                if !data.collection_arrow {
                    /*
                        Bu syntax catcher şunu yapıyor: 
                        fn ae (e: string) {
                                          ^
                        Kullanıcı dönüş tipi belirlemedi
                    */
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[234] SyntaxError: Expected return type ".to_string()
                    }
                } else {
                    data.collect_return_type = false;
                    data.collecting_inner_code = true;
                }
            } else if letter_char == ">" {
                data.collection_arrow = true;
            } else if data.collection_arrow {
                if nameables.contains(letter_char) {
                    data.return_type = data.return_type + letter_char;
                }
                // Araya giren yabancı karakterleri type doğrulama algoritması bozar
                // else {
                //   
                //        Bu syntax catcher şunu yapıyor: 
                //        fn ae(e: string) > st!
                //                             ^
                //   
                //    has_syntax_error = true;
                //    syntax_error = SyntaxError {
                //        pos: PositionOfElement {
                //            colmn: colmn + 1,
                //            line
                //        },
                //        message: "[254] SyntaxError: Unexpected Token ".to_string() + "'" + letter_char + "'"
                //    }
                //}
            } else {
                    /*
                        Bu syntax catcher şunu yapıyor: 
                        fn ae(e: string) > st!
                                             ^
                    */
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[270] SyntaxError: Expected '>' Got ".to_string() + "'" + letter_char + "'"
                    }
            }
        } else if data.collect_parameters {
            if data.parameter_type_collect {

                if (data.parameter_type == "" || next_char == " " || next_char == ",") && letter_char == " " {

                } else if data.parameter_type != "" && letter_char == " " && (next_char != " " && next_char != "," && next_char != ")") {
                    /*
                        Bu syntax catcher şunu yapıyor: 
                        fn ae (e e)
                                 ^
                        Ama bunda hata çıkarmamalı

                        fn de (e :
                                 ^
                    */
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[325] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'"
                    }
                } else if letter_char == ")" {
                    if data.parameter_type != "" && data.parameter_value != "" {
                        data.parameters.push(
                            function::Parameter {
                                name: data.parameter_value,
                                type_of: data.parameter_type
                            }
                        );
                    };
                    data.parameter_text_collect = false;
                    data.parameter_type_collect = false;
                    data.collect_parameters = false;
                    data.collect_return_type = true;
                    data.parameter_value = "".to_string();
                    data.parameter_type = "".to_string();
                } else if letter_char == "," {
                    data.parameters.push(
                        function::Parameter {
                            name: data.parameter_value,
                            type_of: data.parameter_type
                        }
                    );
                    data.parameter_text_collect = true;
                    data.parameter_type_collect = false;
                    data.parameter_value = "".to_string();
                    data.parameter_type = "".to_string();
                } else if nameables.contains(letter_char) {
                    data.parameter_type = data.parameter_type + letter_char;
                } else {
                    /*
                        Bu syntax catcher şunu yapıyor: 
                        fn ae=
                             ^
                    */
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[334] SyntaxError: Unexpected Token ".to_string() + "'" + letter_char + "'"
                    }
                }
            } else if data.parameter_text_collect { // Name collect
                if (data.parameter_value == "" || next_char == " " || next_char == ":" || nameables.contains(next_char)) && letter_char == " " {
                
                } else if data.parameter_value != "" && letter_char == " " && (next_char != " " && next_char != ":") {
                    /*
                        Bu syntax catcher şunu yapıyor: 
                        fn ae (e e)
                                 ^
                        Ama bunda hata çıkarmamalı

                        fn de (e :
                                 ^
                    */
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[381] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'"
                    }
                } else if letter_char == ":" {
                    data.parameter_text_collect = false;
                    data.parameter_type_collect = true;
                } else if nameables.contains(letter_char) {
                    data.parameter_value = data.parameter_value + letter_char;
                } else if letter_char == ")" && data.parameter_value == "" {
                    data.parameter_text_collect = false;
                    data.parameter_type_collect = false;
                    data.collect_parameters = false;
                    data.collect_return_type = true;
                    data.parameter_value = "".to_string();
                    data.parameter_type = "".to_string();
                } else {
                    /*
                        Bu syntax catcher şunu yapıyor: 
                        fn ae=
                             ^
                    */
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[400] SyntaxError: Unexpected Token ".to_string() + "'" + letter_char + "'"
                    }
                }
            }
        } else if data.collect_name {
            if letter_char == " " && (data.function_name == "" || (next_char == " " || next_char == "(")) {
                let q = "ok";
            } else if data.function_name != "" && letter_char == " " && (next_char != " " && next_char != "(") { // Should work perfect
                /*
                    Bu syntax catcher şunu yapıyor: 
                    fn ae e
                          ^
                    Ama bunda hata çıkarmamalı

                    fn de (
                          ^
                */
                has_syntax_error = true;
                syntax_error = SyntaxError {
                    pos: PositionOfElement {
                        colmn: colmn + 1,
                        line
                    },
                    message: "[430] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'"
                }
            } else if letter_char == "(" { //parameters started
                data.collect_name = false;
                data.collect_parameters = true;
                data.parameter_text_collect = true;
            } else if nameables.contains(letter_char) {
                data.function_name = data.function_name + letter_char;
            } else {
                /*
                    Bu syntax catcher şunu yapıyor: 
                    fn ae=
                         ^
                */
                has_syntax_error = true;
                syntax_error = SyntaxError {
                    pos: PositionOfElement {
                        colmn: colmn + 1,
                        line
                    },
                    message: "[450] SyntaxError: Unexpected Token ".to_string() + "'" + letter_char + "'"
                }
            }
        }

        // } karakterinden sonra ; geliyor onu ignorlamamız lazım

        if data.collect || data.ignore_inner_brace {
            if data.ignore_inner_brace {
                data.ignore_inner_brace = false;
            }
            return CollectorResponse {
                complete: false,
                has_syntax_error,
                syntax_error,
                collected: Collected::Null(false),
                modified_data: ModifiedDataTypes::Function(data)
            };
        } else {
            if letter_char == ";" && !data.collecting_inner_code {
                return CollectorResponse {
                    complete: true,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Function(function::Function {
                        name: data.function_name,
                        parameters: data.parameters,
                        type_return: data.return_type,
                        inner_code: data.inner_code,
                        pos: data.position
                    }),
                    modified_data:  ModifiedDataTypes::Null(false)
                };
            } else if letter_char == "}" && !data.ignore_inner_brace {
                // ; burda yakalayammadık bir sonraki sefere belki?
                data.ignore_inner_brace = true;
                return CollectorResponse {
                    complete: false,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Null(false),
                    modified_data: ModifiedDataTypes::Function(data)
                };
            } else if letter_char == " " && !data.ignore_inner_brace {
                // ; burda yakalayammadık bir sonraki sefere belki?
                data.ignore_inner_brace = true;
                return CollectorResponse {
                    complete: false,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Null(false),
                    modified_data: ModifiedDataTypes::Function(data)
                };
            } else {
                /*
                    Bu syntax catcher şunu yapıyor: 
                    fn ae(e: string) > string {}
                                                ^
                                                Burada ; olmalıydı
                */
                has_syntax_error = true;
                syntax_error = SyntaxError {
                    pos: PositionOfElement {
                        colmn: colmn + 1,
                        line
                    },
                    message: "[505] SyntaxError: Unexpected Token ".to_string() + "'" + letter_char + "'"
                };
                return CollectorResponse {
                    complete: false,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Null(false),
                    modified_data: ModifiedDataTypes::Null(false)
                };
            }
        }
    } else {
        return CollectorResponse {
            complete: false,
            has_syntax_error,
            syntax_error,
            collected: Collected::Null(false),
            modified_data: ModifiedDataTypes::Null(false)
        };
    }
}

pub fn condition_collector(modified_data: ModifiedDataTypes,letter_char: &String, _last_char: &String, _next_char: &String, colmn: usize, line: usize) -> CollectorResponse {
    
    let keyword_letters = "ifelse ";

    let mut has_syntax_error = false;
    let mut syntax_error : SyntaxError = SyntaxError {
        pos: PositionOfElement {
            line: 0,
            colmn: 0
        },
        message: "".to_string()
    };

    if let ModifiedDataTypes::Condition(qdata) = modified_data {
        let mut data = qdata;
        
        // collect_inner_code > collect condition > collect keyword

        if data.collecting_inner_code {
            if data.keyword == "else" && data.else_await_brace {
                if letter_char == "{" {
                    data.else_await_brace = false;
                } else if letter_char != " " {
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[579] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'"
                    }
                }
            } else {
                if letter_char == "}" && !data.ignore_inner_brace {
                    let cdata = data.clone();
                    data.collect = false;
                    data.collecting_inner_code = false;
                    data.ignore_inner_brace = true;
                    data.conditional_chain.push(
                        condition::ConditionChain {
                            type_of_condition: data.keyword,
                            param: data.condition,
                            inner_code: data.inner_code,
                            pos: cdata.position
                        }
                    );
                    data.expect_semicolon = true;
                    data.collecting_inner_code = false;
                    data.collect_keyword = true;
                    data.keyword = "".to_string();
                    data.condition = "".to_string();
                    data.inner_code = "".to_string();
                } else {
                    data.inner_code = data.inner_code + letter_char;
                    if letter_char == "{" {
                        data.ignore_inner_brace = true;
                    } else if letter_char == "}" {
                        data.ignore_inner_brace = false;
                    }
                }
            }
        } else if data.collect_condition {
            if letter_char == "{" {
                data.collecting_inner_code = true;
                data.collect_condition = false;
                data.else_await_brace = false;
            } else {
                data.condition = data.condition + letter_char;
            }
        } else if data.collect_keyword {
            if letter_char == ";" && data.expect_semicolon {
                data.collect = true
            } else if letter_char == " " && (data.keyword == "else" || data.keyword == "elsif") {
                if data.keyword == "else" {
                    data.collecting_inner_code = true;
                    data.else_await_brace = true;
                } else {
                    data.collect_condition = true;
                }
                data.position = condition::PositionOfElement {
                    colmn,
                    line
                };
                data.ignore_inner_brace = false;
                data.expect_semicolon = false;
                data.collect_keyword = false;
            } else if (data.keyword == "" || data.keyword == "else " || data.keyword == "elsif ") && letter_char == " " {
                //EMPTY
            } else {
                if keyword_letters.contains(letter_char) {
                    data.keyword = data.keyword + letter_char;
                } else {
                    has_syntax_error = true;
                    syntax_error = SyntaxError {
                        pos: PositionOfElement {
                            colmn: colmn + 1,
                            line
                        },
                        message: "[582] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'"
                    }
                }
            }
        }

        if data.collect {
            return CollectorResponse {
                complete: true,
                has_syntax_error,
                syntax_error,
                collected: Collected::Condition(
                    condition::Condition {
                        chain: data.conditional_chain
                    }
                ),
                modified_data: ModifiedDataTypes::Null(false)
            };
        } else {
            let cdata = data.clone();
            return CollectorResponse {
                complete: false,
                has_syntax_error,
                syntax_error,
                collected: Collected::Null(false),
                modified_data: ModifiedDataTypes::Condition(cdata)
            };
        }
    } else {
        return CollectorResponse {
            complete: false,
            has_syntax_error,
            syntax_error,
            collected: Collected::Null(false),
            modified_data: ModifiedDataTypes::Null(false)
        };
    }
}

pub fn caller_collector(modified_data: ModifiedDataTypes,letter_char: &String, _last_char: &String, _next_char: &String, colmn: usize, line: usize) -> CollectorResponse {
    
    let mut has_syntax_error = false;
    let mut syntax_error : SyntaxError = SyntaxError {
        pos: PositionOfElement {
            line: 0,
            colmn: 0
        },
        message: "".to_string()
    };

    if let ModifiedDataTypes::Caller(qdata) = modified_data {
        let mut data = qdata;

        if data.quote_started && letter_char == &('"'.to_string()) {
            data.quote_started = false;
        } else if letter_char == &('"'.to_string()) {
            data.quote_started = true;
        }

        if !data.quote_started && letter_char == ";" {
            if data.is_variable {
                data.variable_value = data.collected.clone();
                data.collect = true;
            } else if data.is_return {
                data.returned = data.collected.clone();
                data.collect = true;
            } else if data.is_function {
                data.collect = true;
            }
        } else if !data.quote_started && letter_char == ")" && data.is_function {
            if data.collected.clone() == " " || data.collected.clone() == "" {
                //println!("Syntax Error");
            } else {
                data.function_parameters.push(data.collected.clone());
            }
        } else if !data.quote_started && letter_char == "," && data.is_function {
            data.function_parameters.push(data.collected.clone());
        } else if !data.quote_started && letter_char == "(" {
            data.is_function = true;
            data.function_name = data.collected.clone();
            data.collected = "".to_string();
        } else if !data.quote_started && data.collected.clone() == "return" && letter_char == " " {
            data.collected = "".to_string();
            data.is_return = true;        
        } else if !data.quote_started && letter_char == "=" {
            data.is_variable = true;
            data.variable_name = data.collected;
            data.collected = "".to_string();
        } else {
            data.collected = data.collected + letter_char;
        }

        if data.collect {
            if data.is_function {
                let mut cleaned_function_parameters = Vec::new();
                for function_parameter in data.function_parameters {
                    cleaned_function_parameters.push(
                        function_caller::Parameter {
                            name: "preset".to_string(),
                            value_type: "string".to_string(),
                            value: function_parameter
                        }
                    )
                };
                return CollectorResponse {
                    complete: true,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Caller(Callers::FunctionCaller(
                        function_caller::FunctionCaller {
                            name: data.function_name,
                            paramaters: cleaned_function_parameters,
                            pos: function_caller::PositionOfElement {
                                colmn,
                                line
                            }
                        }
                    )),
                    modified_data: ModifiedDataTypes::Null(false)
                };
            } else if data.is_variable {
                let cleaned_variable_target = value_collector::valueCollector(&data.variable_name, line.clone(), colmn.clone());
                let cleaned_variable_value = value_collector::valueCollector(&data.variable_value, line.clone(), colmn.clone());
                if cleaned_variable_value.error.len() != 0 || cleaned_variable_target.error.len() != 0 {
                    has_syntax_error = true;
                    syntax_error = if cleaned_variable_value.error.len() != 0 {cleaned_variable_value.clone().error.first().unwrap().to_owned()} else {cleaned_variable_target.error.clone().first().unwrap().to_owned()};
                }
                return CollectorResponse {
                    complete: true,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Caller(Callers::VariableDefinier(
                        variable_definier::VariableDefinier {
                            target: cleaned_variable_target.data,
                            value: cleaned_variable_value.data,
                            pos: variable_definier::PositionOfElement {
                                colmn,
                                line
                            }
                        }
                    )),
                    modified_data: ModifiedDataTypes::Null(false)
                };
            } else if data.is_return {
                let cleaned_return_value = value_collector::valueCollector(&data.returned, line.clone(), colmn.clone());
                if cleaned_return_value.error.len() != 0 {
                    has_syntax_error = true;
                    syntax_error = cleaned_return_value.error.first().unwrap().to_owned()
                }
                return CollectorResponse {
                    complete: true,
                    has_syntax_error,
                    syntax_error,
                    collected: Collected::Caller(Callers::ReturnCaller(
                        return_collector::ReturnCaller {
                            value: cleaned_return_value.data,
                            pos: return_collector::PositionOfElement {
                                colmn,
                                line
                            }
                        }
                    )),
                    modified_data: ModifiedDataTypes::Null(false)
                };
            }
            return CollectorResponse {
                complete: true,
                has_syntax_error,
                syntax_error,
                collected: Collected::Null(false),
                modified_data: ModifiedDataTypes::Null(false)
            };
        } else {
            return CollectorResponse {
                complete: false,
                has_syntax_error,
                syntax_error,
                collected: Collected::Null(false),
                modified_data: ModifiedDataTypes::Caller(
                    CallerModifiedData {
                        collect: data.collect,
                        is_function: data.is_function,
                        is_variable: data.is_variable,
                        is_return: data.is_return,
                        collected: data.collected.clone(),
                        return_keyword: data.return_keyword,
                        returned: data.returned,
                        variable_name: data.variable_name,
                        variable_value: data.variable_value,
                        function_name: data.function_name,
                        function_parameters: data.function_parameters,
                        quote_started: data.quote_started,
                        position: data.position,
                    }
                )
            };
        }
    } else {
        return CollectorResponse {
            complete: false,
            has_syntax_error,
            syntax_error,
            collected: Collected::Null(false),
            modified_data: ModifiedDataTypes::Null(false)
        };
    }
}
