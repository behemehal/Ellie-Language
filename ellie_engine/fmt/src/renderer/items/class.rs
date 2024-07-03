use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::class::Class;

impl CodeRenderer for Class {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let is_public = if self.public { "pub " } else { "" };
        let mut class_input = format!("{state_scope_length}{is_public}class {}", self.name);

        if !self.generic_definings.is_empty() {
            let mut generic_definings = "<".to_string();
            for (index, generic_defining) in self.generic_definings.iter().enumerate() {
                let mut _generic_defining = generic_defining.name.to_string();

                if index != self.generic_definings.len() - 1 {
                    if options.leave_space_after_comma {
                        _generic_defining += ", "
                    } else {
                        _generic_defining += ","
                    }
                }
                generic_definings += &_generic_defining;
            }
            generic_definings += ">";
            class_input += &generic_definings;
        }

        if self.body.is_empty() {
            class_input += &format!("{{}}{}", options.render_line_ending());
            class_input
        } else {
            if options.render_brace_next_line {
                class_input += &format!(
                    "{line_ending}{state_scope_length}{{{line_ending}",
                    line_ending = options.render_line_ending()
                );
            } else {
                class_input += &format!(
                    " {{{line_ending}",
                    line_ending = options.render_line_ending()
                );
            }

            let mut child_state = state.clone();
            child_state.scope_length += 1;
            child_state.ending_token = options.render_line_ending();
            for line in self.body.iter() {
                class_input += &line.render(&child_state, options);
            }
            class_input += format!(
                "{state_scope_length}}}{line_ending}",
                line_ending = options.render_line_ending()
            )
            .as_str();
            class_input
        }
    }
}
