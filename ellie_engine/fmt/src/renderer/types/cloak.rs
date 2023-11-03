use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::cloak_type::CloakType;

impl CodeRenderer for CloakType {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;

        let mut cloak = format!("{state_scope_length}(");

        for (index, item) in self.collective.iter().enumerate() {
            let value = item.value.render(
                &State {
                    scope_length: 0,
                    ending_token: String::new(),
                },
                options,
            );

            if index != self.collective.len() - 1 {
                cloak.push_str(&format!("{}, ", value));
            } else {
                cloak.push_str(&value.to_string());
            }
        }
        cloak.push_str(&format!("){state_ending_token}"));
        cloak
    }
}
