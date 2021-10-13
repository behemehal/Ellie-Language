use crate::definite::items as crate_definite;
use alloc::boxed::Box;
use ellie_core::definite::items;
use libc::c_char;

#[repr(C)]
pub struct ImportItem {
    pub from_path: *mut c_char,
    pub resolution_id: u64,
    pub from_import: u64,
    pub item: Box<crate_definite::Collecting>,
    pub public: bool,
}

pub unsafe fn build_import_item_from(from: items::import_item::ImportItem) -> ImportItem {
    ImportItem {
        from_path: from.from_path.as_ptr() as *mut i8,
        resolution_id: from.resolution_id,
        from_import: from.from_import,
        item: Box::new(crate_definite::build_collecting_from(*from.item)),
        public: from.public,
    }
}
