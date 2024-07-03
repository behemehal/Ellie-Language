use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::loop_type::Loop;

impl CodeRenderer for Loop {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);

        let iterator = self
            .condition
            .current
            .render(&State::empty_state(), options);

        let mut loop_input = format!("{state_scope_length}loop {iterator}",);

        if self.body.is_empty() {
            loop_input += &format!(
                " {{}}{line_ending}",
                line_ending = options.render_line_ending()
            );
            return loop_input;
        }

        if options.render_brace_next_line {
            loop_input += &format!(
                "{line_ending}{state_scope_length}{{{line_ending}",
                line_ending = options.render_line_ending()
            );
        } else {
            loop_input += &format!(
                " {{{line_ending}",
                line_ending = options.render_line_ending()
            );
        }

        let mut child_state = state.clone();
        child_state.scope_length += 1;
        child_state.ending_token = options.render_line_ending();
        for line in self.body.iter() {
            loop_input += &line.render(&child_state, options);
        }
        loop_input += format!(
            "{state_scope_length}}}{line_ending}",
            line_ending = options.render_line_ending()
        )
        .as_str();
        loop_input
    }
}
