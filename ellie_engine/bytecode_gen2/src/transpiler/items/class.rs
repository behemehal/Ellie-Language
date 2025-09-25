use std::println;

use ellie_core::definite::items::class;

use crate::{assembler::LocalHeader, instructions::Instruction};

impl super::Transpiler for class::Class {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        println!("Skiping class transpile: {}", self.name);
        true
    }
}
