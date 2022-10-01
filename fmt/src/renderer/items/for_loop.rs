use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::for_loop::ForLoop;

impl CodeRenderer for ForLoop {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        todo!()
    }
}
