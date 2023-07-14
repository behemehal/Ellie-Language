use crate::{
    instruction_table, instructions::Instruction,
    utils::limit_platform_size,
};
use alloc::{string::ToString};
use ellie_core::{
    definite::items::self_item,
    defs::{DebugHeader, DebugHeaderType},
};

impl super::Transpiler for self_item::SelfItem {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        assembler
            .instructions
            .push(instruction_table::Instructions::STA(Instruction::implicit()));

        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Variable,
            hash: limit_platform_size(self.class_hash, assembler.platform_attributes.architecture),
            start_end: (assembler.location() - 1, assembler.location()),
            module: processed_page.path.clone(),
            name: "self".to_string(),
            pos: self.pos,
        });

        //assembler.locals.push(LocalHeader {
        //    name: "self".to_string(),
        //    cursor: assembler.location(),
        //    page_hash: processed_page.hash,
        //    hash: Some(self.class_hash),
        //    reference: Instruction::absolute(self.class_hash),
        //});
        //for item in &self.attributes {
        //    assembler.locals.push(LocalHeader {
        //        name: format!("self.{}", item.name),
        //        cursor: 0,
        //        page_hash: processed_page.hash,
        //        hash: Some(item.hash),
        //        reference: Instruction::absolute(item.hash),
        //    });
        //}
        true
    }
}
