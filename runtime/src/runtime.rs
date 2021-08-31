//use ellie_parser::syntax::{
//    caller, class, condition, constructor, definers, file_key, for_loop, function, import,
//    import_item, native_function, ret, types, variable,
//};

use alloc::string::String;

pub enum RuntimeItem {
    ImportItem,
    Variable,
    Function,
    ForLoop,
    Condition,
    Class,
    Ret,
    Constructor,
    Caller,
    Import,
    FileKey,
    Getter,
    Setter,
    NativeClass,
    NativeFunction,
}

pub struct Runtime {
    pub start_point: String,
}

impl Runtime {
    pub fn new(main: String) -> Runtime {
        Runtime { start_point: main }
    }
}
