use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::import::Import;

impl CodeRenderer for Import {
    //Renderer Options
    fn render(&self, _state: &State, _options: &FormatterOptions) -> String {
        todo!()
    }
}
