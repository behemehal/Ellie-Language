use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::CloakType;

impl CodeRenderer for CloakType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;

        let comment = if options.leave_space_after_comma {
            ", "
        } else {
            ","
        };

        let mut cloak = String::from("(");

        for (idx, item) in self.entries.iter().enumerate() {
            let item = item.render(&State::empty_state(), options);
            if idx == self.entries.len() - 1 {
                cloak.push_str(&format!("{}", item));
            } else {
                cloak.push_str(&format!("{}{}", item, comment));
            }
        }

        //([type], [size]...)
        format!("{state_scope_length}{cloak}){state_ending_token}")
    }
}
