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
                crate_definite::items::import_item::build_import_from(e),
            )
        }
        definite::items::Collecting::Variable(_) => todo!(),
        definite::items::Collecting::Function(_) => todo!(),
        definite::items::Collecting::ForLoop(_) => todo!(),
        definite::items::Collecting::Condition(_) => todo!(),
        definite::items::Collecting::Class(_) => todo!(),
        definite::items::Collecting::Ret(_) => todo!(),
        definite::items::Collecting::Constructor(_) => todo!(),
        definite::items::Collecting::Caller(_) => todo!(),
        definite::items::Collecting::Import(_) => todo!(),
        definite::items::Collecting::FileKey(_) => todo!(),
        definite::items::Collecting::Getter(_) => todo!(),
        definite::items::Collecting::Setter(_) => todo!(),
        definite::items::Collecting::NativeClass => todo!(),
        definite::items::Collecting::ValueCall(_) => todo!(),
        definite::items::Collecting::Enum(_) => todo!(),
        definite::items::Collecting::NativeFunction(_) => todo!(),
        definite::items::Collecting::None => todo!(),
    }
}
