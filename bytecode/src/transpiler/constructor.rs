use ellie_core::definite::items::constructor;

impl super::Transpiler for constructor::Constructor {
    fn transpile(
        &self,
        _assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        _processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        true
    }
}
