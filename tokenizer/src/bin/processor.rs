use ellie_core::{defs, error};
use ellie_tokenizer::{
    processors::{types::TypeProcessor, Processor},
    syntax::{items::*, types::*},
};
use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
};

fn main() {
    let code = "1.3";
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
            "----\nTokenize success:\n{:?}\nHash: {:#?}",
            processor,
            correct_hasher.finish()
        );
    }

    /*
    match read_file("./ellie.ei") {
        Ok(file) => {
            let mut hasher = DefaultHasher::new();
            file.hash(&mut hasher);
            let fileHash = hasher.finish();
            println!("FILE HASH: {}", fileHash);
            println!("FILE DATA: {}", file);


        }
        Err(_) => panic!("?"),
    }
    */
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
