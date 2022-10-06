use ellie_tokenizer::processors::items::Processors;

use super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;

//Return
pub mod brk;
pub mod class;
pub mod condition;
pub mod constructor;
pub mod enum_type;
pub mod file_key;
pub mod for_loop;
pub mod function;
pub mod getter;
pub mod getter_call;
pub mod go;
pub mod import;
pub mod loop_type;
pub mod ret;
pub mod self_item;
pub mod setter;
pub mod setter_call;
pub mod variable;

impl CodeRenderer for Processors {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        match self {
            Processors::Variable(e) => e.data.render(state, options),
            Processors::GetterCall(e) => e.render(state, options),
            Processors::SetterCall(e) => e.render(state, options),
            Processors::Function(e) => e.data.render(state, options),
            Processors::FileKey(e) => e.render(state, options),
            Processors::Import(e) => e.render(state, options),
            Processors::Loop(e) => e.render(state, options),
            Processors::ForLoop(e) => e.render(state, options),
            Processors::Condition(e) => e.render(state, options),
            Processors::Constructor(e) => e.render(state, options),
            Processors::Class(e) => e.render(state, options),
            Processors::Ret(e) => e.render(state, options),
            Processors::Brk(e) => e.render(state, options),
            Processors::Go(e) => e.render(state, options),
            Processors::Enum(e) => e.render(state, options),
            Processors::Getter(e) => e.render(state, options),
            Processors::Setter(e) => e.render(state, options),
            Processors::SelfItem(e) => e.render(state, options),
            _ => unreachable!("Rest of the items are not required to be rendered"),
        }
    }
}
