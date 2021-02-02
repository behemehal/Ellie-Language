use crate::alloc::string::{ToString, String};
use crate::alloc::vec::Vec;
use crate::utils;
use crate::collectors;
use std::ops::Add;


#[derive(Debug, Clone , PartialEq, Eq)]
pub struct StringCollector {
    pub quote_started: String,  // "string" collector or value inside ["array"]
    pub quote_type: String,     // "string" collector or value inside ["array"]
    pub quote_ended: String,    // "string" collector or value inside ["array"]
}


#[derive(Debug, Clone , PartialEq, Eq)]
pub enum OperatorTypes {
    Addition,
    Multiplication,
    Subtraction,
    Division
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Operator {
    pub Type: OperatorTypes,
    pub items: Vec<ValueTypes>
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ChainArrayLayer {
    pub layer: usize,
    pub items: Vec<ValueTypes>
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct Collective {
    pub key: String,
    pub type_of: String,
    pub value: ValueTypes
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct DotQuery {
    pub target: String,
    pub chain: Vec<String>
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub enum ValueTypes {
    Variable(String),
    Number(i32),
    //Array(Vec<ValueTypes>), DEPRECATED
    //ArrayQuery(Vec<ArrayQuery>), DEPRECATED
    //Reference(String), DEPRECATED
    DotQuery(DotQuery),
    Operator(OperatorTypes),
    Collective(Vec<Collective>),
    String(String),
    Bool(bool),
    None
}

#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ChildCollectorResult {
    pub error: Vec<collectors::SyntaxError>,
    pub data: ValueTypes
}


#[derive(Debug, Clone , PartialEq, Eq)]
pub struct ValueCollector {
    //pub value_type: ValueTypes,
    //pub currentType: CollectingType,
    pub collect_undefined: String,

    pub variable_start: bool,
    pub variable_collect: String,

    pub string_started: bool,
    pub string_quote_type: String,
    pub string_collect: String,

    pub number_started: bool,
    pub number_collect: String,

    pub collective_started: bool,
    pub collective_value: String,
    pub collective_child: i32,
    pub collectived: Vec<ValueTypes>,

    pub dot_nation_query_start: bool,
    pub dot_nation_query_items: Vec<String>,

    pub operator_started: bool,
    pub operator_chain_started: bool,
    pub operator_items: Vec<Operator>,
    pub operator_collect: String

    /*
        Opearator + - things

        (1 + 2)
        ^
        
        operator_started: true
        operator_chain_started=true
        
        (1 + 2)
        ^
        operator_collect += 1

        (1 + 2)
           ^
        Operator {
            type: OperatorTypes.Addition,
            items: vec![
                ValueTypes.Number(1)
                ValueTypes.Number(2)
            ]
        }

        (1 + ( 1 + 1))
           ^
        Operator {
            type: OperatorTypes.Addition,
            items: vec![
                ValueTypes.Number(1),
                ValueTypes.Number(1),
                ValueTypes.Operator(
                    Operator {
                        type: OperatorTypes.Addition,
                        items: vec![
                            ValueTypes.Number(1)
                            ValueTypes.Number(1)
                        ]
                    }
                )
            ]
        }

        Matematik: İlk önce parantez içi hesaplanır
        
        
        //
            Syntax Handler Notes

            (1 + ) )
                 ^
            SyntaxError: Unexpected token )

            (1 + ( ) )
                   ^
            SyntaxError: Unexpected token )

            (1  1 + 2)
              ^
            let user use blank spaces
    
            (1  1 + 2)
                ^
            last char was a blank space and a number already in memory but ! if string quote" started ignore this

        //

    */


}

pub fn CollectiveChildCollector(value: &String, layer: usize, colmn: usize, line: usize) -> ChildCollectorResult {
    let mut errors : Vec<collectors::SyntaxError> = Vec::new();
    let mut layer = layer + 1;
    let mut collected: Vec<collectors::value_collector::ValueTypes> = Vec::new();
    let chars = value.chars();

    let mut key = "".to_string();
    let mut type_of = "".to_string();
    let mut value_collect = "".to_string();


    let keyables = "qwertyuopasdfghjklizxcvbnm0123456789";

    for (colmn_, lchar) in chars.enumerate() {
        let letter_char = &lchar.to_string();
        let last_char = &utils::get_letter(value.to_string(), colmn, false).to_owned();
        let next_char = &utils::get_letter(value.to_string(), colmn, true).to_owned();
        let next_next_char = &utils::get_letter(value.to_string(), colmn + 1, true).to_owned();

        if letter_char == "{" {

        } else if letter_char == "}" {

        }

    }
    ChildCollectorResult {
        error: errors,
        data: ValueTypes::None
    }
}

pub fn DotChildCollector(value: &String, colmn: usize, line: usize) -> ChildCollectorResult {
    let mut syntax_errors: Vec<collectors::SyntaxError> = Vec::new();
    let mut target_started = true;
    let mut chain : Vec<String> = Vec::new();
    let mut target_value = "".to_string();
    let mut value_collect = "".to_string();
    let nameables = "qwertyuopasdfghjklizxcvbnm";

    let chars = value.chars();
    for (colmn_, lchar) in chars.enumerate() {
        let letter_char = &lchar.to_string();
        let last_char = &utils::get_letter(value.to_string(), colmn, false).to_owned();
        let next_char = &utils::get_letter(value.to_string(), colmn, true).to_owned();
        let next_next_char = &utils::get_letter(value.to_string(), colmn + 1, true).to_owned();
        if target_started {
            if (letter_char != " " || (letter_char == " " && target_value == "")) && nameables.contains(&letter_char.to_lowercase()) {
                if letter_char != " " {
                    target_value = target_value + letter_char;
                }
            } else if letter_char == "." {
                target_started = false;
            } else {
                syntax_errors.push(
                    collectors::SyntaxError {
                        message: "[ValueCollector::435] SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'",
                        pos: collectors::PositionOfElement {
                            line,
                            colmn: colmn + colmn_
                        }
                    }
                )
            }
        } else {
            if (letter_char != " " || (letter_char == " " && value_collect == "")) && nameables.contains(&letter_char.to_lowercase()) {
                if letter_char != " " {
                    value_collect = value_collect + letter_char;
                }
            } else if letter_char == "." {
                chain.push(value_collect);
                value_collect = "".to_string();
            } else {
                syntax_errors.push(
                    collectors::SyntaxError {
                        message: "[ValueCollector::454] SyntaxError: SyntaxError: Unexpected identifier ".to_string() + "'" + letter_char + "'",
                        pos: collectors::PositionOfElement {
                            line,
                            colmn: colmn + colmn_
                        }
                    }
                )
            }
        }

    }

    if value_collect != "" {
        chain.push(value_collect);
        value_collect = "".to_string();
    }

    ChildCollectorResult {
        error: syntax_errors,
        data: ValueTypes::DotQuery(
            DotQuery {
                target: target_value,
                chain
            }
        )
    }
}

pub fn valueCollector(value: &String, line: usize, colmn_: usize) -> ChildCollectorResult {
    let mut errors = Vec::new();
    let mut data = ValueTypes::None;
    let chars = value.chars();
    let mut collecting = ValueCollector {
        collect_undefined: "".to_string(),
        
        variable_start: false,
        variable_collect: "".to_string(),

        string_started: false,
        string_quote_type: "".to_string(),
        string_collect: "".to_string(),

        number_started: false,
        number_collect: "".to_string(),

        collective_started: false,
        collective_child: 0,
        collective_value: "".to_string(),
        collectived: Vec::new(),

        dot_nation_query_start: false,
        dot_nation_query_items: Vec::new(),

        operator_started: false,
        operator_chain_started: false,
        operator_items: Vec::new(),
        operator_collect: "".to_string()

    };
    
    for (colmn, lchar) in chars.enumerate() {
        let letter_char = &lchar.to_string();
        let last_char = &utils::get_letter(value.to_string(), colmn, false).to_owned();
        let next_char = &utils::get_letter(value.to_string(), colmn, true).to_owned();
        let next_next_char = &utils::get_letter(value.to_string(), colmn + 1, true).to_owned();
    
        if letter_char == "{" && !collecting.string_started {
            if collecting.collective_started {
                collecting.collective_child = collecting.collective_child+1;
            } else {
                collecting.collective_started = true;
                println!("Collective Started");
            }
        } else if letter_char == "}" && !collecting.string_started {
            if collecting.collective_child == 0 {
                println!("Collected Collective");
            } else {
                collecting.collective_child = collecting.collective_child - 1;
            }
        } else if collecting.collective_started {

            if letter_char == "\"" || letter_char == "'" {
                if collecting.string_started {
                    if collecting.string_quote_type == letter_char.to_string() && last_char != "\\" {
                        collecting.string_quote_type = "".to_string();
                        collecting.string_started = false;
                    }
                } else {
                    collecting.string_quote_type = letter_char.to_string();
                    collecting.string_started = true;
                }
            }
            collecting.collective_value = collecting.collective_value + letter_char;
        } else if collecting.number_started {
            if letter_char.parse::<i32>().is_ok() {
                collecting.number_collect = collecting.number_collect + letter_char;
            } else {
                errors.push(
                    collectors::SyntaxError {
                        message: "[ValueCollector::536] SyntaxError: Unexpected Token '".to_string() + &letter_char.to_string() + "'",
                        pos: collectors::PositionOfElement {
                            line: line,
                            colmn: colmn_
                        }
                    }
                );
            }
        } else if collecting.variable_start && letter_char != "\"" && letter_char != "'" {
            let nameables = "qwertyuopasdfghjklizxcvbnm.";
            if nameables.contains(&letter_char.to_lowercase()) || (letter_char == " " && (!nameables.contains(&next_char.to_lowercase()) || next_char == "")) {
                if letter_char != " " {
                    collecting.variable_collect = collecting.variable_collect + letter_char;
                }
            } else {
                errors.push(
                    collectors::SyntaxError {
                        message: "[ValueCollector::543] SyntaxError: Unexpected Token '".to_string() + &letter_char.to_string() + "'",
                        pos: collectors::PositionOfElement {
                            line: line,
                            colmn: colmn_ + colmn
                        }
                    }
                );
            }
        } else if((letter_char == "\"" || letter_char == "'") && last_char != "\\") {
            if collecting.string_started {

            } else {
                if collecting.number_started || collecting.variable_start {
                    errors.push(
                        collectors::SyntaxError {
                            message: "[ValueCollector::558] SyntaxError: Unexpected Token '".to_string() + &letter_char.to_string() + "'",
                            pos: collectors::PositionOfElement {
                                line: line,
                                colmn: colmn_
                            }
                        }
                    );
                } else {
                    collecting.string_started = true;
                    collecting.string_quote_type = letter_char.to_string();
                }
            }
        } else if !collecting.variable_start && !collecting.string_started && !collecting.number_started {
            
            let number = letter_char.parse::<i32>();
            if number.is_ok() {
                collecting.number_started = true;
                collecting.number_collect = collecting.number_collect + letter_char
            } else {
                if letter_char != " " {
                    collecting.variable_start = true;
                    collecting.variable_collect = collecting.variable_collect + letter_char
                } else if letter_char == " " && collecting.variable_start {
                    errors.push(
                        collectors::SyntaxError {
                            message: "[ValueCollector::616] StupidityError: Don't know what is this".to_string(),
                            pos: collectors::PositionOfElement {
                                line: line,
                                colmn: colmn_
                            }
                        }
                    );
                }
            }

        }
    }

    /*
    if collecting.expect_object_brace != 0 && (collecting.object_value != "") {
        if let ValueTypes::None = collecting.object_resolved {
            if (collecting.object_value != "") {
                errors.push(
                    collectors::SyntaxError {
                        message: "[ValueCollector::611] SyntaxError: Unclosed Brace".to_string(),
                        pos: collectors::PositionOfElement {
                            line: line,
                            colmn: value.len()
                        }
                    }
                );
            } else if collecting.expect_object_brace != 0 {
                errors.push(
                    collectors::SyntaxError {
                        message: "[ValueCollector::621] SyntaxError: Unclosed Brace".to_string(),
                        pos: collectors::PositionOfElement {
                            line: line,
                            colmn: value.len()
                        }
                    }
                );
            }
        } else {
            errors.push(
                collectors::SyntaxError {
                    message: "[ValueCollector::632] SyntaxError: Unclosed Brace".to_string(),
                    pos: collectors::PositionOfElement {
                        line: line,
                        colmn: value.len()
                    }
                }
            );
        }
    }
    */

    if collecting.string_started {
        data = collectors::value_collector::ValueTypes::String(collecting.string_collect)
    } else if collecting.variable_start {
        if collecting.variable_collect == "true" || collecting.variable_collect == "false" {
            data = ValueTypes::Bool(collecting.variable_collect.clone().parse::<bool>().unwrap());
        } else {
            if collecting.variable_collect.contains(".") {
                let processed_dot_query = DotChildCollector(&collecting.variable_collect, colmn_, line);
                if processed_dot_query.error.len() == 0 {
                    if let ValueTypes::DotQuery(captured_dot_query) = processed_dot_query.data.clone() {
                        if captured_dot_query.chain.len() == 0 {
                            errors.push(
                                collectors::SyntaxError {
                                    message: "[ValueCollector::702] SyntaxError: Unexpected Token '".to_string() + value.get(captured_dot_query.target.len()..captured_dot_query.target.len() + 1).unwrap() + "'",
                                    pos: collectors::PositionOfElement {
                                        line: line,
                                        colmn: colmn_ + captured_dot_query.target.len() + 1
                                    }
                                }
                            );
                        } else {
                            data = processed_dot_query.data.clone();
                        }
                    }
                } else {
                    for err in processed_dot_query.error {
                        errors.push(err);
                    }
                }
            } else {
                if let Err(pos) = utils::compare_contains("qwertyuopasdfghjklizxcvbnm".to_string(), collecting.variable_collect.clone()) {
                    errors.push(
                        collectors::SyntaxError {
                            message: "[ValueCollector::702] SyntaxError: Unexpected Token '".to_string() + value.get(pos..pos+1).unwrap() + "'",
                            pos: collectors::PositionOfElement {
                                line: line,
                                colmn: colmn_
                            }
                        }
                    );
                } else {
                    data = ValueTypes::Variable(collecting.variable_collect.clone());
                }    
            }
        }
    } else if collecting.number_started {
        let number = collecting.number_collect.clone().parse::<i32>();
        if number.is_ok() {
            data = ValueTypes::Number(number.unwrap());
        } else {
            errors.push(
                collectors::SyntaxError {
                    message: "[ValueCollector::616] TypeError: Wrong type of variable data".to_string(),
                    pos: collectors::PositionOfElement {
                        line: line,
                        colmn: colmn_
                    }
                }
            );
        }
    }

    ChildCollectorResult {
        error: errors,
        data
    }

    //TODO: check expect_object_brace if its not 0 thats means there is a object that not completed
    //println!("{:#?}", collecting);
}

/*
    String Collecting : OK
    Variable Collecting: NOT OK
    Array Collecting: {
        String: OK
    }



    [


        ArrayStarted: true

        [
            ArrayChainStarted.push(true)


        ]
    ]


*/