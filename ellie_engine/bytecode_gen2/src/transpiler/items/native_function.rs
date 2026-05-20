use crate::utils::limit_platform_size;
use ellie_core::defs::{DebugHeader, DebugHeaderType};

impl super::Transpiler for ellie_core::definite::items::native_function::NativeFunction {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        // Native functions have no generated code — just register a debug header
        // so the function call transpiler can identify them by hash as CALLN targets.
        assembler.debug_headers.push(DebugHeader {
            rtype: DebugHeaderType::NativeFunction,
            hash: limit_platform_size(self.hash, assembler.platform_attributes.architecture),
            start_end: (0, 0),
            module_name: self.module_name.clone(),
            module_hash: processed_page.hash,
            name: self.name.clone(),
            pos: self.pos,
        });
        true
    }
}
