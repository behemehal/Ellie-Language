use std::{
    collections::hash_map::DefaultHasher,
    fs::File,
    hash::{Hash, Hasher},
    io::Read,
};

use ellie_tokenizer::processors::{items::*, Processor};

fn main() {
    println!("OK");

    let code = "int>";

    let mut processor: definer_processor::DefinerProcessor = Processor::new();
    let mut last_char = '\0';
    for letter_char in code.chars() {
        println!("CHAR: {}", letter_char);
        processor.iterate(last_char, letter_char);
        last_char = letter_char;
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
