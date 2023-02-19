pub mod array;
pub mod as_keyword;
pub mod brace_reference;
pub mod byte;
pub mod char;
pub mod class_call;
pub mod cloak;
pub mod collective;
pub mod enum_data;
pub mod float;
pub mod function_call;
pub mod integer;
pub mod negative;
pub mod null_resolver;
pub mod operator;
pub mod reference;
pub mod string;
pub mod variable;

use super::{CodeRenderer, State};
use crate::fmt::FormatterOptions;
use ellie_tokenizer::processors::types::Processors;

impl CodeRenderer for Processors {
    fn render(&self, state: &State, options: &FormatterOptions) -> String {
        match self {
            Processors::Integer(e) => e.data.render(state, options),
            Processors::Byte(e) => e.render(state, options),
            Processors::Decimal(e) => e.data.render(state, options),
            Processors::Char(e) => e.render(state, options),
            Processors::String(e) => e.data.render(state, options),
            Processors::Variable(e) => e.data.render(state, options),
            Processors::Negative(e) => e.render(state, options),
            Processors::Array(e) => e.data.render(state, options),
            Processors::Operator(e) => e.data.render(state, options),
            Processors::Reference(e) => e.data.render(state, options),
            Processors::BraceReference(e) => e.data.render(state, options),
            Processors::EnumData(e) => e.data.render(state, options),
            Processors::NullResolver(e) => e.render(state, options),
            Processors::FunctionCall(e) => e.data.render(state, options),
            Processors::ClassCall(e) => e.data.render(state, options),
            Processors::Cloak(e) => e.data.render(state, options),
            Processors::Collective(e) => e.data.render(state, options),
            Processors::AsKeyword(e) => e.data.render(state, options),
        }
    }
}
