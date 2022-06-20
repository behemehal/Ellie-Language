use ellie_core::definite::items::constructor;

impl super::Transpiler for constructor::Constructor {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        std::println!("[Assembler,Ignore,Element] Constructor");
        true
    }
}
