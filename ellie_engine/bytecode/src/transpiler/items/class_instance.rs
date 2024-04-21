use alloc::{format, string::ToString};
use ellie_core::definite::types::class_instance;

use crate::{assembler::LocalHeader, instructions::Instruction};
//TODO: TO BE REMOVED
impl super::Transpiler for class_instance::ClassInstance {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        assembler.locals.push(LocalHeader {
            name: "self".to_string(),
            cursor: 0,
            page_hash: processed_page.hash,
            hash: Some(self.class_hash),
            reference: Instruction::absolute(self.class_hash),
            borrowed: None,
        });
        for item in &self.attributes {
            assembler.locals.push(LocalHeader {
                name: format!("self.{}", item.name),
                cursor: 0,
                page_hash: processed_page.hash,
                hash: Some(item.hash),
                reference: Instruction::absolute(item.hash),
                borrowed: None,
            });
        }
        true
    }
}
