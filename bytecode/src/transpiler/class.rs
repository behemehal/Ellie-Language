use ellie_core::definite::items::class;

impl super::Transpiler for class::Class {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        for dependency in &processed_page.dependencies {
            assembler.assemble_dependency(&dependency.hash);
        }
        assembler.assemble_dependency(&self.inner_page_id);
        true
    }
}
