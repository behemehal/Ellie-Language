use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::setter_call::SetterCall;

impl CodeRenderer for SetterCall {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        todo!()
    }
}
