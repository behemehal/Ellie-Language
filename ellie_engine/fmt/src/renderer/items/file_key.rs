use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::file_key::FileKey;

impl CodeRenderer for FileKey {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();
        let identifier = if self.is_global { "@!" } else { "@" };
        let variable_name = &self.key_name;

        let value_def = {
            let value_value = self.value.render(&State::empty_state(), options);
            format!(" = {}", value_value)
        };

        // [@][!][name][ = value]
        format!("{state_scope_length}{identifier}{variable_name}{value_def};{line_ending}")
    }
}
