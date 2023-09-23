use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::null_resolver::NullResolver;

impl CodeRenderer for NullResolver {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let value = self.target.render(state, options);
        format!("{state_scope_length}{value}!{state_ending_token}")
    }
}
