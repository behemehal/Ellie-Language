use ellie_core::definite::items::function_parameter;

impl super::Transpiler for function_parameter::FunctionParameter {
    fn transpile(
        &self,
        _assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        _processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        true
    }
}
