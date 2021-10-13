use crate::alloc::boxed::Box;
use crate::alloc::vec::Vec;
use crate::definite as crate_definite;
use crate::defs;
use ellie_core::definite::items;

#[repr(C)]
pub enum ConditionType {
    If,
    ElseIf,
    Else,
}

#[repr(C)]
pub struct ConditionChain {
    pub rtype: ConditionType,
    pub condition: Box<crate_definite::types::Types>,
    pub code: *mut crate_definite::items::Collecting,
    pub pos: defs::Cursor,
}

#[repr(C)]
pub struct Condition {
    pub chains: *mut ConditionChain,
    pub keyword_pos: defs::Cursor,
    pub cloak_pos: defs::Cursor,
}

pub unsafe fn build_condition_from(from: items::condition::Condition) -> Condition {
    Condition {
        chains: from
            .chains
            .into_iter()
            .map(|chain| ConditionChain {
                rtype: match chain.rtype {
                    items::condition::ConditionType::If => ConditionType::If,
                    items::condition::ConditionType::ElseIf => ConditionType::ElseIf,
                    items::condition::ConditionType::Else => ConditionType::Else,
                },
                condition: Box::new(crate_definite::types::build_collecting_from(
                    *chain.condition,
                )),
                code: chain
                    .code
                    .into_iter()
                    .map(|item| crate_definite::items::build_collecting_from(item))
                    .collect::<Vec<_>>()
                    .as_mut_ptr(),
                pos: defs::Cursor {
                    range_start: defs::CursorPosition(
                        chain.pos.range_start.0,
                        chain.pos.range_start.1,
                    ),
                    range_end: defs::CursorPosition(chain.pos.range_end.0, chain.pos.range_end.1),
                },
            })
            .collect::<Vec<_>>()
            .as_mut_ptr(),
        keyword_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.keyword_pos.range_start.0,
                from.keyword_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(
                from.keyword_pos.range_end.0,
                from.keyword_pos.range_end.1,
            ),
        },
        cloak_pos: defs::Cursor {
            range_start: defs::CursorPosition(
                from.cloak_pos.range_start.0,
                from.cloak_pos.range_start.1,
            ),
            range_end: defs::CursorPosition(from.cloak_pos.range_end.0, from.cloak_pos.range_end.1),
        },
    }
}
