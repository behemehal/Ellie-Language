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
pub mod native_function;
pub mod ret;
pub mod setter;
pub mod variable;

use crate::definite as crate_definite;
use ellie_core::definite;

#[repr(C)]
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

pub unsafe fn build_collecting_from(target: definite::items::Collecting) -> Collecting {
    match target {
        definite::items::Collecting::ImportItem(e) => {
            crate_definite::items::Collecting::ImportItem(
                crate_definite::items::import_item::build_import_item_from(e),
            )
        }
        definite::items::Collecting::Variable(e) => {
            Collecting::Variable(variable::build_variable_from(e))
        }
        definite::items::Collecting::Function(e) => {
            Collecting::Function(function::build_function_from(e))
        }
        definite::items::Collecting::ForLoop(e) => {
            Collecting::ForLoop(for_loop::build_for_loop_from(e))
        }
        definite::items::Collecting::Condition(e) => {
            Collecting::Condition(condition::build_condition_from(e))
        }
        definite::items::Collecting::Class(e) => Collecting::Class(class::build_class_from(e)),
        definite::items::Collecting::Ret(e) => Collecting::Ret(ret::build_ret_from(e)),
        definite::items::Collecting::Constructor(e) => {
            Collecting::Constructor(constructor::build_constructor_from(e))
        }
        definite::items::Collecting::Caller(e) => Collecting::Caller(caller::build_caller_from(e)),
        definite::items::Collecting::Import(e) => Collecting::Import(import::build_import_from(e)),
        definite::items::Collecting::FileKey(e) => {
            Collecting::FileKey(file_key::build_file_key_from(e))
        }
        definite::items::Collecting::Getter(e) => Collecting::Getter(getter::build_getter_from(e)),
        definite::items::Collecting::Setter(e) => Collecting::Setter(setter::build_setter_from(e)),
        definite::items::Collecting::NativeClass => Collecting::NativeClass,
        definite::items::Collecting::ValueCall(e) => {
            Collecting::ValueCall(crate::definite::types::build_collecting_from(e))
        }
        definite::items::Collecting::Enum(e) => Collecting::Enum(enum_type::build_enum_from(e)),
        definite::items::Collecting::NativeFunction(e) => {
            Collecting::NativeFunction(native_function::build_native_function_from(e))
        }
        definite::items::Collecting::None => Collecting::None,
    }
}
