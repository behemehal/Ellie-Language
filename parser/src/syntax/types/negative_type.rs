use crate::syntax::types;
use serde::Serialize;

use alloc::vec::Vec;
use alloc::boxed::Box;


/*

    true = true

    !true = false

*/

#[derive(PartialEq, Default, Debug, Clone, Serialize)]
pub struct Negative {
    pub value: Box<types::Types>
}