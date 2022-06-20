use ellie_core::definite::items::ret;

impl super::Transpiler for ret::Ret {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        std::println!("[Assembler,Ignore,Element] Ret");
        true
    }
}
