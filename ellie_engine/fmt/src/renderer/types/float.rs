use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::types::decimal_type::DecimalType;

impl CodeRenderer for DecimalType {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let state_ending_token = &state.ending_token;
        let value = self.raw.split(".").collect::<Vec<&str>>()[1].to_string();
        let starter = if self.raw.starts_with(".") || options.decimal_starts_with_dot {
            "."
        } else {
            "0."
        };
        let tag = if self.is_double {
            "d"
        } else if options.decorate_float_with_f_tag {
            "f"
        } else {
            ""
        };
        format!("{state_scope_length}{starter}{value}{tag}{state_ending_token}")
    }
}
