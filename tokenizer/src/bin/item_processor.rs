use ellie_core::{defs, error};
use ellie_tokenizer::processors::items::{ItemProcessor, Processor};
use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

fn main() {
    let code = "fn test(param1: int, *param2: int) : int {}";
    let mut errors: Vec<error::Error> = Vec::new();
    let mut pos = defs::CursorPosition::default();
    let mut iterator: ellie_tokenizer::iterator::Iterator =
        ellie_tokenizer::iterator::Iterator::default();

    let mut last_char = '\0';
    for letter_char in code.chars() {
        iterator.iterate(last_char, letter_char);
        last_char = letter_char;
    }
    iterator.finalize();

    if !iterator.errors.is_empty() {
        println!("{:#?}", iterator.errors.clone())
    } else {
        println!("{:#?}", iterator)
    }
}
