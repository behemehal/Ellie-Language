use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::variable_type::VariableType;

impl CodeRenderer for VariableType {
    fn render(&self, state: &State, _: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;
        let value = &self.value;
        format!("{state_scope_length}{value}{state_ending_token}")
    }
}
