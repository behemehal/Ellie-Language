use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::reference_type::ReferenceType;

impl CodeRenderer for ReferenceType {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;


        let mut reference = self.reference.render(state, options);

        for chain in self.chain.iter() {
            reference += &format!(".{}", chain.value);
        }

        format!("{state_scope_length}{reference}{state_ending_token}")
    }
}
