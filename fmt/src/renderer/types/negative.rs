use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::negative_type::Negative;

impl CodeRenderer for Negative {
    //Renderer Options
        fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let value = self.value.render(state, options);
        format!("{state_scope_length}!{value}{state_ending_token}")
    }
}
