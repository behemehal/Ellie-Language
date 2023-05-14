use super::super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::syntax::items::import::Import;

impl CodeRenderer for Import {
    //Renderer Options
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        let state_scope_length = state.render_scope_space(options);
        let is_public = if self.public { "pub " } else { "" };
        let mut import = format!("{state_scope_length}{is_public}import ");
        if self.link_module {
            import += format!("@{}", self.path.as_str()).as_str();
        } else if self.path_module {
            import += format!("\"{}\"", self.path.as_str()).as_str();
        }
        if self.reference != "" {
            import += " : ";
            import += self.reference.as_str();
        }
        import += ";";
        import
    }
}
