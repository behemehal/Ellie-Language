use crate::fmt::FormatterOptions;

//Language Items
pub mod items;

//Language Types
pub mod defs;
pub mod types;

#[derive(Default, Clone, Debug)]
pub struct State {
    pub scope_length: usize,
    pub ending_token: String,
}

impl State {
    // Render state's scope length as spaces
    pub fn render_scope_space(&self, settings: &FormatterOptions) -> String {
        let mut scope_space = String::new();
        for _ in 0..self.scope_length {
            if settings.use_tabs {
                scope_space += "\t";
            } else {
                for _ in 0..settings.tab_size {
                    scope_space += " ";
                }
            }
        }
        scope_space
    }

    pub fn empty_state() -> State {
        State {
            scope_length: 0,
            ending_token: String::new(),
        }
    }
}

pub trait CodeRenderer {
    fn render(&self, state: &State, options: &FormatterOptions) -> String;
}
