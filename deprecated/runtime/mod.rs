pub mod runtime_variable;
pub mod runtime_function;
use crate::utils;
use crate::collectors;
use crate::alloc::string::{ToString, String};
use crate::alloc::vec::{Vec};
use crate::alloc::vec;
use std::env;


#[derive(Debug, Clone , PartialEq, Eq)]
pub enum ReturnedType {
    Null(bool),
    FunctionReturn(bool),
    SyntaxError
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct RuntimeReturn {
    pub has_error: bool,
    pub syntax_errors: Vec<collectors::SyntaxError>,
    pub returned: ReturnedType
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct DynamicFunction {
    pub name: String,
    pub parameters: Vec<collectors::function::Parameter>,
    pub return_type: String,
    pub caller: fn(DynamicCaller) -> DynamicResponse
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct DynamicCaller {
    pub pos: collectors::PositionOfElement,
    pub parameters: Vec<runtime_variable::Variable>
}



pub fn run(mapped_data: collectors::Compiled, extra_variables: Vec<runtime_variable::Variable>, dynamic_functions: Vec<DynamicFunction>) -> RuntimeReturn {
    let debug = false;
    //let debug = env::args().any(|x| x == "--debug");
    if debug {
        println!("---DEBUG MODE---");
    }

    let mut has_error = false;
    let mut syntax_errors : Vec<collectors::SyntaxError> = Vec::new();
    let mut variables: Vec<runtime_variable::Variable> = Vec::new();
    let mut functions: Vec<runtime_function::Function> = Vec::new();

    for extra_variable in extra_variables {
        variables.push(extra_variable);
    }
    
    for dynamic_function in dynamic_functions.clone() {
        functions.push(
            runtime_function::Function {
                name: dynamic_function.name,
                parameters: dynamic_function.parameters,
                dynamic: true,
                type_return: dynamic_function.return_type,
                code: runtime_function::Code::Null(true)
            }
        );
    }

    variables.push(
        runtime_variable::Variable {
            name: "export".to_string(),
            value: runtime_variable::ValueTypes::TypeFunction(
                runtime_variable::FunctionVariable {
                    dynamic: false,
                    name: "export".to_string(),
                    type_return: "void".to_string(),
                    code: runtime_function::Code::Compiled(crate::collectors::Compiled {
                        items: Vec::new(),
                        syntax_errors: Vec::new()
                    })
                }
            ),
            type_identifier: "function".to_string(),
            muteable: false
        }
    );


    //let envVariables = vec![
    //    runtime_variable::Variable {
    //        name: "export".to_string(),
    //        value: runtime_variable::ValueTypes::TypeFunction(
    //            runtime_variable::FunctionVariable {
    //                code: crate::collectors::Compiled {
    //                    items: Vec::new(),
    //                    syntax_errors: Vec::new()
    //                }
    //            }
    //        ),
    //        type_identifier: "function".to_string()
    //    }
    //];


    for e in mapped_data.items {
        if let collectors::CompiledItems::Variable(data) = e {
            let index = variables.iter().position(|r| r.name == data.name);
            if index == None {


                let cleaned_value = crate::utils::remove_all_white_spaces(data.value.clone());
                if data.type_of == "string" {
                    if cleaned_value[0..1] == '"'.to_string() || cleaned_value[0..1] == "'".to_string() {
                        variables.push(
                            runtime_variable::Variable {
                                name: data.name,
                                value: runtime_variable::ValueTypes::TypeString(runtime_variable::StringVariable {
                                    length: data.value.len() - 2,
                                    value: cleaned_value
                                }),
                                type_identifier: "string".to_string(),
                                muteable: true
                            }
                        );
                    }

                    
                } else if data.type_of == "array" {
                    if debug {
                        //println!("Found Array Variable");
                    }
                } else if data.type_of == "bool" {
                    //let type = |x| if crate::utils::remove_all_white_spaces(data.value.clone()).parse::<bool>().is_ok() { "bool" } else if crate::utils::remove_all_white_spaces(data.value.clone()).parse::<i32>().is_ok() {"number"} 
                    let parsed_value = crate::utils::remove_all_white_spaces(data.value.clone()).parse::<bool>();
                    if parsed_value.is_err() {
                        has_error = true;
                        syntax_errors.push(collectors::SyntaxError {
                            pos: collectors::PositionOfElement {
                                colmn: data.pos.colmn,
                                line: data.pos.line
                            },
                            message: "RefferenceError: expected 'bool' found 'unknown_type'".to_string()
                        });
                        break;
                    } else {
                        variables.push(
                            runtime_variable::Variable {
                                name: data.name,
                                value: runtime_variable::ValueTypes::TypeBool(runtime_variable::BooleanVariable {
                                    value: parsed_value.unwrap()
                                }),
                                type_identifier: "string".to_string(),
                                muteable: true
                            }
                        );
                    }
                    if debug {
                        //println!("Found Boolean Variable");
                    }
                } else if data.type_of == "number" {
                    if debug {
                        //println!("DEBUG: {}", data.value.clone());
                    }
                    //println!("Found Number Variable");
                    let parsed_value = crate::utils::remove_all_white_spaces(data.value.clone()).parse::<i32>();
                    if parsed_value.is_err() {
                        has_error = true;
                        syntax_errors.push(collectors::SyntaxError {
                            pos: collectors::PositionOfElement {
                                colmn: data.pos.colmn,
                                line: data.pos.line
                            },
                            message: "RefferenceError: expected 'number' found 'unknown_type'".to_string()
                        });
                    } else {
                        variables.push(
                            runtime_variable::Variable {
                                name: data.name,
                                value: runtime_variable::ValueTypes::TypeNumber(runtime_variable::NumberVariable {
                                    positive: parsed_value.clone().unwrap() > 0,
                                    value: parsed_value.clone().unwrap()
                                }),
                                type_identifier: "string".to_string(),
                                muteable: true
                            }
                        );
                    }
                } else {
                    has_error = true;
                    syntax_errors.push(collectors::SyntaxError {
                        pos: collectors::PositionOfElement {
                            colmn: data.pos.colmn,
                            line: data.pos.line
                        },
                        message: "RefferenceError: Unexpected Type '".to_string() + &data.type_of + "'"
                    });
                    break;
                }
            } else {
                has_error = true;
                syntax_errors.push(collectors::SyntaxError {
                    pos: collectors::PositionOfElement {
                        colmn: data.pos.colmn,
                        line: data.pos.line
                    },
                    message: "RefferenceError: Already Definied (Add --adef to enable feature)".to_string()
                });
                break;
            }
        } else if let collectors::CompiledItems::Function(data) = e {
            let mut parameters = data.parameters.clone();
            //for inner_parameter in data.parameters.clone() {
            //    parameters.push(
            //        crate::collectors::function::Parameter {
            //            name: inner_parameter.name,
            //            type_of: inner_parameter.type_of
            //        }
            //    );
            //};
            let cleaned_code =crate::utils::clean_up(data.inner_code.clone());
            let b: Vec<char> = cleaned_code.chars().skip(1).collect();
            let fixed_code: String = if cleaned_code.starts_with("\n") { b.into_iter().collect() } else { cleaned_code };
            let compiled_inner_code = crate::mapper::map(fixed_code);
            if compiled_inner_code.syntax_errors.len() != 0 {
                has_error = true;
                syntax_errors.push(compiled_inner_code.syntax_errors[0].clone());
                break;
            } else {
                let available_return_types = vec!["void".to_string(), "string".to_string(), "array".to_string(), "bool".to_string(), "number".to_string()];
                if available_return_types.iter().position(|t| t == &data.type_return.clone()) == None {
                    has_error = true;
                    syntax_errors.push(collectors::SyntaxError {
                        pos: collectors::PositionOfElement {
                            colmn: data.pos.colmn,
                            line: data.pos.line
                        },
                        message: "RefferenceError: Unexpected Return Type '".to_string() + &data.type_return + "'"
                    });
                    break;
                } else {
                    functions.push(
                        runtime_function::Function {
                            name: data.name,
                            parameters: parameters,
                            dynamic: false,
                            type_return: data.type_return,
                            code: runtime_function::Code::Compiled(compiled_inner_code)
                        }
                    );
                }
            }
        } else if let collectors::CompiledItems::Condition(data) = e {
            if debug {
                //println!("Found Condition");
            }
        } else if let collectors::CompiledItems::Callers(data) = e {
            if let collectors::Callers::FunctionCaller(dataq) = data {
                let function_memory_index = functions.iter().position(|r| r.name == crate::utils::remove_all_white_spaces(dataq.name.clone()));
                if function_memory_index == None {
                    has_error = true;
                    syntax_errors.push(collectors::SyntaxError {
                        pos: collectors::PositionOfElement {
                            colmn: dataq.pos.colmn,
                            line: dataq.pos.line
                        },
                        message: "RefferenceError: ".to_string() + &dataq.name + " is not defined"
                    });
                    break;
                } else {
                    let target_function = functions[function_memory_index.unwrap()].clone();
                    if dataq.paramaters.len() != target_function.parameters.len() {
                        has_error = true;
                        syntax_errors.push(collectors::SyntaxError {
                            pos: collectors::PositionOfElement {
                                colmn: dataq.pos.colmn,
                                line: dataq.pos.line
                            },
                            message: "RefferenceError: this function takes ".to_string() + &target_function.parameters.len().to_string() + " argument but " + &dataq.paramaters.len().to_string() + " arguments were supplied'"
                        });
                    } else {
                        let mut cleaned_parameters: Vec<runtime_function::Parameter> = vec![];

                        for parameter in dataq.paramaters {
                            let cleaned_parameter = crate::utils::remove_all_white_spaces(parameter.value);
                            let found_variable = variables.iter().position(|r| r.name == cleaned_parameter);
                            let found_function = functions.iter().position(|r| r.name == cleaned_parameter);
                            if cleaned_parameter[0..1] == '"'.to_string() || cleaned_parameter[0..1] == "'".to_string() {
                                cleaned_parameters.push(runtime_function::Parameter {
                                    name: parameter.name,
                                    target_type: parameter.value_type,
                                    value: cleaned_parameter,
                                    is_variable: false,
                                    is_string: true,
                                    is_number: false,
                                    is_bool: false,
                                    is_function: false
                                });
                            } else if cleaned_parameter.parse::<i32>().is_ok() {
                                cleaned_parameters.push(runtime_function::Parameter {
                                    name: parameter.name,
                                    target_type: parameter.value_type,
                                    value: cleaned_parameter,
                                    is_variable: false,
                                    is_string: false,
                                    is_number: true,
                                    is_bool: false,
                                    is_function: false
                                });
                            } else if cleaned_parameter == "true" || cleaned_parameter == "false" {
                                cleaned_parameters.push(runtime_function::Parameter {
                                    name: parameter.name,
                                    target_type: parameter.value_type,
                                    value: cleaned_parameter,
                                    is_variable: false,
                                    is_string: false,
                                    is_number: false,
                                    is_bool: true,
                                    is_function: false
                                });
                            } else if found_variable != None {
                                cleaned_parameters.push(runtime_function::Parameter {
                                    name: parameter.name,
                                    target_type: parameter.value_type,
                                    value: cleaned_parameter,
                                    is_variable: true,
                                    is_string: variables[found_variable.unwrap()].type_identifier == "string",
                                    is_number: variables[found_variable.unwrap()].type_identifier == "number",
                                    is_bool: variables[found_variable.unwrap()].type_identifier == "bool",
                                    is_function: false
                                });
                            } else if found_function != None {
                                cleaned_parameters.push(runtime_function::Parameter {
                                    name: parameter.name,
                                    target_type: parameter.value_type,
                                    value: cleaned_parameter,
                                    is_variable: true,
                                    is_string: false,
                                    is_number: false,
                                    is_bool: false,
                                    is_function: true
                                });
                            } else {
                                cleaned_parameters.push(runtime_function::Parameter {
                                    name: parameter.name,
                                    target_type: parameter.value_type,
                                    value: cleaned_parameter,
                                    is_variable: false,
                                    is_string: false,
                                    is_number: false,
                                    is_bool: false,
                                    is_function: false
                                });
                            }
                            
                        };

                        let undefined_paramater = cleaned_parameters.iter().position(|r| (r.is_variable == false && r.is_string == false && r.is_number == false && r.is_bool == false && r.is_function == false));
                        let missmatch_paramater = cleaned_parameters.iter().position(|r| ((r.is_string && r.target_type != "string") && (r.is_number && r.target_type != "number") && (r.is_bool && r.target_type != "bool") && (r.is_function && r.target_type != "fn")));


                        if missmatch_paramater != None {
                            let wrong_element = &cleaned_parameters[missmatch_paramater.clone().unwrap()];
                            let got_instead  = (if wrong_element.is_bool {"bool"} else if wrong_element.is_function {"function"} else if wrong_element.is_number {"number"} else if wrong_element.is_string {"string"} else {"undefined"}).to_string();
                            has_error = true;
                            syntax_errors.push(collectors::SyntaxError {
                                pos: collectors::PositionOfElement {
                                    colmn: dataq.pos.colmn,
                                    line: dataq.pos.line
                                },
                                message: "ReferenceError: expected ".to_string() + &wrong_element.target_type + " found " + &got_instead
                            });
                            break;
                        } else if undefined_paramater != None {
                            has_error = true;
                            syntax_errors.push(collectors::SyntaxError {
                                pos: collectors::PositionOfElement {
                                    colmn: dataq.pos.colmn,
                                    line: dataq.pos.line
                                },
                                message: "RefferenceError: ".to_string() + &cleaned_parameters[undefined_paramater.unwrap()].value + " is not defined"
                            });
                            break;
                        } else {
                            let mut fixed_parameters: Vec<runtime_variable::Variable> = Vec::new();
                            for (index, cleaned_parameter) in cleaned_parameters.iter().enumerate() {
                                    let fparamaters = functions[function_memory_index.unwrap()].clone();
                                    let paramater = &fparamaters.clone().parameters[index];
                                    fixed_parameters.push(
                                        runtime_variable::Variable {
                                            name: paramater.clone().name,
                                            value: if cleaned_parameter.is_variable {
                                                if cleaned_parameter.is_function {
                                                    let found_function_data = functions[functions.iter().position(|r| r.name == cleaned_parameters[0].value).unwrap()].clone();
                                                    runtime_variable::ValueTypes::TypeFunction(
                                                        runtime_variable::FunctionVariable {
                                                            code: found_function_data.code,
                                                            dynamic: found_function_data.dynamic,
                                                            name: found_function_data.name,
                                                            type_return: found_function_data.type_return
                                                        }
                                                    )
                                                } else {
                                                    let found_variable_data = variables[variables.iter().position(|r| r.name == cleaned_parameters[0].value).unwrap()].clone();
                                                    found_variable_data.value
                                                }
                                                
                                            } else if cleaned_parameter.target_type == "number" {
                                                runtime_variable::ValueTypes::TypeNumber(
                                                    runtime_variable::NumberVariable {
                                                        positive: cleaned_parameter.value.parse::<i32>().unwrap() > 0,
                                                        value: cleaned_parameter.value.parse::<i32>().unwrap()
                                                    }
                                                )
                                            } else if cleaned_parameter.target_type == "string" {
                                                runtime_variable::ValueTypes::TypeString(
                                                    runtime_variable::StringVariable {
                                                        length: cleaned_parameter.clone().value.len(),
                                                        value: cleaned_parameter.clone().value
                                                    }
                                                )
                                            } else if cleaned_parameter.target_type == "array" {
                                                //runtime_variable::ValueTypes::TypeArray(
                                                //    runtime_variable::ArrayVariable {
                                                //        length: cleaned_parameter.value.len(),
                                                //        value: cleaned_parameter.value
                                                //    }
                                                //)
                                                runtime_variable::ValueTypes::TypeString(
                                                    runtime_variable::StringVariable {
                                                        length: 15,
                                                        value: "not_implemented".to_string()
                                                    }
                                                )
                                            } else if cleaned_parameter.target_type == "bool" {
                                                runtime_variable::ValueTypes::TypeBool(
                                                    runtime_variable::BooleanVariable {
                                                        value: cleaned_parameter.value.parse::<bool>().unwrap()
                                                    }
                                                )
                                            } else if cleaned_parameter.target_type == "fn" {
                                                runtime_variable::ValueTypes::TypeFunction(
                                                    runtime_variable::FunctionVariable {
                                                        dynamic: false,
                                                        type_return: "unvoid".to_string(),
                                                        name: cleaned_parameter.name.clone(),
                                                        code: runtime_function::Code::Compiled(crate::mapper::map(cleaned_parameter.clone().value))
                                                    }
                                                )
                                            } else {
                                                runtime_variable::ValueTypes::TypeString(
                                                    runtime_variable::StringVariable {
                                                        length: 9,
                                                        value: "undefined".to_string()
                                                    }
                                                )
                                            },
                                            type_identifier: cleaned_parameter.clone().target_type,
                                            muteable: false
                                        }
                                    )
                            }
                            if target_function.dynamic {

                                let target_dynamic_function = dynamic_functions.clone().iter().position(|f| f.name == target_function.name);

                                if target_dynamic_function != None {
                                    (dynamic_functions[target_dynamic_function.unwrap()].caller)(DynamicCaller {
                                        pos: collectors::PositionOfElement {
                                            colmn: dataq.pos.colmn,
                                            line: dataq.pos.line
                                        },
                                        parameters: fixed_parameters
                                    });
                                }
                                
                                

                                /*

                                fn get(&self, key: &str) -> Container
                                {
                                    self.get_func(self, key)
                                }

                                to this

                                fn get(&self, key: &str) -> i32
                                {
                                    (self.get_func)(self, key)
                                }

                                */

                                //let c = cleaned_parameters.iter().map(|x| 
                                //    runtime_variable::Variable {
                                //        muteable: false,
                                //        name: x.name,
                                //        type_identifier: x.target_type,
                                //        value: if x.is_string {
                                //            runtime_variable::ValueTypes::TypeString(runtime_variable::StringVariable {
                                //                length: x.value.len(),
                                //                value: x.value
                                //            })
                                //        } else if x.is_number {
                                //            runtime_variable::ValueTypes::TypeNumber(
                                //                runtime_variable::NumberVariable {
                                //                    positive: 
                                //                }
                                //            )
                                //        }
                                //    }
                                //);

                                //[
                                //            runtime_variable::Variable {
                                //                muteable: false,
                                //                name: 
                                //            }
                                //]




                                //if cleaned_parameters[0].is_variable {
                                //    if cleaned_parameters[0].is_function {
                                //        let found_function_data = functions[functions.iter().position(|r| r.name == cleaned_parameters[0].value).unwrap()].clone();
                                //        println!("[{}: {} > {}]", 
                                //            (if found_function_data.dynamic {"DynamicFunction"} else {"Function"}).to_string(),
                                //            found_function_data.name,
                                //            found_function_data.type_return
                                //        );
                                //    } else {
                                //        let found_variable_data = variables[variables.iter().position(|r| r.name == cleaned_parameters[0].value).unwrap()].clone();
                                //        println!("{}", print_clear(found_variable_data.value));
                                //    }
                                //    
                                //} else {
                                //    //println!("{}", cleaned_parameters[0].value);
                                //}


                            } else {
                                if let runtime_function::Code::Compiled(compiled_function_code) = target_function.code {
                                    println!("- {:#?}", compiled_function_code);
                                    run(compiled_function_code, fixed_parameters, dynamic_functions.clone());
                                }
                            }
                        }
                        // TODO: Implement paramater type check!!!!
                    }
                }
            } else if let collectors::Callers::ReturnCaller(dataq) = data {
                //println!("Return Data");
            } else if let collectors::Callers::VariableDefinier(dataq) = data {


                //if dataq.target
                let cleaned_value = crate::utils::remove_all_white_spaces(dataq.value.clone());
                let cleaned_name = crate::utils::remove_all_white_spaces(dataq.target.clone());
                let variable_memory_index = variables.iter().position(|r| r.name == cleaned_name);

                if variable_memory_index == None {
                    has_error = true;
                    syntax_errors.push(collectors::SyntaxError {
                        pos: collectors::PositionOfElement {
                            colmn: dataq.pos.colmn,
                            line: dataq.pos.line
                        },
                        message: "RefferenceError: ".to_string() + &cleaned_name + " is not defined"
                    });
                } else {
                    if cleaned_value[0..1] == '"'.to_string() || cleaned_value[0..1] == "'".to_string() { // Raw text
                        //let mut v = variables.iter().nth(variable_memory_index.unwrap()).unwrap();
                        variables[variable_memory_index.unwrap()].value = runtime_variable::ValueTypes::TypeString(
                            runtime_variable::StringVariable {
                                length: cleaned_value.len() - 2,
                                value: cleaned_value
                            }
                        )
                    } else {
                         
                    }
                }

                //println!("Fount variable definier?: {:?}", );

                //if dataq.target == "export" {
//
                //}
                
                //if dataq.target == "export" {
                //    let index = functions.iter().position(|r| r.name == dataq.value).unwrap();
                //    data.value == collectors::fun 
                //} 

            }
        }
    }

    return RuntimeReturn {
        has_error,
        syntax_errors,
        returned: ReturnedType::Null(true)
    }
} 