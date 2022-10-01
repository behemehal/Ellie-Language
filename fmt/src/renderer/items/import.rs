use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::import::Import;

impl CodeRenderer for Import {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        todo!()
    }
}
