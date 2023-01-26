use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::comment::Comment;

impl CodeRenderer for Comment {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let line_ending = &options.render_line_ending();

        if self.line_comment {
            format!("{state_scope_length}//{comment}{line_ending}", comment = self.content.last().unwrap())
        } else {
            todo!()
            //format!("{state_scope_length}/*{comment}*/{line_ending}", state_scope_length = state_scope_length, comment = self.comment, line_ending = line_ending)
        }
    }
}
