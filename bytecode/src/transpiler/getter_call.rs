use super::type_resolver::resolve_type;
use crate::instructions;
use alloc::{vec, string::ToString};
use ellie_core::{
    definite::items::getter_call,
    defs::{self, DebugHeader, DebugHeaderType},
};

impl super::Transpiler for getter_call::GetterCall {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        let debug_header_start = assembler.location();
        let mut dependencies = vec![processed_page.hash];
        dependencies.extend(processed_page.dependencies.iter().map(|d| d.hash));
        resolve_type(
            assembler,
            &self.data,
            instructions::Registers::A,
            &hash,
            Some(dependencies),
        );
        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::Function,
            hash: 00009999999,
            module: processed_page.path.clone(),
            name: "@getter".to_string(),
            start_end: (debug_header_start, assembler.location()),
            pos: self.pos,
        });
        true
    }
}
