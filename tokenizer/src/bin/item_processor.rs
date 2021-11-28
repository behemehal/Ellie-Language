use ellie_core::{defs, error};
use ellie_tokenizer::processors::items::{ItemProcessor, Processor};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    let code = "@test =123;";

    let mut errors: Vec<error::Error> = Vec::new();
    let mut pos = defs::CursorPosition::default();
    let mut processor: ItemProcessor = ItemProcessor::default();

    let mut last_char = '\0';
    for letter_char in code.chars() {
        processor.iterate(&mut errors, pos, last_char, letter_char);
        pos = pos.skip_char(1);
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

        if processor.is_complete() {
            println!(
                "----\nTokenize success:\n{:#?}\nHash: {:#?}\n",
                processor.current.clone().to_definite(),
                correct_hasher.finish(),
            );
        } else {
            panic!(
                "----\nTokenize failed (Not complete):\n{:#?}\nHash: {:#?}",
                processor.current.clone(),
                correct_hasher.finish(),
            );
        }
    }
}
