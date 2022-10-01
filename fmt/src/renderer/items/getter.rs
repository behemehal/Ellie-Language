use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::getter::Getter;

impl CodeRenderer for Getter {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        todo!()
    }
}
