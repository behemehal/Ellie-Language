use serde::{Deserialize, Serialize};

use crate::defs;
pub mod class;
pub mod condition;
pub mod constructor;
pub mod enum_type;
pub mod file_key;
pub mod for_loop;
pub mod function;
pub mod generic;
pub mod getter;
pub mod getter_call;
pub mod import;
pub mod native_function;
pub mod setter;
pub mod setter_call;
pub mod variable;
pub mod extend;

pub mod constructor_parameter;
pub mod function_parameter;
pub mod self_item;

pub mod brk;
pub mod go;
pub mod ret;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Collecting {
    Variable(variable::Variable),
    Function(function::Function),
    ForLoop(for_loop::ForLoop),
    Condition(condition::Condition),
    Class(class::Class),
    Extend(extend::Extend),
    Ret(ret::Ret),
    Brk(brk::Brk),
    Go(go::Go),
    Constructor(constructor::Constructor),
    Import(import::Import),
    FileKey(file_key::FileKey),
    Getter(getter::Getter),
    Setter(setter::Setter),
    Generic(generic::Generic),
    GetterCall(getter_call::GetterCall),
    SetterCall(setter_call::SetterCall),
    Enum(enum_type::EnumType),
    NativeFunction(native_function::NativeFunction),

    FuctionParameter(function_parameter::FunctionParameter),
    ConstructorParameter(constructor_parameter::ConstructorParameter),
    SelfItem(self_item::SelfItem),

    None,
}

impl Default for Collecting {
    fn default() -> Self {
        Collecting::None
    }
}

impl Collecting {
    pub fn get_pos(&self) -> defs::Cursor {
        match self {
            Collecting::Variable(e) => e.pos,
            Collecting::Function(e) => e.pos,
            Collecting::ForLoop(e) => e.pos,
            Collecting::Condition(e) => e.pos,
            Collecting::Class(e) => e.pos,
            Collecting::Ret(e) => e.pos,
            Collecting::Constructor(e) => e.pos,
            Collecting::Import(e) => e.pos,
            Collecting::FileKey(e) => e.pos,
            Collecting::Getter(e) => e.pos,
            Collecting::Setter(e) => e.pos,
            Collecting::Generic(e) => e.pos,
            Collecting::GetterCall(e) => e.pos,
            Collecting::SetterCall(e) => defs::Cursor {
                range_start: e.target_pos.range_start,
                range_end: e.value_pos.range_end,
            },
            Collecting::ConstructorParameter(e) => e.pos,
            Collecting::FuctionParameter(e) => defs::Cursor {
                range_start: e.name_pos.range_start,
                range_end: e.rtype_pos.range_end,
            },
            Collecting::Enum(e) => e.pos,
            Collecting::NativeFunction(e) => e.pos,
            Collecting::None => unreachable!(),
            Collecting::Brk(e) => e.pos,
            Collecting::Go(e) => e.pos,
            Collecting::SelfItem(_) => unreachable!(),
            Collecting::Extend(e) => e.pos,
        }
    }

    pub fn is_pub(self) -> bool {
        match self {
            Collecting::Variable(e) => e.public,
            Collecting::Function(e) => e.public,
            Collecting::ForLoop(_) => false,
            Collecting::Condition(_) => false,
            Collecting::Class(e) => e.public,
            Collecting::Ret(_) => false,
            Collecting::Constructor(_) => false,
            Collecting::Import(e) => e.public,
            Collecting::FileKey(_) => false,
            Collecting::Getter(e) => e.public,
            Collecting::Setter(e) => e.public,
            Collecting::GetterCall(_) => false,
            Collecting::SetterCall(_) => false,
            Collecting::Enum(e) => e.public,
            Collecting::NativeFunction(e) => e.public,
            Collecting::SelfItem(_) => true,
            Collecting::None => false,
            Collecting::Generic(_) => false,
            Collecting::Brk(_) => false,
            Collecting::Go(_) => false,
            Collecting::FuctionParameter(_) => false,
            Collecting::ConstructorParameter(_) => false,
            Collecting::Extend(_) => false,
        }
    }
}
