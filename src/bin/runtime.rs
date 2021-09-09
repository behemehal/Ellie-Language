use fs::File;
use std::{fs, io::Read};

fn main() {
    let mut file_content = Vec::new();
    let file_read = File::open("./test_dir/test.ei");
    if file_read.is_err() {
        println!("File not found ~./test_dir/test.ei");
        std::process::exit(1);
    } else if let Ok(mut file) = file_read {
        file.read_to_end(&mut file_content).expect("Unable to read");
        let code_string = String::from_utf8(file_content);
        if code_string.is_err() {
            println!("Unable to read file ~./test_dir/test.ei")
        } else if let Ok(_code) = code_string {
            
        }
    }
}
