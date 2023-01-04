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
        //module_name>function_name
        let address = format!("{}>{}", self.module_name, self.name);

        //Skip to the end of the function
        assembler
            .instructions
            .push(instructions::Instructions::FN(Instruction::absolute(144)));
        let start = assembler.location();

        assembler.locals.push(LocalHeader {
            name: self.name.clone(),
            cursor: assembler.location(),
            page_hash: processed_page.hash,
            hash: Some(self.hash),
            reference: Instruction::absolute(assembler.location()),
        });

        //Reserve memory spaces for parameters
        /*
        for hash in 0..self.parameters.len() {
            assembler.instructions.push(Instructions::STA(Instruction::implicit()));

            assembler.instructions.push(Instructions::LDA(Instruction::function_parameter(hash)));
            assembler.instructions.push(Instructions::STA(Instruction::implicit()));

            assembler.locals.push(LocalHeader {
                name: self.parameters[hash].name.clone(),
                page_hash: processed_page.hash,
                cursor: assembler.location(),
                reference: Instruction::function_parameter(hash),
            });

        }
        */

        assembler
            .instructions
            .push(instructions::Instructions::CALLN(
                instructions::Instruction::immediate(
                    instructions::Types::String(address.len()),
                    address.as_bytes().to_vec(),
                ),
            ));

        let mut hash = self.hash.to_le_bytes().to_vec();
        hash.extend_from_slice(&(assembler.instructions.len()).to_le_bytes());

        assembler.instructions[start] = Instructions::FN(Instruction::immediate(
            instructions::Types::String(hash.len()),
            hash.to_vec(),
        ));

        assembler.instructions.push(instructions::Instructions::RET(
            instructions::Instruction::implicit(),
        ));
        true
    }
}
