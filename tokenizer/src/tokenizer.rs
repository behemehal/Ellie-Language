use crate::processors::items::{self, Processor};
use ellie_core::{com, defs, error};
use serde::{Deserialize, Serialize};

pub struct TokenizerOptions {
    pub functions: bool,
    pub variables: bool,
    pub classes: bool,
    pub imports: bool,
}

pub struct Page {
    pub hash: u64,
    pub cursor: defs::CursorPosition,
    pub errors: Vec<error::Error>,
    pub items: Vec<crate::processors::items::Processors>,
    pub current: crate::processors::items::Processors,
    pub keyword: String,
}

#[derive(Default)]

pub struct ResolvedImport {
    pub hash: u64,
    pub code: String,
}

pub struct Tokenizer<F, E> {
    pub emitter: F,
    pub import_resolver: E,
    pub code: String,
    pub iterator: crate::iterator::Iterator,
}

impl<F, E> Tokenizer<F, E>
where
    F: FnMut(com::Message) + Clone + Sized,
    E: FnMut(String) -> ResolvedImport + Clone + Sized,
{
    pub fn new(code: String, emitter: F, import_resolver: E) -> Self {
        Tokenizer {
            emitter,
            import_resolver,
            code: code,
            iterator: crate::iterator::Iterator::default(),
        }
    }

    pub fn tokenize(&mut self) -> Result<Vec<items::Processors>, Vec<error::Error>> {
        let mut last_char = '\0';
        for letter_char in self.code.chars() {
            self.iterator.iterate(last_char, letter_char);
            last_char = letter_char;
        }
        self.iterator.finalize();

        if !self.iterator.errors.is_empty() {
            Err(self.iterator.errors.clone())
        } else {
            Ok(self.iterator.collected.clone())
        }
    }
}
