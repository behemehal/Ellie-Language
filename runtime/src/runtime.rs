//use ellie_parser::syntax::{
//    caller, class, condition, constructor, definers, file_key, for_loop, function, import,
//    import_item, native_function, ret, types, variable,
//};

use alloc::vec::Vec;
use crate::heap;
use crate::stack;
use crate::thread;

pub struct Runtime<F, E> {

    pub threads: Vec<thread::Thread<F, E>>
}

impl<F, E> Runtime<F, E> {}