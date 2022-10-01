use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::definers::GenericType;

impl CodeRenderer for GenericType {
    //Renderer Options
    fn render(&self, state: &State, _: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space();
        let state_ending_token = &state.ending_token;
        let type_of_value = &self.rtype;
        //type
        format!("{state_scope_length}{type_of_value}{state_ending_token}")
    }
}
