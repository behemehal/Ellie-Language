use super::type_resolver::resolve_type;
use crate::{instructions, utils::limit_platform_size};
use alloc::{string::ToString, vec};
use ellie_core::{
    definite::items::getter_call,
    defs::{DebugHeader, DebugHeaderType},
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
            rtype: DebugHeaderType::GetterCall,
            hash: limit_platform_size(00099999999, assembler.platform_attributes.architecture),
            module: processed_page.path.clone(),
            name: "@getter".to_string(),
            start_end: (debug_header_start, assembler.location()),
            pos: self.pos,
        });
        true
    }
}
