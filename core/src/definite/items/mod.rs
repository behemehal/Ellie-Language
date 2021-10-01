use serde::{Deserialize, Serialize};

pub mod caller;
pub mod class;
pub mod condition;
pub mod constructor;
pub mod enum_type;
pub mod file_key;
pub mod for_loop;
pub mod function;
pub mod getter;
pub mod import;
pub mod import_item;
pub mod setter;
pub mod variable;

pub mod native_function;
pub mod ret;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Collecting {
    ImportItem(import_item::ImportItem),
    Variable(variable::Variable),
    Function(function::Function),
    ForLoop(for_loop::ForLoop),
    Condition(condition::Condition),
    Class(class::Class),
    Ret(ret::Ret),
    Constructor(constructor::Constructor),
    Caller(caller::Caller),
    Import(import::Import),
    FileKey(file_key::FileKey),
    Getter(getter::Getter),
    Setter(setter::Setter),
    NativeClass,
    ValueCall(crate::definite::types::Types),
    Enum(enum_type::EnumType),
    NativeFunction(native_function::NativeFunction),
    None,
}

impl Collecting {
    pub fn is_pub(self) -> bool {
        match self {
            Collecting::ImportItem(e) => e.public,
            Collecting::Variable(e) => e.public,
            Collecting::Function(e) => e.public,
            Collecting::ForLoop(_) => false,
            Collecting::Condition(_) => false,
            Collecting::Class(e) => e.public,
            Collecting::Ret(_) => false,
            Collecting::Constructor(_) => false,
            Collecting::Caller(_) => false,
            Collecting::Import(e) => e.public,
            Collecting::FileKey(_) => false,
            Collecting::Getter(e) => e.public,
            Collecting::Setter(e) => e.public,
            Collecting::NativeClass => true,
            Collecting::ValueCall(_) => false,
            Collecting::Enum(e) => e.public,
            Collecting::NativeFunction(e) => e.public,
            Collecting::None => false,
        }
    }
}