use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::comment::Comment;

impl CodeRenderer for Comment {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();
        let tab: &String = &options.render_tab();

        if self.line_comment {
            format!(
                "{state_scope_length}//{comment}{line_ending}",
                comment = self.content.last().unwrap()
            )
        } else {
            let mut comment = format!("{state_scope_length}/*{line_ending}");
            for line in &self.content {
                let line = line.trim();
                if line == "" {
                    continue;
                }
                comment += &format!("{state_scope_length}{tab}{line}{line_ending}");
            }
            comment += &format!("{state_scope_length}*/{line_ending}");
            comment
        }
    }
}
