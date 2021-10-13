use crate::alloc::boxed::Box;
use crate::definite::items;
use crate::definite::types;
use crate::defs;



#[repr(C)]
pub struct ForLoop {
    pub parameter: Box<types::Types>,
    pub parameter_pos: defs::Cursor,
    pub code: Vec<items::Collecting>,
    pub pos: defs::Cursor,
}
