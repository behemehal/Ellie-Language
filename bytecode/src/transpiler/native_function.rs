use crate::{
    assembler::LocalHeader,
    instructions::{self, Instruction, Instructions},
};
use alloc::format;
use ellie_core::definite::items::native_function;

impl super::Transpiler for native_function::NativeFunction {
    fn transpile(
        &self,
        assembler: &mut crate::assembler::Assembler,
        _hash: usize,
        processed_page: &ellie_parser::parser::ProcessedPage,
    ) -> bool {
        //Reserve native_function as CALLN instruction because there is no body.
        //module_name>function_name>function_parameter_len
        let address = format!(
            "{}>{}:{}",
            self.module_name,
            self.name,
            self.parameters.len()
        );
        //Reserve memory spaces for parameters
        for _ in &self.parameters {
            assembler
                .instructions
                .push(Instructions::STA(Instruction::implict()))
        }
        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.instructions.len(),
            page_hash: processed_page.hash,
            reference: None,
        });
        assembler
            .instructions
            .push(instructions::Instructions::CALLN(
                instructions::Instruction::immediate(
                    instructions::Types::String(address.len()),
                    address.as_bytes().to_vec(),
                ),
            ));
        true
    }
}
