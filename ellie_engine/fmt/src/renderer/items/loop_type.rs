use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::loop_type::Loop;

impl CodeRenderer for Loop {
    //Renderer Options
    fn render(&self, _state: &State, _options: &FormatterOptions) -> String {
        todo!()
    }
}
