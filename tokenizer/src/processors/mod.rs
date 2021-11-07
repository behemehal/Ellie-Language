use ellie_core::{defs, error};

pub trait Processor {
    fn new() -> Self;
    fn keyword(&self) -> &str;
    fn has_accessibility(&self) -> bool;
    fn iterate(&mut self, cursor: defs::CursorPosition, last_char: char, letter_char: char);
    fn has_error(&self) -> bool;
    fn errors(&self) -> Vec<error::Error>;
    fn is_complete(&self) -> bool;
}

pub fn reliable_char(character: &char) -> bool {
    let allowed = [
        'q', 'w', 'e', 'r', 't', 'y', 'u', 'ı', 'o', 'p', 'ğ', 'ü', 'a', 's', 'd', 'f', 'g', 'h',
        'j', 'k', 'l', 'ş', 'i', 'z', 'x', 'c', 'v', 'b', 'n', 'm', 'ö', 'ç', '1', '2', '3', '4',
        '5', '6', '7', '8', '9', '0', 'Q', 'W', 'E', 'R', 'T', 'Y', 'U', 'I', 'O', 'P', 'Ğ', 'Ü',
        'A', 'S', 'D', 'F', 'G', 'H', 'J', 'K', 'L', 'Ş', 'İ', 'Z', 'X', 'C', 'V', 'B', 'N', 'M',
        'Ö',
    ];
    allowed.contains(character)
}

pub mod items;
pub mod types;
