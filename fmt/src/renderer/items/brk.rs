use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::brk::Brk;

impl CodeRenderer for Brk {
    //Renderer Options
    fn render(&self, state: &State, _: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;
        format!("{state_scope_length}ret;{state_ending_token}")
    }
}
