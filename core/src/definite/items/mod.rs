use serde::{Deserialize, Serialize};

pub mod caller;
pub mod class;
pub mod condition;
pub mod constructor;
pub mod file_key;
pub mod for_loop;
pub mod function;
pub mod import;
pub mod import_item;
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
    Getter,
    Setter,
    NativeClass,
    NativeFunction(native_function::NativeFunction),
    None,
}
