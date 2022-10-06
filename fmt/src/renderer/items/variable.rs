use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::variable::Variable;

impl CodeRenderer for Variable {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();

        let is_public = if self.public { "pub " } else { "" };

        let identifier = if self.constant {
            if options.use_shorts {
                String::from("c ")
            } else {
                String::from("const ")
            }
        } else {
            if options.use_shorts {
                String::from("v ")
            } else {
                String::from("var ")
            }
        };

        let variable_name = &self.name;

        let rtype_def = {
            if self.has_type {
                let type_value = self
                    .rtype
                    .definer_type
                    .render(&State::empty_state(), options);
                if options.space_before_type_colon {
                    format!(" : {}", type_value)
                } else {
                    format!(": {}", type_value)
                }
            } else {
                String::from("")
            }
        };

        let value_def = {
            if self.has_value {
                let value_value = self.value.render(&State::empty_state(), options);
                format!(" = {}", value_value)
            } else {
                String::from("")
            }
        };
        // [pub] [v | c | var | const] [name] [: type] [ = value]
        format!("{state_scope_length}{is_public}{identifier}{variable_name}{rtype_def}{value_def};{line_ending}")
    }
}
