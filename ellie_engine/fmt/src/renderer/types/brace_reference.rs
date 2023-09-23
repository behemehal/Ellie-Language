use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::brace_reference_type::BraceReferenceType;

impl CodeRenderer for BraceReferenceType {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let reference = self.reference.render(state, options);
        let value = self.value.render(state, options);
        format!("{state_scope_length}{reference}[{value}]{state_ending_token}")
    }
}
