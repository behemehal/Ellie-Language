use serde::{Deserialize, Serialize};
pub mod class;
pub mod condition;
pub mod constructor;
pub mod enum_type;
pub mod file_key;
pub mod for_loop;
pub mod function;
pub mod getter;
pub mod getter_call;
pub mod import;
pub mod setter;
pub mod setter_call;
pub mod variable;
pub mod generic;

pub mod native_function;
pub mod ret;

#[derive(PartialEq, Debug, Clone, Serialize, Deserialize)]
pub enum Collecting {
    Variable(variable::Variable),
    Function(function::Function),
    ForLoop(for_loop::ForLoop),
    Condition(condition::Condition),
    Class(class::Class),
    Ret(ret::Ret),
    Constructor(constructor::Constructor),
    Import(import::Import),
    FileKey(file_key::FileKey),
    Getter(getter::Getter),
    Setter(setter::Setter),
    Generic(generic::Generic),
    NativeClass,
    GetterCall(getter_call::GetterCall),
    SetterCall(setter_call::SetterCall),
    Enum(enum_type::EnumType),
    NativeFunction(native_function::NativeFunction),
    None,
}

impl Default for Collecting {
    fn default() -> Self {
        Collecting::None
    }
}

impl Collecting {
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
            Collecting::NativeClass => true,
            Collecting::GetterCall(_) => false,
            Collecting::SetterCall(_) => false,
            Collecting::Enum(e) => e.public,
            Collecting::NativeFunction(e) => e.public,
            Collecting::None => false,
            Collecting::Generic(_) => false,
        }
    }
}
