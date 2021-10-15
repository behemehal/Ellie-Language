use libc::c_char;

pub mod definers;
pub mod items;
pub mod types;

#[repr(C)]
pub struct CollectingItems {
    pub arr: *mut items::Collecting,
    pub size: usize,
}

#[repr(C)]
pub struct DefiniteParsed {
    pub name: *mut c_char,
    pub items: CollectingItems,
}

pub unsafe fn build_definite_parsed_from(
    from: ellie_core::definite::DefiniteParsed,
) -> DefiniteParsed {
    DefiniteParsed {
        name: from.name.as_ptr() as *mut i8,
        items: CollectingItems {
            arr: from
                .items
                .clone()
                .into_iter()
                .map(|x| items::build_collecting_from(x))
                .collect::<Vec<_>>()
                .as_mut_ptr(),
            size: from.items.len(),
        },
    }
}
