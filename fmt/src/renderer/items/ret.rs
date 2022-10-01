use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::ret::Ret;

impl CodeRenderer for Ret {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;
        let value_of_ret = self.value.current.render(state, options);
        format!("{state_scope_length}ret {value_of_ret};{state_ending_token}")
    }
}
