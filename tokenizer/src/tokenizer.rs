use ellie_core::com;

pub struct TokenizerOptions {
    pub functions: bool,
    pub variables: bool,
    pub classes: bool,
    pub imports: bool,
}

/*
pub struct Page {
    pub hash: u64,
    pub cursor: defs::CursorPosition,
    pub errors: Vec<error::Error>,
    pub items: Vec<definite::items::Collecting>,
    pub current: crate::processors::items::Processors,
    pub keyword: String,
}
*/

pub struct ResolvedImport {
    pub hash: u64,
    pub code: String,
}

pub struct Tokenizer<F, E> {
    pub emitter: F,
    pub import_resolver: E,
    //pub imports: Vec<Page>,
    pub main: u64,
}

impl<F, E> Tokenizer<F, E>
where
    F: FnMut(com::Message) + Clone + Sized,
    E: FnMut(String) -> ResolvedImport + Clone + Sized,
{
    pub fn new(_options: TokenizerOptions, _main: &str, emitter: F, import_resolver: E) -> Self {
        Tokenizer {
            emitter,
            import_resolver,
            //imports: vec![Page {
            //    hash: 0,
            //    cursor: defs::CursorPosition::default(),
            //    errors: Vec::new(),
            //    items: Vec::new(),
            //    current: crate::processors::items::Processors::Null,
            //    keyword: String::new(),
            //}],
            main: 0,
        }
    }
}
