use crate::utils;
use crate::collectors;
use crate::alloc::string::{ToString, String};
use crate::alloc::vec::{Vec};
use crate::alloc::vec;
pub mod types;

//pub fn run_condition(compiled_items: collectors::CompiledItems, variables: Vec<types::Variable>, functions: Vec<types::Function>) -> types::Return {
//    let variables : Vec<types::Variable> = Vec::new();
//    let functions : Vec<types::Function> = Vec::new();
//    let return_value : types::Variable;
//}

fn run_items(mut runtime: Runtime, compiled_items: Vec<collectors::CompiledItems>, parent: String) -> Result<types::FunctionReturn, collectors::SyntaxError> {
    //let mut return_value = Ok(types::FunctionReturn::None);

    for item in compiled_items {
        if let collectors::CompiledItems::Variable(data) = item {
            runtime.addVariable(types::Variable {
                data,
                muteable: true,
                parent: parent.clone()
            });
        } else if let collectors::CompiledItems::Condition(condition) = item {
            println!("-------------------------------");
            println!("- CONDITONS NOT SUPPORTED YET -");
            println!("-------------------------------");
        } else if let collectors::CompiledItems::Function(function) = item {
            println!("------------------------------------");
            println!("- FUNCTIONS CALL NOT SUPPORTED YET -");
            println!("------------------------------------");
        } else if let collectors::CompiledItems::Callers(caller) = item {

            if let collectors::Callers::VariableDefinier(variable_data) = caller {
                
                if let collectors::value_collector::ValueTypes::Variable(found_variable_name) = variable_data.target {
                    let found_variable = runtime.getVariable(found_variable_name);
                    if let Ok(found) = found_variable {





                        //let redefined_variable = types::Variable {
                        //    data: collectors::variable::Variable {
                        //        type_of: ,
                        //        name: ,
                        //        value: found.data.value,
                        //        pos: found.data.pos
                        //    },
                        //    muteable: true,
                        //    parent: found.parent.clone()
                        //}
                    } else {
                        Err::<types::FunctionReturn,collectors::SyntaxError>(
                            collectors::SyntaxError {
                                pos: collectors::PositionOfElement {
                                    colmn: variable_data.pos.colmn,
                                    line: variable_data.pos.line
                                },
                                message: "RefferenceError: Targeted variable not found in scope".to_string()
                            }
                        );
                    }
                } else if let collectors::value_collector::ValueTypes::DotQuery(found_object_name) = variable_data.target {
                    println!("-------------------------------------");
                    println!("- OBJECT REDEFİNE NOT SUPPORTED YET -");
                    println!("-------------------------------------");
                }
            } else if let collectors::Callers::FunctionCaller(function_data) = caller {
            
            } else if let collectors::Callers::ReturnCaller(return_data) = caller {
                println!("---------------------------------");
                println!("- RETURN CALL NOT SUPPORTED YET -");
                println!("---------------------------------");
            }
        
        }
    
    }

    println!("Process Exit With Code 0");
    println!("{:#?}", runtime);
    Ok(types::FunctionReturn::None)
    //return return_value;
}

pub fn run(compiled_items: Vec<collectors::CompiledItems>, runtime_options: types::RuntimeOptions) {
    let runtime = Runtime::new(runtime_options);
    run_items(runtime, compiled_items, "@global".to_string());
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Runtime {
    pub variables : Vec<types::Variable>,
    pub functions : Vec<types::Function>,
    pub runtime_options: types::RuntimeOptions
}

impl Runtime {

    fn new(runtime_options: types::RuntimeOptions) -> Runtime {
        Runtime {
            variables: Vec::new(),
            functions: Vec::new(),
            runtime_options
        }
    }

    fn addFunction(&mut self, data: types::Function) {
        self.functions.push(data);
    }

    fn addVariable(&mut self, data: types::Variable) {
        self.variables.push(data)
    }

    fn getVariable(&mut self, name: String) -> Result<types::Variable, bool> {
        if let Some(variable) = self.variables.iter().find(|x| x.data.name == name) {
            Ok(variable.clone())
        } else {
            Err(true)
        }
    }

    fn setVariable(&mut self, name: String, value: types::Variable) -> Result<bool, bool> {
        if let Some(index) = self.variables.iter().position(|x| x.data.name == name) {
            self.variables[index] = value;
            Ok(true)
        } else {
            Err(true)
        }
    }
}