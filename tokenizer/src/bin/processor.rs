use ellie_core::{defs, error};
use ellie_tokenizer::{
    processors::{
        types::{Processors, TypeProcessor},
        Processor,
    },
    syntax::types::*,
};
use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
};

fn main() {
    let code = "()";
    let mut errors: Vec<error::Error> = Vec::new();
    let mut pos = defs::CursorPosition::default();
    let mut processor: TypeProcessor = Processor::new();
    let mut last_char = '\0';
    for letter_char in code.chars() {
        processor.iterate(&mut errors, pos, last_char, letter_char);
        pos.skip_char(1);
        last_char = letter_char;
    }

    if !errors.is_empty() {
        let mut errors_hash = DefaultHasher::new();
        format!("{:?}", errors.clone()).hash(&mut errors_hash);
        panic!(
            "Errors occured: {:#?}\nHash: {}",
            errors,
            errors_hash.finish()
        );
    } else {
        let correct = format!("{:?}", processor.clone());
        let mut correct_hasher = DefaultHasher::new();
        correct.hash(&mut correct_hasher);

        println!(
            "----\nTokenize success:\n{:#?}\nHash: {:#?}\nTexted: {}",
            processor.current.clone().to_definite(),
            correct_hasher.finish(),
            resolve_to_text(processor.current)
        );
    }
}

pub fn read_file(file_dir: &str) -> Result<String, String> {
    let file_read = File::open(file_dir);
    match file_read {
        Err(r) => Err(r.to_string()),
        Ok(mut file) => {
            let mut file_content = Vec::new();
            file.read_to_end(&mut file_content).expect("Unable to read");
            match String::from_utf8(file_content) {
                Ok(code_string) => Ok(code_string),
                Err(e) => Err(e.to_string()),
            }
        }
    }
}

pub fn resolve_to_text(content: Processors) -> String {
    match content {
        Processors::Integer(e) => format!("{:?}", e.data.value),
        Processors::Float(_) => "float".to_string(),
        Processors::Char(_) => "char".to_string(),
        Processors::String(_) => "string".to_string(),
        Processors::Variable(e) => e.data.value,
        Processors::Negative(e) => "!".to_string() + &resolve_to_text(*e.value),
        Processors::Array(_) => "array".to_string(),
        Processors::Reference(e) => {
            let mut built = String::new();
            built += &resolve_to_text(*e.data.reference);
            for i in e.data.chain {
                built += ".";
                built += &i.value;
            }
            built
        }
        Processors::Operator(e) => {
            let mut built = "(".to_string();

            built += &resolve_to_text(*e.data.first);

            built += match e.data.operator {
                operator_type::Operators::ComparisonType(e) => match e {
                    operator_type::ComparisonOperators::Equal => " == ",
                    operator_type::ComparisonOperators::NotEqual => " != ",
                    operator_type::ComparisonOperators::GreaterThan => " > ",
                    operator_type::ComparisonOperators::LessThan => " < ",
                    operator_type::ComparisonOperators::GreaterThanOrEqual => " >= ",
                    operator_type::ComparisonOperators::LessThanOrEqual => " <= ",
                    operator_type::ComparisonOperators::Null => " ?87 ",
                },
                operator_type::Operators::LogicalType(e) => match e {
                    operator_type::LogicalOperators::And => " && ",
                    operator_type::LogicalOperators::Or => " || ",
                    operator_type::LogicalOperators::Null => " ?94 ",
                },
                operator_type::Operators::ArithmeticType(e) => match e {
                    operator_type::ArithmeticOperators::Addition => " + ",
                    operator_type::ArithmeticOperators::Subtraction => " - ",
                    operator_type::ArithmeticOperators::Multiplication => " * ",
                    operator_type::ArithmeticOperators::Exponentiation => " ** ",
                    operator_type::ArithmeticOperators::Division => " / ",
                    operator_type::ArithmeticOperators::Modulus => " % ",
                    operator_type::ArithmeticOperators::Null => " ?105 ",
                },
                operator_type::Operators::AssignmentType(e) => match e {
                    operator_type::AssignmentOperators::Assignment => " = ",
                    operator_type::AssignmentOperators::AdditionAssignment => " += ",
                    operator_type::AssignmentOperators::SubtractionAssignment => " -= ",
                    operator_type::AssignmentOperators::MultiplicationAssignment => " *= ",
                    operator_type::AssignmentOperators::DivisionAssignment => " /= ",
                    operator_type::AssignmentOperators::ModulusAssignment => " &= ",
                    operator_type::AssignmentOperators::ExponentiationAssignment => " **= ",
                    operator_type::AssignmentOperators::Null => " ?117 ",
                },
                operator_type::Operators::Null => " ?121 ",
            };

            built += &resolve_to_text(*e.data.second);
            built += ")";
            built
        }
        Processors::BraceReference(e) => {
            let mut built = "(".to_string();
            built += &resolve_to_text(*e.data.reference);
            built += "[";
            built += &resolve_to_text(*e.data.value);
            built += "])";
            built
        }
        Processors::FunctionCall(e) => {
            let mut built = "(".to_string();
            built += &resolve_to_text(*e.data.target);
            built += ")(";
            for i in e.data.parameters.into_iter().enumerate() {
                if i.0 != 0 {
                    built += ","
                }
                built += &resolve_to_text(i.1.value);
            }
            built += ")";
            built
        }
    }
}

/*
    println!("OK");

    let code = "\"\\ellie\"";
    let mut errors: Vec<error::Error> = Vec::new();
    let mut pos = defs::CursorPosition::default();
    let mut processor: string_type::StringTypeCollector = Processor::new();
    let mut last_char = '\0';
    for letter_char in code.chars() {
        processor.iterate(&mut errors, pos, last_char, letter_char);
        pos.skip_char(1);
        last_char = letter_char;
    }

    if !errors.is_empty() {
        let mut errors_hash = DefaultHasher::new();
        format!("{:?}", errors.clone()).hash(&mut errors_hash);
        panic!(
            "Errors occured: {:#?}\nHash: {}",
            errors,
            errors_hash.finish()
        );
    } else {
        let correct = format!("{:?}", processor.clone());
        let mut correct_hasher = DefaultHasher::new();
        correct.hash(&mut correct_hasher);

        println!(
            "----\nTokenize success:\n{:?}\nHash: {:#?}",
            processor,
            correct_hasher.finish()
        );
    }
*/
