use ellie_core::definite::items::for_loop;

impl super::Transpiler for for_loop::ForLoop {
    fn transpile(
        &self,
        _assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        _processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        true
    }
}
